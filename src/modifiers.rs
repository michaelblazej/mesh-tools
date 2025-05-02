//! Mesh modifier utilities
//!
//! This module provides functions to modify and transform meshes, including:
//! - Transformations (scaling, rotation, translation)
//! - Subdivision (smooth, catmull-clark)
//! - Simplification (reduction of triangles)
//! - Welding (combining vertices)
//! - Smoothing operations

use crate::{Mesh, Vertex, Triangle, Edge, MeshResult, MeshError};
use glam::{Vec2, Vec3, Vec4, Quat, Mat4};
use std::collections::{HashMap, HashSet};

/// Applies a transformation matrix to all vertices in the mesh
pub fn transform_mesh(mesh: &mut Mesh, transform: Mat4) {
    for vertex in &mut mesh.vertices {
        // Apply transformation to position
        let pos = transform.transform_point3(vertex.position);
        vertex.position = pos;
        
        // Transform normal if it exists (using the transposed inverse for correct normal transformation)
        if let Some(normal) = &mut vertex.normal {
            // Extract the 3x3 rotation/scale part and invert+transpose it
            let normal_transform = transform.inverse().transpose();
            *normal = (normal_transform.transform_vector3(*normal)).normalize();
        }
        
        // Transform tangent if it exists
        if let Some(tangent) = &mut vertex.tangent {
            let new_tangent = transform.transform_vector3(tangent.truncate()).extend(tangent.w);
            *tangent = new_tangent;
        }
    }
}

/// Applies a scale transformation to the mesh
pub fn scale_mesh(mesh: &mut Mesh, scale: Vec3) {
    let transform = Mat4::from_scale(scale);
    transform_mesh(mesh, transform);
}

/// Applies a rotation transformation to the mesh
pub fn rotate_mesh(mesh: &mut Mesh, rotation: Quat) {
    let transform = Mat4::from_quat(rotation);
    transform_mesh(mesh, transform);
}

/// Applies a translation transformation to the mesh
pub fn translate_mesh(mesh: &mut Mesh, translation: Vec3) {
    let transform = Mat4::from_translation(translation);
    transform_mesh(mesh, transform);
}

/// Flips the normals of all vertices in the mesh
pub fn flip_normals(mesh: &mut Mesh) {
    for vertex in &mut mesh.vertices {
        if let Some(normal) = &mut vertex.normal {
            *normal = -*normal;
        }
    }
    
    // Also flip the winding order of all triangles
    for triangle in &mut mesh.triangles {
        // Swap the second and third indices to flip the winding order
        let temp = triangle.indices[1];
        triangle.indices[1] = triangle.indices[2];
        triangle.indices[2] = temp;
    }
}

/// Vertex welding parameters
pub struct WeldParameters {
    /// The threshold distance for welding vertices
    pub threshold: f32,
    /// Whether to weld only vertices with matching normals
    pub respect_normals: bool,
    /// Whether to weld only vertices with matching UVs
    pub respect_uvs: bool,
}

impl Default for WeldParameters {
    fn default() -> Self {
        Self {
            threshold: 0.0001,
            respect_normals: false,
            respect_uvs: false,
        }
    }
}

/// Welds vertices that are within a certain distance of each other
pub fn weld_vertices(mesh: &mut Mesh, params: WeldParameters) -> MeshResult<usize> {
    // Create a mapping from old vertex indices to new ones
    let mut vertex_map = Vec::with_capacity(mesh.vertices.len());
    let mut unique_vertices: Vec<Vertex> = Vec::new();
    let mut welded_count = 0;
    
    for (old_idx, vertex) in mesh.vertices.iter().enumerate() {
        let mut found_match = false;
        
        for (new_idx, unique) in unique_vertices.iter().enumerate() {
            // Check position distance
            if (vertex.position - unique.position).length_squared() > params.threshold * params.threshold {
                continue;
            }
            
            // Check normals if required
            if params.respect_normals {
                match (&vertex.normal, &unique.normal) {
                    (Some(n1), Some(n2)) => {
                        if (*n1 - *n2).length_squared() > params.threshold {
                            continue;
                        }
                    },
                    (None, None) => {},
                    _ => continue, // One has normal, one doesn't
                }
            }
            
            // Check UVs if required
            if params.respect_uvs {
                match (&vertex.uv, &unique.uv) {
                    (Some(uv1), Some(uv2)) => {
                        if (*uv1 - *uv2).length_squared() > params.threshold {
                            continue;
                        }
                    },
                    (None, None) => {},
                    _ => continue, // One has UV, one doesn't
                }
            }
            
            // If we get here, we found a match
            vertex_map.push(new_idx);
            found_match = true;
            welded_count += 1;
            break;
        }
        
        if !found_match {
            // No match found, add as a new unique vertex
            vertex_map.push(unique_vertices.len());
            unique_vertices.push(vertex.clone());
        }
    }
    
    // Update triangles to reference new vertex indices
    for triangle in &mut mesh.triangles {
        for i in 0..3 {
            if triangle.indices[i] >= vertex_map.len() {
                return Err(MeshError::InvalidIndex(triangle.indices[i]));
            }
            triangle.indices[i] = vertex_map[triangle.indices[i]];
        }
    }
    
    // Update polygons if they exist
    if let Some(ref mut polygons) = mesh.polygons {
        for polygon in polygons {
            for i in 0..polygon.indices.len() {
                if polygon.indices[i] >= vertex_map.len() {
                    return Err(MeshError::InvalidIndex(polygon.indices[i]));
                }
                polygon.indices[i] = vertex_map[polygon.indices[i]];
            }
        }
    }
    
    // Replace the vertices with the unique ones
    mesh.vertices = unique_vertices;
    
    // Return the number of vertices removed
    Ok(welded_count)
}

/// Removes unused vertices (not referenced by any triangle)
pub fn remove_unused_vertices(mesh: &mut Mesh) -> usize {
    // Find all used vertices
    let mut used_vertices = HashSet::new();
    for triangle in &mesh.triangles {
        for &idx in &triangle.indices {
            used_vertices.insert(idx);
        }
    }
    
    // If all vertices are used, no need to do anything
    if used_vertices.len() == mesh.vertices.len() {
        return 0;
    }
    
    // Create new vertex list and mapping
    let mut new_vertices = Vec::with_capacity(used_vertices.len());
    let mut vertex_map = vec![0; mesh.vertices.len()];
    
    for &old_idx in used_vertices.iter() {
        vertex_map[old_idx] = new_vertices.len();
        new_vertices.push(mesh.vertices[old_idx].clone());
    }
    
    // Update triangle indices
    for triangle in &mut mesh.triangles {
        for i in 0..3 {
            triangle.indices[i] = vertex_map[triangle.indices[i]];
        }
    }
    
    // Update polygons if they exist
    if let Some(ref mut polygons) = mesh.polygons {
        for polygon in polygons {
            for i in 0..polygon.indices.len() {
                polygon.indices[i] = vertex_map[polygon.indices[i]];
            }
        }
    }
    
    let removed_count = mesh.vertices.len() - new_vertices.len();
    mesh.vertices = new_vertices;
    
    removed_count
}

/// Remove degenerate triangles (triangles with duplicate vertices or zero area)
pub fn remove_degenerate_triangles(mesh: &mut Mesh) -> usize {
    let original_count = mesh.triangles.len();
    
    // Filter out degenerate triangles
    mesh.triangles.retain(|triangle| {
        let [a, b, c] = triangle.indices;
        
        // Check for duplicate vertices
        if a == b || b == c || c == a {
            return false;
        }
        
        // Check for zero area (colinear vertices)
        if mesh.vertices.len() > std::cmp::max(a, std::cmp::max(b, c)) {
            let va = mesh.vertices[a].position;
            let vb = mesh.vertices[b].position;
            let vc = mesh.vertices[c].position;
            
            let edge1 = vb - va;
            let edge2 = vc - va;
            
            // Cross product to check for colinearity
            let cross = edge1.cross(edge2);
            
            // If the cross product is close to zero, the triangle is degenerate
            if cross.length_squared() < 1e-10 {
                return false;
            }
        }
        
        true
    });
    
    original_count - mesh.triangles.len()
}

/// Generates smooth vertex normals for a mesh by averaging face normals
pub fn generate_smooth_normals(mesh: &mut Mesh) {
    // First ensure we have an edge structure to determine which triangles are connected
    let mut vertex_triangles: Vec<Vec<usize>> = vec![Vec::new(); mesh.vertices.len()];
    
    // For each triangle, record which vertices it uses
    for (triangle_idx, triangle) in mesh.triangles.iter().enumerate() {
        for &vertex_idx in &triangle.indices {
            vertex_triangles[vertex_idx].push(triangle_idx);
        }
    }
    
    // Compute face normals for each triangle
    let mut face_normals = Vec::with_capacity(mesh.triangles.len());
    for triangle in &mesh.triangles {
        let [a, b, c] = triangle.indices;
        
        if a >= mesh.vertices.len() || b >= mesh.vertices.len() || c >= mesh.vertices.len() {
            // Invalid triangle, skip it
            face_normals.push(Vec3::Z);
            continue;
        }
        
        let va = mesh.vertices[a].position;
        let vb = mesh.vertices[b].position;
        let vc = mesh.vertices[c].position;
        
        let edge1 = vb - va;
        let edge2 = vc - va;
        let normal = edge1.cross(edge2).normalize_or_zero();
        face_normals.push(normal);
    }
    
    // Compute vertex normals by averaging face normals of adjacent triangles
    for (vertex_idx, triangles) in vertex_triangles.iter().enumerate() {
        if triangles.is_empty() {
            continue;
        }
        
        let mut normal = Vec3::ZERO;
        for &triangle_idx in triangles {
            normal += face_normals[triangle_idx];
        }
        
        let normalized = normal.normalize_or_zero();
        if normalized != Vec3::ZERO {
            mesh.vertices[vertex_idx].normal = Some(normalized);
        }
    }
    
    mesh.has_normals = true;
}

/// Subdivides each triangle into 4 triangles
pub fn subdivide_mesh(mesh: &mut Mesh) -> MeshResult<()> {
    // Edge midpoint cache to avoid creating duplicate vertices
    let mut edge_midpoints: HashMap<(usize, usize), usize> = HashMap::new();
    
    // Clone the original triangles because we'll be modifying the mesh
    let original_triangles = mesh.triangles.clone();
    mesh.triangles.clear();
    
    // For each original triangle
    for triangle in &original_triangles {
        let [a, b, c] = triangle.indices;
        
        // Get or create midpoints for each edge
        let midpoint_ab = get_or_create_midpoint(mesh, a, b, &mut edge_midpoints)?;
        let midpoint_bc = get_or_create_midpoint(mesh, b, c, &mut edge_midpoints)?;
        let midpoint_ca = get_or_create_midpoint(mesh, c, a, &mut edge_midpoints)?;
        
        // Create 4 new triangles
        let _ = mesh.add_triangle(a, midpoint_ab, midpoint_ca)?;
        let _ = mesh.add_triangle(midpoint_ab, b, midpoint_bc)?;
        let _ = mesh.add_triangle(midpoint_ca, midpoint_bc, c)?;
        let _ = mesh.add_triangle(midpoint_ab, midpoint_bc, midpoint_ca)?;
    }
    
    Ok(())
}

// Helper function for subdivision to find or create midpoints
fn get_or_create_midpoint(
    mesh: &mut Mesh,
    a: usize,
    b: usize,
    cache: &mut HashMap<(usize, usize), usize>
) -> MeshResult<usize> {
    // Ensure vertex indices are valid
    if a >= mesh.vertices.len() || b >= mesh.vertices.len() {
        return Err(MeshError::InvalidIndex(std::cmp::max(a, b)));
    }
    
    // Use consistent ordering to ensure we get the same midpoint for (a,b) and (b,a)
    let key = if a < b { (a, b) } else { (b, a) };
    
    if let Some(&idx) = cache.get(&key) {
        return Ok(idx);
    }
    
    // Create a new midpoint vertex
    let va = &mesh.vertices[a];
    let vb = &mesh.vertices[b];
    
    let position = (va.position + vb.position) * 0.5;
    
    // Interpolate attributes
    let normal = match (&va.normal, &vb.normal) {
        (Some(na), Some(nb)) => Some((*na + *nb).normalize()),
        _ => None
    };
    
    let uv = match (&va.uv, &vb.uv) {
        (Some(uva), Some(uvb)) => Some((*uva + *uvb) * 0.5),
        _ => None
    };
    
    let tangent = match (&va.tangent, &vb.tangent) {
        (Some(ta), Some(tb)) => {
            // Handle the w component specially (it's a sign)
            let w = if ta.w == tb.w { ta.w } else { 1.0 };
            Some(Vec4::new(
                (ta.x + tb.x) * 0.5,
                (ta.y + tb.y) * 0.5,
                (ta.z + tb.z) * 0.5,
                w
            ).normalize())
        },
        _ => None
    };
    
    let color = match (&va.color, &vb.color) {
        (Some(ca), Some(cb)) => Some((*ca + *cb) * 0.5),
        _ => None
    };
    
    // Create the midpoint vertex with interpolated attributes
    let mut midpoint = Vertex::new(position);
    if let Some(n) = normal {
        midpoint.normal = Some(n);
    }
    if let Some(u) = uv {
        midpoint.uv = Some(u);
    }
    if let Some(t) = tangent {
        midpoint.tangent = Some(t);
    }
    if let Some(c) = color {
        midpoint.color = Some(c);
    }
    
    let idx = mesh.add_vertex(midpoint);
    cache.insert(key, idx);
    
    Ok(idx)
}

/// Extrudes faces along their normals
pub fn extrude_faces(mesh: &mut Mesh, faces: &[usize], amount: f32) -> MeshResult<()> {
    if !mesh.has_normals() {
        // If we don't have normals, we can't extrude
        generate_smooth_normals(mesh);
    }
    
    // Gather all edges and vertices associated with the selected faces
    let mut face_vertices = HashSet::new();
    let mut face_edges = HashSet::new();
    
    for &face_idx in faces {
        if face_idx >= mesh.triangles.len() {
            return Err(MeshError::InvalidIndex(face_idx));
        }
        
        let triangle = mesh.triangles[face_idx];
        let [a, b, c] = triangle.indices;
        
        face_vertices.insert(a);
        face_vertices.insert(b);
        face_vertices.insert(c);
        
        face_edges.insert(Edge::new(a, b));
        face_edges.insert(Edge::new(b, c));
        face_edges.insert(Edge::new(c, a));
    }
    
    // Create new vertices by extruding existing ones
    let mut vertex_map = HashMap::new();
    for &orig_idx in &face_vertices {
        let orig_vertex = &mesh.vertices[orig_idx];
        let normal = orig_vertex.normal.unwrap_or(Vec3::Z);
        
        // Create a new vertex offset along the normal
        let mut new_vertex = orig_vertex.clone();
        new_vertex.position += normal * amount;
        
        let new_idx = mesh.add_vertex(new_vertex);
        vertex_map.insert(orig_idx, new_idx);
    }
    
    // Create triangles for the side faces
    for edge in &face_edges {
        let [a, b] = edge.vertices;
        
        if let (Some(&a_new), Some(&b_new)) = (vertex_map.get(&a), vertex_map.get(&b)) {
            // Create two triangles for a quad face connecting original and extruded vertices
            let _ = mesh.add_triangle(a, b, b_new)?;
            let _ = mesh.add_triangle(a, b_new, a_new)?;
        }
    }
    
    // Create extruded face triangles
    for &face_idx in faces {
        let triangle = mesh.triangles[face_idx];
        let [a, b, c] = triangle.indices;
        
        if let (Some(&a_new), Some(&b_new), Some(&c_new)) = 
            (vertex_map.get(&a), vertex_map.get(&b), vertex_map.get(&c)) {
            // Create the extruded face with reversed winding to ensure outward normal
            let _ = mesh.add_triangle(a_new, c_new, b_new)?;
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives;
    
    #[test]
    fn test_transform_mesh() {
        // Create a cube
        let mut cube = primitives::create_cube(1.0, 1.0, 1.0);
        let original_vertex_count = cube.vertices.len();
        
        // Scale it
        scale_mesh(&mut cube, Vec3::new(2.0, 3.0, 4.0));
        
        // Check that the vertex count hasn't changed
        assert_eq!(cube.vertices.len(), original_vertex_count);
        
        // Check that a vertex position has been properly scaled
        let max_vertex = cube.vertices.iter()
            .fold(Vec3::ZERO, |max, v| Vec3::new(
                max.x.max(v.position.x.abs()),
                max.y.max(v.position.y.abs()),
                max.z.max(v.position.z.abs())
            ));
            
        assert!((max_vertex.x - 1.0).abs() < 0.001);
        assert!((max_vertex.y - 1.5).abs() < 0.001);
        assert!((max_vertex.z - 2.0).abs() < 0.001);
    }
    
    #[test]
    fn test_flip_normals() {
        // Create a sphere
        let mut sphere = primitives::create_sphere(1.0, 8, 4);
        
        // Get a sample normal
        let original_normal = sphere.vertices[0].normal.unwrap();
        
        // Flip normals
        flip_normals(&mut sphere);
        
        // Check that the normal has been inverted
        let flipped_normal = sphere.vertices[0].normal.unwrap();
        assert!((original_normal + flipped_normal).length() < 0.001);
    }
    
    #[test]
    fn test_weld_vertices() {
        // Create a simple mesh with duplicate vertices
        let mut mesh = Mesh::new();
        
        // Add some vertices, including duplicates
        let v0 = mesh.add_vertex(Vertex::new(Vec3::new(0.0, 0.0, 0.0)));
        let v1 = mesh.add_vertex(Vertex::new(Vec3::new(1.0, 0.0, 0.0)));
        let v2 = mesh.add_vertex(Vertex::new(Vec3::new(0.0, 1.0, 0.0)));
        // Duplicate of v0, slightly offset
        let v3 = mesh.add_vertex(Vertex::new(Vec3::new(0.001, 0.0, 0.0)));
        
        // Add a triangle
        let _ = mesh.add_triangle(v0, v1, v2);
        let _ = mesh.add_triangle(v3, v1, v2);
        
        // Weld vertices
        let params = WeldParameters {
            threshold: 0.01,
            ..Default::default()
        };
        
        let welded_count = weld_vertices(&mut mesh, params).unwrap();
        
        // Should have welded one vertex
        assert_eq!(welded_count, 1);
        assert_eq!(mesh.vertices.len(), 3);
    }
    
    #[test]
    fn test_subdivide_mesh() {
        // Create a simple mesh with one triangle
        let mut mesh = Mesh::new();
        
        // Add vertices
        let v0 = mesh.add_vertex(Vertex::new(Vec3::new(0.0, 0.0, 0.0)));
        let v1 = mesh.add_vertex(Vertex::new(Vec3::new(1.0, 0.0, 0.0)));
        let v2 = mesh.add_vertex(Vertex::new(Vec3::new(0.0, 1.0, 0.0)));
        
        // Add a triangle
        let _ = mesh.add_triangle(v0, v1, v2);
        
        // Subdivide the mesh
        let _ = subdivide_mesh(&mut mesh);
        
        // Should now have 6 vertices (original 3 + 3 midpoints)
        assert_eq!(mesh.vertices.len(), 6);
        
        // Should have 4 triangles after subdivision
        assert_eq!(mesh.triangles.len(), 4);
    }
}
