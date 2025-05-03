
use crate::{Mesh, Vertex, Triangle, Edge, MeshResult, MeshError};
use glam::{Vec2, Vec3, Vec4, Quat, Mat4};
use std::collections::{HashMap, HashSet};

pub fn transform_mesh(mesh: &mut Mesh, transform: Mat4) {
    for vertex in &mut mesh.vertices {
        
        let pos = transform.transform_point3(vertex.position);
        vertex.position = pos;
        
        
        if let Some(normal) = &mut vertex.normal {
            
            let normal_transform = transform.inverse().transpose();
            *normal = (normal_transform.transform_vector3(*normal)).normalize();
        }
        
        
        if let Some(tangent) = &mut vertex.tangent {
            let new_tangent = transform.transform_vector3(tangent.truncate()).extend(tangent.w);
            *tangent = new_tangent;
        }
    }
}

pub fn scale_mesh(mesh: &mut Mesh, scale: Vec3) {
    let transform = Mat4::from_scale(scale);
    transform_mesh(mesh, transform);
}

pub fn rotate_mesh(mesh: &mut Mesh, rotation: Quat) {
    let transform = Mat4::from_quat(rotation);
    transform_mesh(mesh, transform);
}

pub fn translate_mesh(mesh: &mut Mesh, translation: Vec3) {
    let transform = Mat4::from_translation(translation);
    transform_mesh(mesh, transform);
}

pub fn flip_normals(mesh: &mut Mesh) {
    
    for vertex in &mut mesh.vertices {
        if let Some(ref mut normal) = vertex.normal {
            *normal = -*normal;
        }
    }
    
    
    for triangle in &mut mesh.triangles {
        let temp = triangle.indices[0];
        triangle.indices[0] = triangle.indices[2];
        triangle.indices[2] = temp;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct WeldParameters {
    pub distance_threshold: f32,
    pub check_normals: bool,
    pub normal_threshold: f32,
    pub check_uvs: bool,
    pub uv_threshold: f32,
}

impl Default for WeldParameters {
    fn default() -> Self {
        Self {
            distance_threshold: 0.0001,
            check_normals: true,
            normal_threshold: 0.01,
            check_uvs: true,
            uv_threshold: 0.01,
        }
    }
}

pub fn weld_vertices(mesh: &mut Mesh, params: WeldParameters) -> MeshResult<usize> {
    
    let mut vertex_map = HashMap::new();
    let mut new_vertices = Vec::<Vertex>::new();
    
    
    for (old_idx, vertex) in mesh.vertices.iter().enumerate() {
        
        let mut found_match = false;
        
        for (new_idx, new_vertex) in new_vertices.iter().enumerate() {
            
            let dist = (vertex.position - new_vertex.position).length_squared();
            if dist > params.distance_threshold * params.distance_threshold {
                continue;
            }
            
            
            if params.check_normals && vertex.normal.is_some() && new_vertex.normal.is_some() {
                let normal_dot = vertex.normal.unwrap().dot(new_vertex.normal.unwrap());
                if normal_dot < 1.0 - params.normal_threshold {
                    continue;
                }
            }
            
            
            if params.check_uvs && vertex.uv.is_some() && new_vertex.uv.is_some() {
                let uv_dist = (vertex.uv.unwrap() - new_vertex.uv.unwrap()).length_squared();
                if uv_dist > params.uv_threshold * params.uv_threshold {
                    continue;
                }
            }
            
            
            vertex_map.insert(old_idx, new_idx);
            found_match = true;
            break;
        }
        
        
        if !found_match {
            vertex_map.insert(old_idx, new_vertices.len());
            new_vertices.push(vertex.clone());
        }
    }
    
    
    let welded_count = mesh.vertices.len() - new_vertices.len();
    
    
    for triangle in &mut mesh.triangles {
        for i in 0..3 {
            if let Some(&new_idx) = vertex_map.get(&triangle.indices[i]) {
                triangle.indices[i] = new_idx;
            } else {
                return Err(MeshError::InvalidIndex(triangle.indices[i]));
            }
        }
    }
    
    
    mesh.vertices = new_vertices;
    
    Ok(welded_count)
}

pub fn remove_unused_vertices(mesh: &mut Mesh) -> usize {
    
    let mut used_vertices = vec![false; mesh.vertices.len()];
    
    for triangle in &mesh.triangles {
        for &idx in &triangle.indices {
            if idx < used_vertices.len() {
                used_vertices[idx] = true;
            }
        }
    }
    
    
    let unused_count = used_vertices.iter().filter(|&&used| !used).count();
    
    if unused_count == 0 {
        return 0; 
    }
    
    
    let mut index_map = vec![0; mesh.vertices.len()];
    let mut new_index = 0;
    
    for (old_idx, &used) in used_vertices.iter().enumerate() {
        if used {
            index_map[old_idx] = new_index;
            new_index += 1;
        }
    }
    
    
    let mut new_vertices = Vec::with_capacity(mesh.vertices.len() - unused_count);
    
    for (idx, vertex) in mesh.vertices.iter().enumerate() {
        if used_vertices[idx] {
            new_vertices.push(vertex.clone());
        }
    }
    
    
    for triangle in &mut mesh.triangles {
        for i in 0..3 {
            triangle.indices[i] = index_map[triangle.indices[i]];
        }
    }
    
    
    mesh.vertices = new_vertices;
    
    unused_count
}

pub fn remove_degenerate_triangles(mesh: &mut Mesh) -> usize {
    let initial_count = mesh.triangles.len();
    
    
    mesh.triangles.retain(|triangle| {
        
        let [a, b, c] = triangle.indices;
        if a == b || b == c || a == c {
            return false;
        }
        
        
        let va = mesh.vertices[a].position;
        let vb = mesh.vertices[b].position;
        let vc = mesh.vertices[c].position;
        
        let edge1 = vb - va;
        let edge2 = vc - va;
        let cross = edge1.cross(edge2);
        
        cross.length_squared() > 1e-10 
    });
    
    initial_count - mesh.triangles.len()
}

pub fn generate_smooth_normals(mesh: &mut Mesh) {
    
    let mut normal_sums = vec![Vec3::ZERO; mesh.vertices.len()];
    
    
    for triangle in &mesh.triangles {
        let [a, b, c] = triangle.indices;
        
        
        let va = mesh.vertices[a].position;
        let vb = mesh.vertices[b].position;
        let vc = mesh.vertices[c].position;
        
        
        let edge1 = vb - va;
        let edge2 = vc - va;
        let normal = edge1.cross(edge2).normalize();
        
        
        normal_sums[a] += normal;
        normal_sums[b] += normal;
        normal_sums[c] += normal;
    }
    
    
    for (i, normal_sum) in normal_sums.iter().enumerate() {
        if normal_sum.length_squared() > 0.0 {
            mesh.vertices[i].normal = Some(normal_sum.normalize());
        } else {
            
            mesh.vertices[i].normal = Some(Vec3::Y);
        }
    }
}

pub fn subdivide_mesh(mesh: &mut Mesh) -> MeshResult<()> {
    
    let mut edge_midpoints = HashMap::new();
    
    
    let original_triangles = mesh.triangles.clone();
    
    
    mesh.triangles.clear();
    
    
    for triangle in &original_triangles {
        let [a, b, c] = triangle.indices;
        
        
        let ab = get_or_create_midpoint(mesh, a, b, &mut edge_midpoints)?;
        let bc = get_or_create_midpoint(mesh, b, c, &mut edge_midpoints)?;
        let ca = get_or_create_midpoint(mesh, c, a, &mut edge_midpoints)?;
        
        
        mesh.add_triangle(a, ab, ca)?;     
        mesh.add_triangle(ab, b, bc)?;     
        mesh.add_triangle(ca, bc, c)?;     
        mesh.add_triangle(ab, bc, ca)?;    
    }
    
    Ok(())
}

fn get_or_create_midpoint(
    mesh: &mut Mesh,
    a: usize,
    b: usize,
    cache: &mut HashMap<(usize, usize), usize>
) -> MeshResult<usize> {
    
    let (min_idx, max_idx) = if a < b { (a, b) } else { (b, a) };
    let key = (min_idx, max_idx);
    
    
    if let Some(&midpoint_idx) = cache.get(&key) {
        return Ok(midpoint_idx);
    }
    
    
    if min_idx >= mesh.vertices.len() || max_idx >= mesh.vertices.len() {
        return Err(MeshError::InvalidIndex(std::cmp::max(min_idx, max_idx)));
    }
    
    let v1 = &mesh.vertices[min_idx];
    let v2 = &mesh.vertices[max_idx];
    
    
    let pos = (v1.position + v2.position) * 0.5;
    
    
    let normal = match (v1.normal, v2.normal) {
        (Some(n1), Some(n2)) => Some((n1 + n2).normalize()),
        _ => None,
    };
    
    
    let uv = match (v1.uv, v2.uv) {
        (Some(uv1), Some(uv2)) => Some((uv1 + uv2) * 0.5),
        _ => None,
    };
    
    
    let mut vertex = Vertex::new(pos);
    
    if let Some(n) = normal {
        vertex.normal = Some(n);
    }
    
    if let Some(u) = uv {
        vertex.uv = Some(u);
    }
    
    
    let midpoint_idx = mesh.add_vertex(vertex);
    
    
    cache.insert(key, midpoint_idx);
    
    Ok(midpoint_idx)
}

pub fn extrude_faces(mesh: &mut Mesh, faces: &[usize], amount: f32) -> MeshResult<()> {
    if faces.is_empty() {
        return Ok(());
    }
    
    
    let mut vertices_to_extrude = HashSet::new();
    let mut face_normals = Vec::with_capacity(faces.len());
    
    for &face_idx in faces {
        if face_idx >= mesh.triangles.len() {
            return Err(MeshError::InvalidIndex(face_idx));
        }
        
        let triangle = mesh.triangles[face_idx];
        let [a, b, c] = triangle.indices;
        
        vertices_to_extrude.insert(a);
        vertices_to_extrude.insert(b);
        vertices_to_extrude.insert(c);
        
        
        let va = mesh.vertices[a].position;
        let vb = mesh.vertices[b].position;
        let vc = mesh.vertices[c].position;
        
        let normal = (vb - va).cross(vc - va).normalize();
        face_normals.push((face_idx, normal));
    }
    
    
    let mut vertex_map = HashMap::new();
    
    for &old_idx in &vertices_to_extrude {
        let old_vertex = &mesh.vertices[old_idx];
        let mut new_vertex = old_vertex.clone();
        
        
        let mut avg_normal = Vec3::ZERO;
        let mut count = 0;
        
        for &(face_idx, normal) in &face_normals {
            let triangle = mesh.triangles[face_idx];
            if triangle.indices.contains(&old_idx) {
                avg_normal += normal;
                count += 1;
            }
        }
        
        if count > 0 {
            avg_normal /= count as f32;
            avg_normal = avg_normal.normalize();
            
            
            new_vertex.position += avg_normal * amount;
            
            
            let new_idx = mesh.add_vertex(new_vertex);
            vertex_map.insert(old_idx, new_idx);
        }
    }
    
    
    for &face_idx in faces {
        let triangle = mesh.triangles[face_idx];
        let [a, b, c] = triangle.indices;
        
        
        let a_new = *vertex_map.get(&a).ok_or(MeshError::InvalidIndex(a))?;
        let b_new = *vertex_map.get(&b).ok_or(MeshError::InvalidIndex(b))?;
        let c_new = *vertex_map.get(&c).ok_or(MeshError::InvalidIndex(c))?;
        
        
        mesh.add_triangle(a_new, c_new, b_new)?; 
        
        
        mesh.add_triangle(a, b, b_new)?;
        mesh.add_triangle(a, b_new, a_new)?;
        
        mesh.add_triangle(b, c, c_new)?;
        mesh.add_triangle(b, c_new, b_new)?;
        
        mesh.add_triangle(c, a, a_new)?;
        mesh.add_triangle(c, a_new, c_new)?;
    }
    
    Ok(())
}
