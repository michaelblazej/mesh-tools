//! # Mesh Modifier Utilities
//!
//! This module provides functions to modify and transform meshes, including:
//! 
//! - **Transformations**: Scale, rotate, and translate meshes in 3D space
//! - **Subdivision**: Break down triangles into smaller ones for smoother surfaces
//! - **Simplification**: Reduce triangle count while preserving shape
//! - **Welding**: Combine nearby vertices to clean up meshes
//! - **Smoothing**: Generate smooth normals and improve surface appearance
//!
//! ## Examples
//!
//! ```rust
//! use mesh_tools::{Mesh, primitives::create_cube};
//! use mesh_tools::modifiers::{translate_mesh, rotate_mesh, scale_mesh, generate_smooth_normals};
//! use glam::{Vec3, Quat};
//!
//! // Create a cube and transform it
//! let mut cube = create_cube(1.0, 1.0, 1.0);
//!
//! // Scale it non-uniformly
//! scale_mesh(&mut cube, Vec3::new(2.0, 1.0, 0.5));
//!
//! // Rotate it 45 degrees around Y axis
//! rotate_mesh(&mut cube, Quat::from_rotation_y(std::f32::consts::FRAC_PI_4));
//!
//! // Move it up 3 units
//! translate_mesh(&mut cube, Vec3::new(0.0, 3.0, 0.0));
//!
//! // Generate smooth normals
//! generate_smooth_normals(&mut cube);
//! ```

use crate::{Mesh, Vertex, Triangle, Edge, MeshResult, MeshError};
use glam::{Vec2, Vec3, Vec4, Quat, Mat4};
use std::collections::{HashMap, HashSet};

/// Applies a transformation matrix to all vertices in the mesh
///
/// This is a general-purpose transformation function that can handle any
/// combination of translation, rotation, and scaling through a single matrix.
/// It correctly transforms positions, normals, and tangents.
///
/// # Arguments
///
/// * `mesh` - The mesh to transform
/// * `transform` - The 4x4 transformation matrix to apply
///
/// # Examples
///
/// ```
/// use mesh_tools::{Mesh, primitives::create_cube, modifiers::transform_mesh};
/// use glam::Mat4;
///
/// let mut cube = create_cube(1.0, 1.0, 1.0);
///
/// // Create a transformation matrix (rotation + translation)
/// let transform = Mat4::from_rotation_y(0.5) * Mat4::from_translation(glam::Vec3::new(0.0, 1.0, 0.0));
///
/// // Apply the transformation
/// transform_mesh(&mut cube, transform);
/// ```
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
///
/// Scales the mesh uniformly or non-uniformly along the X, Y, and Z axes.
/// This correctly adjusts positions, normals, and tangents.
///
/// # Arguments
///
/// * `mesh` - The mesh to scale
/// * `scale` - The scale factors for X, Y, and Z axes
///
/// # Examples
///
/// ```
/// use mesh_tools::{Mesh, primitives::create_cube, modifiers::scale_mesh};
/// use glam::Vec3;
///
/// let mut cube = create_cube(1.0, 1.0, 1.0);
///
/// // Scale uniformly (2x size)
/// scale_mesh(&mut cube, Vec3::splat(2.0));
///
/// // Or scale non-uniformly to create a stretched object
/// // scale_mesh(&mut cube, Vec3::new(1.0, 2.0, 0.5));
/// ```
pub fn scale_mesh(mesh: &mut Mesh, scale: Vec3) {
    let transform = Mat4::from_scale(scale);
    transform_mesh(mesh, transform);
}

/// Applies a rotation transformation to the mesh
///
/// Rotates the mesh using a quaternion, which can represent any rotation in 3D space.
/// This correctly adjusts positions, normals, and tangents.
///
/// # Arguments
///
/// * `mesh` - The mesh to rotate
/// * `rotation` - The rotation quaternion
///
/// # Examples
///
/// ```
/// use mesh_tools::{Mesh, primitives::create_cube, modifiers::rotate_mesh};
/// use glam::Quat;
/// use std::f32::consts::PI;
///
/// let mut cube = create_cube(1.0, 1.0, 1.0);
///
/// // Rotate 90 degrees around Y axis
/// rotate_mesh(&mut cube, Quat::from_rotation_y(PI / 2.0));
///
/// // Rotate around an arbitrary axis
/// let axis = glam::Vec3::new(1.0, 1.0, 1.0).normalize();
/// rotate_mesh(&mut cube, Quat::from_axis_angle(axis, PI / 4.0));
/// ```
pub fn rotate_mesh(mesh: &mut Mesh, rotation: Quat) {
    let transform = Mat4::from_quat(rotation);
    transform_mesh(mesh, transform);
}

/// Applies a translation transformation to the mesh
///
/// Moves the mesh along the X, Y, and Z axes. This only affects vertex positions,
/// not normals or tangents (as they are direction vectors).
///
/// # Arguments
///
/// * `mesh` - The mesh to translate
/// * `translation` - The translation vector
///
/// # Examples
///
/// ```
/// use mesh_tools::{Mesh, primitives::create_cube, modifiers::translate_mesh};
/// use glam::Vec3;
///
/// let mut cube = create_cube(1.0, 1.0, 1.0);
///
/// // Move the cube up 2 units
/// translate_mesh(&mut cube, Vec3::new(0.0, 2.0, 0.0));
///
/// // Move the cube diagonally
/// translate_mesh(&mut cube, Vec3::new(1.0, 0.0, 1.0));
/// ```
pub fn translate_mesh(mesh: &mut Mesh, translation: Vec3) {
    let transform = Mat4::from_translation(translation);
    transform_mesh(mesh, transform);
}

/// Flips the normals of all vertices in the mesh
///
/// This reverses the direction of all normals and also reverses the winding order
/// of triangles to maintain consistent surface orientation.
///
/// # Arguments
///
/// * `mesh` - The mesh whose normals will be flipped
///
/// # Examples
///
/// ```
/// use mesh_tools::{Mesh, primitives::create_sphere, modifiers::flip_normals};
///
/// let mut sphere = create_sphere(1.0, 16, 8);
///
/// // Flip the normals (useful for inside-out objects like interior rooms)
/// flip_normals(&mut sphere);
/// ```
pub fn flip_normals(mesh: &mut Mesh) {
    // Reverse the direction of all normals
    for vertex in &mut mesh.vertices {
        if let Some(ref mut normal) = vertex.normal {
            *normal = -*normal;
        }
    }
    
    // Reverse the winding order of all triangles
    for triangle in &mut mesh.triangles {
        let temp = triangle.indices[0];
        triangle.indices[0] = triangle.indices[2];
        triangle.indices[2] = temp;
    }
}

/// Vertex welding parameters
///
/// Controls how vertices are welded together to simplify a mesh.
#[derive(Debug, Clone, Copy)]
pub struct WeldParameters {
    /// Distance threshold below which vertices are considered the same
    pub distance_threshold: f32,
    /// Whether to weld only vertices with similar normals
    pub check_normals: bool,
    /// Angle threshold (in radians) for considering normals similar
    pub normal_threshold: f32,
    /// Whether to weld only vertices with similar UVs
    pub check_uvs: bool,
    /// Distance threshold for considering UVs similar
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

/// Welds vertices that are within a certain distance of each other
///
/// This reduces the number of vertices in a mesh by combining those that are very close
/// together. This is useful for cleaning up imported meshes, fixing T-junctions, and
/// preparing meshes for subdivision.
///
/// # Arguments
///
/// * `mesh` - The mesh in which to weld vertices
/// * `params` - Parameters controlling the welding process
///
/// # Returns
///
/// The number of vertices that were welded, or an error if the operation failed
///
/// # Examples
///
/// ```
/// use mesh_tools::{Mesh, primitives::create_cube, modifiers::{weld_vertices, WeldParameters}};
///
/// let mut cube = create_cube(1.0, 1.0, 1.0);
///
/// // Weld vertices with default parameters
/// let welded_count = weld_vertices(&mut cube, WeldParameters::default()).unwrap();
///
/// // Or use custom parameters for more aggressive welding
/// let custom_params = WeldParameters {
///     distance_threshold: 0.01, // Weld vertices up to 0.01 units apart
///     check_normals: false,     // Ignore normal differences
///     ..Default::default()
/// };
/// let welded_count = weld_vertices(&mut cube, custom_params).unwrap();
/// ```
pub fn weld_vertices(mesh: &mut Mesh, params: WeldParameters) -> MeshResult<usize> {
    // Build a map of original vertex indices to new (potentially welded) vertex indices
    let mut vertex_map = HashMap::new();
    let mut new_vertices = Vec::new();
    
    // For each vertex in the original mesh
    for (old_idx, vertex) in mesh.vertices.iter().enumerate() {
        // Check if there's a nearby vertex we can weld to
        let mut found_match = false;
        
        for (new_idx, new_vertex) in new_vertices.iter().enumerate() {
            // Check position distance
            let dist = (vertex.position - new_vertex.position).length_squared();
            if dist > params.distance_threshold * params.distance_threshold {
                continue;
            }
            
            // Check normals if requested
            if params.check_normals && vertex.normal.is_some() && new_vertex.normal.is_some() {
                let normal_dot = vertex.normal.unwrap().dot(new_vertex.normal.unwrap());
                if normal_dot < 1.0 - params.normal_threshold {
                    continue;
                }
            }
            
            // Check UVs if requested
            if params.check_uvs && vertex.uv.is_some() && new_vertex.uv.is_some() {
                let uv_dist = (vertex.uv.unwrap() - new_vertex.uv.unwrap()).length_squared();
                if uv_dist > params.uv_threshold * params.uv_threshold {
                    continue;
                }
            }
            
            // If we got here, we found a match
            vertex_map.insert(old_idx, new_idx);
            found_match = true;
            break;
        }
        
        // If no match found, add this as a new vertex
        if !found_match {
            vertex_map.insert(old_idx, new_vertices.len());
            new_vertices.push(vertex.clone());
        }
    }
    
    // Count how many vertices were welded
    let welded_count = mesh.vertices.len() - new_vertices.len();
    
    // Update triangles with new vertex indices
    for triangle in &mut mesh.triangles {
        for i in 0..3 {
            if let Some(&new_idx) = vertex_map.get(&triangle.indices[i]) {
                triangle.indices[i] = new_idx;
            } else {
                return Err(MeshError::InvalidIndex(triangle.indices[i]));
            }
        }
    }
    
    // Replace vertices
    mesh.vertices = new_vertices;
    
    Ok(welded_count)
}

/// Removes unused vertices (not referenced by any triangle)
///
/// Scans the mesh for vertices that aren't used in any triangle and removes them,
/// remapping the remaining triangle indices as needed.
///
/// # Arguments
///
/// * `mesh` - The mesh to clean up
///
/// # Returns
///
/// The number of vertices removed
///
/// # Examples
///
/// ```
/// use mesh_tools::{Mesh, Vertex, Triangle, modifiers::remove_unused_vertices};
/// use glam::Vec3;
///
/// // Create a mesh with an unused vertex
/// let mut mesh = Mesh::new();
/// let v0 = mesh.add_vertex(Vertex::new(Vec3::new(0.0, 0.0, 0.0)));
/// let v1 = mesh.add_vertex(Vertex::new(Vec3::new(1.0, 0.0, 0.0)));
/// let v2 = mesh.add_vertex(Vertex::new(Vec3::new(0.0, 1.0, 0.0)));
/// let unused = mesh.add_vertex(Vertex::new(Vec3::new(2.0, 2.0, 2.0))); // Unused vertex
///
/// mesh.add_triangle(v0, v1, v2).unwrap();
///
/// // Remove unused vertices
/// let removed_count = remove_unused_vertices(&mut mesh);
/// assert_eq!(removed_count, 1); // One vertex should be removed
/// ```
pub fn remove_unused_vertices(mesh: &mut Mesh) -> usize {
    // Find which vertices are used
    let mut used_vertices = vec![false; mesh.vertices.len()];
    
    for triangle in &mesh.triangles {
        for &idx in &triangle.indices {
            if idx < used_vertices.len() {
                used_vertices[idx] = true;
            }
        }
    }
    
    // Count unused vertices
    let unused_count = used_vertices.iter().filter(|&&used| !used).count();
    
    if unused_count == 0 {
        return 0; // Early return if no unused vertices
    }
    
    // Build a map from old indices to new indices
    let mut index_map = vec![0; mesh.vertices.len()];
    let mut new_index = 0;
    
    for (old_idx, &used) in used_vertices.iter().enumerate() {
        if used {
            index_map[old_idx] = new_index;
            new_index += 1;
        }
    }
    
    // Create new vertex list with only used vertices
    let mut new_vertices = Vec::with_capacity(mesh.vertices.len() - unused_count);
    
    for (idx, vertex) in mesh.vertices.iter().enumerate() {
        if used_vertices[idx] {
            new_vertices.push(vertex.clone());
        }
    }
    
    // Update triangle indices
    for triangle in &mut mesh.triangles {
        for i in 0..3 {
            triangle.indices[i] = index_map[triangle.indices[i]];
        }
    }
    
    // Replace vertices
    mesh.vertices = new_vertices;
    
    unused_count
}

/// Remove degenerate triangles (triangles with duplicate vertices or zero area)
///
/// Degenerate triangles can cause rendering artifacts and physics problems, so it's
/// often useful to remove them.
///
/// # Arguments
///
/// * `mesh` - The mesh to clean up
///
/// # Returns
///
/// The number of triangles removed
///
/// # Examples
///
/// ```
/// use mesh_tools::{Mesh, Vertex, Triangle, modifiers::remove_degenerate_triangles};
/// use glam::Vec3;
///
/// // Create a mesh with a degenerate triangle
/// let mut mesh = Mesh::new();
/// let v0 = mesh.add_vertex(Vertex::new(Vec3::new(0.0, 0.0, 0.0)));
/// let v1 = mesh.add_vertex(Vertex::new(Vec3::new(1.0, 0.0, 0.0)));
///
/// // Add a valid triangle
/// mesh.add_triangle(v0, v1, mesh.add_vertex(Vertex::new(Vec3::new(0.0, 1.0, 0.0)))).unwrap();
/// // Add a degenerate triangle (two vertices are the same)
/// mesh.add_triangle(v0, v1, v1).unwrap();
///
/// // Remove degenerate triangles
/// let removed_count = remove_degenerate_triangles(&mut mesh);
/// assert_eq!(removed_count, 1); // One triangle should be removed
/// ```
pub fn remove_degenerate_triangles(mesh: &mut Mesh) -> usize {
    let initial_count = mesh.triangles.len();
    
    // Filter out degenerate triangles
    mesh.triangles.retain(|triangle| {
        // Check for duplicate indices
        let [a, b, c] = triangle.indices;
        if a == b || b == c || a == c {
            return false;
        }
        
        // Check for zero area (collinear vertices)
        let va = mesh.vertices[a].position;
        let vb = mesh.vertices[b].position;
        let vc = mesh.vertices[c].position;
        
        let edge1 = vb - va;
        let edge2 = vc - va;
        let cross = edge1.cross(edge2);
        
        cross.length_squared() > 1e-10 // Small threshold for floating point imprecision
    });
    
    initial_count - mesh.triangles.len()
}

/// Generates smooth vertex normals for a mesh by averaging face normals
///
/// This creates smooth normals by averaging the normals of all triangles that
/// share each vertex. This is useful for creating smooth surfaces.
///
/// # Arguments
///
/// * `mesh` - The mesh for which to generate normals
///
/// # Examples
///
/// ```
/// use mesh_tools::{primitives::create_cube, modifiers::generate_smooth_normals};
///
/// // Create a cube and generate smooth normals
/// let mut cube = create_cube(1.0, 1.0, 1.0);
/// generate_smooth_normals(&mut cube);
/// ```
pub fn generate_smooth_normals(mesh: &mut Mesh) {
    // Initialize normal accumulators
    let mut normal_sums = vec![Vec3::ZERO; mesh.vertices.len()];
    
    // Accumulate face normals to vertices
    for triangle in &mesh.triangles {
        let [a, b, c] = triangle.indices;
        
        // Get the vertex positions
        let va = mesh.vertices[a].position;
        let vb = mesh.vertices[b].position;
        let vc = mesh.vertices[c].position;
        
        // Calculate the face normal using the cross product
        let edge1 = vb - va;
        let edge2 = vc - va;
        let normal = edge1.cross(edge2).normalize();
        
        // Accumulate to each vertex
        normal_sums[a] += normal;
        normal_sums[b] += normal;
        normal_sums[c] += normal;
    }
    
    // Normalize and assign the final normals
    for (i, normal_sum) in normal_sums.iter().enumerate() {
        if normal_sum.length_squared() > 0.0 {
            mesh.vertices[i].normal = Some(normal_sum.normalize());
        } else {
            // Fall back to a default normal if no contribution
            mesh.vertices[i].normal = Some(Vec3::Y);
        }
    }
}

/// Subdivides each triangle into 4 triangles
///
/// This increases mesh resolution by splitting each edge at its midpoint and
/// creating 4 new triangles from each original triangle. This is useful for
/// adding detail to a mesh.
///
/// # Arguments
///
/// * `mesh` - The mesh to subdivide
///
/// # Returns
///
/// `Ok(())` if subdivision was successful, or an error if it failed
///
/// # Examples
///
/// ```
/// use mesh_tools::{primitives::create_cube, modifiers::subdivide_mesh};
///
/// // Create a cube and subdivide it
/// let mut cube = create_cube(1.0, 1.0, 1.0);
/// subdivide_mesh(&mut cube).unwrap();
/// // Each face of the cube now has 4 triangles instead of 2
/// ```
pub fn subdivide_mesh(mesh: &mut Mesh) -> MeshResult<()> {
    // Cache for midpoint indices to avoid creating duplicate vertices
    let mut edge_midpoints = HashMap::new();
    
    // Store the original triangles
    let original_triangles = mesh.triangles.clone();
    
    // Clear triangles to rebuild
    mesh.triangles.clear();
    
    // For each original triangle
    for triangle in &original_triangles {
        let [a, b, c] = triangle.indices;
        
        // Get or create midpoints
        let ab = get_or_create_midpoint(mesh, a, b, &mut edge_midpoints)?;
        let bc = get_or_create_midpoint(mesh, b, c, &mut edge_midpoints)?;
        let ca = get_or_create_midpoint(mesh, c, a, &mut edge_midpoints)?;
        
        // Create 4 new triangles
        mesh.add_triangle(a, ab, ca)?;     // Triangle 1
        mesh.add_triangle(ab, b, bc)?;     // Triangle 2
        mesh.add_triangle(ca, bc, c)?;     // Triangle 3
        mesh.add_triangle(ab, bc, ca)?;    // Triangle 4 (center)
    }
    
    Ok(())
}

/// Helper function for subdivision to find or create midpoints
///
/// This finds or creates a vertex at the midpoint of two existing vertices.
/// It caches results to avoid creating duplicate vertices.
///
/// # Arguments
///
/// * `mesh` - The mesh to modify
/// * `a`, `b` - Indices of the two vertices to find the midpoint between
/// * `cache` - Cache of previously computed midpoints
///
/// # Returns
///
/// The index of the midpoint vertex, or an error if it couldn't be created
fn get_or_create_midpoint(
    mesh: &mut Mesh,
    a: usize,
    b: usize,
    cache: &mut HashMap<(usize, usize), usize>
) -> MeshResult<usize> {
    // Ensure a < b for consistent key ordering
    let (min_idx, max_idx) = if a < b { (a, b) } else { (b, a) };
    let key = (min_idx, max_idx);
    
    // Check if we've already created this midpoint
    if let Some(&midpoint_idx) = cache.get(&key) {
        return Ok(midpoint_idx);
    }
    
    // Get the vertices
    if min_idx >= mesh.vertices.len() || max_idx >= mesh.vertices.len() {
        return Err(MeshError::InvalidIndex(std::cmp::max(min_idx, max_idx)));
    }
    
    let v1 = &mesh.vertices[min_idx];
    let v2 = &mesh.vertices[max_idx];
    
    // Create the midpoint position
    let pos = (v1.position + v2.position) * 0.5;
    
    // Create the midpoint normal if both vertices have normals
    let normal = match (v1.normal, v2.normal) {
        (Some(n1), Some(n2)) => Some((n1 + n2).normalize()),
        _ => None,
    };
    
    // Create the midpoint UV if both vertices have UVs
    let uv = match (v1.uv, v2.uv) {
        (Some(uv1), Some(uv2)) => Some((uv1 + uv2) * 0.5),
        _ => None,
    };
    
    // Create the midpoint vertex
    let mut vertex = Vertex::new(pos);
    
    if let Some(n) = normal {
        vertex.normal = Some(n);
    }
    
    if let Some(u) = uv {
        vertex.uv = Some(u);
    }
    
    // Add the vertex to the mesh
    let midpoint_idx = mesh.add_vertex(vertex);
    
    // Cache the result
    cache.insert(key, midpoint_idx);
    
    Ok(midpoint_idx)
}

/// Extrudes faces along their normals
///
/// This creates new geometry by moving a set of faces along their normals and
/// connecting the original edges to the moved edges with new faces.
///
/// # Arguments
///
/// * `mesh` - The mesh to modify
/// * `faces` - Indices of the faces to extrude
/// * `amount` - Distance to extrude along the face normal
///
/// # Returns
///
/// `Ok(())` if extrusion was successful, or an error if it failed
///
/// # Examples
///
/// ```
/// use mesh_tools::{primitives::create_cube, modifiers::extrude_faces};
///
/// // Create a cube
/// let mut cube = create_cube(1.0, 1.0, 1.0);
///
/// // Extrude the top face
/// let top_face_indices = vec![10, 11]; // Indices of triangles on the top face (example)
/// extrude_faces(&mut cube, &top_face_indices, 0.5).unwrap();
/// ```
pub fn extrude_faces(mesh: &mut Mesh, faces: &[usize], amount: f32) -> MeshResult<()> {
    if faces.is_empty() {
        return Ok(());
    }
    
    // Collect all vertices used by the faces to extrude
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
        
        // Calculate face normal
        let va = mesh.vertices[a].position;
        let vb = mesh.vertices[b].position;
        let vc = mesh.vertices[c].position;
        
        let normal = (vb - va).cross(vc - va).normalize();
        face_normals.push((face_idx, normal));
    }
    
    // Create new vertices by extruding along normals
    let mut vertex_map = HashMap::new();
    
    for &old_idx in &vertices_to_extrude {
        let old_vertex = &mesh.vertices[old_idx];
        let mut new_vertex = old_vertex.clone();
        
        // Find the average normal of all faces using this vertex
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
            
            // Extrude the vertex
            new_vertex.position += avg_normal * amount;
            
            // Add the new vertex
            let new_idx = mesh.add_vertex(new_vertex);
            vertex_map.insert(old_idx, new_idx);
        }
    }
    
    // Create the new faces
    for &face_idx in faces {
        let triangle = mesh.triangles[face_idx];
        let [a, b, c] = triangle.indices;
        
        // Get the extruded vertex indices
        let a_new = *vertex_map.get(&a).ok_or(MeshError::InvalidIndex(a))?;
        let b_new = *vertex_map.get(&b).ok_or(MeshError::InvalidIndex(b))?;
        let c_new = *vertex_map.get(&c).ok_or(MeshError::InvalidIndex(c))?;
        
        // Create the extruded face
        mesh.add_triangle(a_new, c_new, b_new)?; // Note: flipped winding order for correct normals
        
        // Create quads to connect old and new faces
        mesh.add_triangle(a, b, b_new)?;
        mesh.add_triangle(a, b_new, a_new)?;
        
        mesh.add_triangle(b, c, c_new)?;
        mesh.add_triangle(b, c_new, b_new)?;
        
        mesh.add_triangle(c, a, a_new)?;
        mesh.add_triangle(c, a_new, c_new)?;
    }
    
    Ok(())
}
