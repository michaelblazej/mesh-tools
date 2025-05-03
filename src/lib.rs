/*!
# Mesh Tools

A Rust library for 3D mesh creation, manipulation, and export. This crate provides tools for:

- Creating primitive 3D shapes (cubes, spheres, planes, etc.)
- Manipulating meshes with various transformations and modifiers
- Exporting meshes to industry-standard formats (GLB/glTF)
- Organizing multiple meshes into scenes with materials
- Breaking meshes into meshlets for GPU-driven rendering

## Example

```rust
use mesh_tools::{
    Mesh, Scene,
    primitives::create_cube,
    modifiers::{translate_mesh, generate_smooth_normals},
    export::{Material, export_to_glb, GlbExportOptions}
};
use glam::Vec3;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a cube
    let mut cube = create_cube(1.0, 1.0, 1.0);
    
    // Generate smooth normals
    generate_smooth_normals(&mut cube);
    
    // Move it up
    translate_mesh(&mut cube, Vec3::new(0.0, 1.0, 0.0));
    
    // Add a material
    let blue_material = Material {
        name: "Blue".to_string(),
        base_color: [0.1, 0.4, 0.8],
        metallic: 0.0,
        roughness: 0.5,
        emissive: [0.0, 0.0, 0.0],
    };
    
    // Export to GLB
    let options = GlbExportOptions {
        material: blue_material,
        export_normals: true,
        export_tangents: false,
        export_uvs: true,
        export_colors: false,
        name: "my_cube".to_string(),
    };
    
    export_to_glb(&cube, "cube.glb", options)?;
    
    Ok(())
}
```
*/

use glam::{Vec2, Vec3, Vec4};
use std::collections::HashMap;
use thiserror::Error;

pub mod primitives;
pub mod modifiers;
pub mod export;

/// Errors that can occur when working with meshes
#[derive(Error, Debug)]
pub enum MeshError {
    /// An index is out of bounds for the mesh's vertex array
    #[error("Invalid index: {0}")]
    InvalidIndex(usize),
    
    /// The mesh contains attributes with inconsistent counts
    #[error("Incompatible attribute counts")]
    IncompatibleAttributeCounts,
    
    /// A required attribute is missing from the mesh
    #[error("Missing required attribute")]
    MissingRequiredAttribute,
    
    /// The face data is malformed (e.g., not enough vertices for a face)
    #[error("Invalid face format")]
    InvalidFaceFormat,
}

/// Result type for mesh operations
pub type MeshResult<T> = Result<T, MeshError>;

/// Represents a unique vertex with all its attributes
///
/// A vertex is the fundamental building block of a mesh, containing a position in 3D space
/// and optional attributes like normals, texture coordinates (UVs), tangents, and colors.
///
/// # Examples
///
/// ```
/// use mesh_tools::Vertex;
/// use glam::Vec3;
///
/// // Create a vertex with just position
/// let vertex1 = Vertex::new(Vec3::new(0.0, 0.0, 0.0));
///
/// // Create a vertex with position and normal
/// let vertex2 = Vertex::with_normal(
///     Vec3::new(1.0, 0.0, 0.0),
///     Vec3::new(1.0, 0.0, 0.0)
/// );
/// ```
#[derive(Debug, Clone)]
pub struct Vertex {
    /// Position in 3D space
    pub position: Vec3,
    /// Normal vector (optional)
    pub normal: Option<Vec3>,
    /// Texture coordinates (optional)
    pub uv: Option<Vec2>,
    /// Tangent vector with handedness in w component (optional)
    pub tangent: Option<Vec4>,
    /// Vertex color (optional)
    pub color: Option<Vec3>,
}

impl Vertex {
    /// Create a new vertex with just position
    ///
    /// # Arguments
    ///
    /// * `position` - The position of the vertex in 3D space
    ///
    /// # Returns
    ///
    /// A new vertex with the given position and no other attributes
    pub fn new(position: Vec3) -> Self {
        Self {
            position,
            normal: None,
            uv: None,
            tangent: None,
            color: None,
        }
    }

    /// Create a vertex with position and normal
    ///
    /// # Arguments
    ///
    /// * `position` - The position of the vertex in 3D space
    /// * `normal` - The normal vector for the vertex (should be normalized)
    ///
    /// # Returns
    ///
    /// A new vertex with the given position and normal
    pub fn with_normal(position: Vec3, normal: Vec3) -> Self {
        Self {
            position,
            normal: Some(normal),
            uv: None,
            tangent: None,
            color: None,
        }
    }

    /// Create a vertex with position and UV coordinates
    ///
    /// # Arguments
    ///
    /// * `position` - The position of the vertex in 3D space
    /// * `uv` - The texture coordinates for the vertex
    ///
    /// # Returns
    ///
    /// A new vertex with the given position and UV coordinates
    pub fn with_uv(position: Vec3, uv: Vec2) -> Self {
        Self {
            position,
            normal: None,
            uv: Some(uv),
            tangent: None,
            color: None,
        }
    }

    /// Create a fully-specified vertex with position, normal, and UV coordinates
    ///
    /// # Arguments
    ///
    /// * `position` - The position of the vertex in 3D space
    /// * `normal` - The normal vector for the vertex (should be normalized)
    /// * `uv` - The texture coordinates for the vertex
    ///
    /// # Returns
    ///
    /// A new vertex with the given position, normal, and UV coordinates
    pub fn with_all(position: Vec3, normal: Vec3, uv: Vec2) -> Self {
        Self {
            position,
            normal: Some(normal),
            uv: Some(uv),
            tangent: None,
            color: None,
        }
    }

    /// Add a normal to an existing vertex
    ///
    /// # Arguments
    ///
    /// * `normal` - The normal vector to add
    ///
    /// # Returns
    ///
    /// The modified vertex with the normal added
    pub fn with_normal_added(mut self, normal: Vec3) -> Self {
        self.normal = Some(normal);
        self
    }

    /// Add UV coordinates to an existing vertex
    ///
    /// # Arguments
    ///
    /// * `uv` - The texture coordinates to add
    ///
    /// # Returns
    ///
    /// The modified vertex with the UV coordinates added
    pub fn with_uv_added(mut self, uv: Vec2) -> Self {
        self.uv = Some(uv);
        self
    }

    /// Add a tangent vector to an existing vertex
    ///
    /// # Arguments
    ///
    /// * `tangent` - The tangent vector to add, with handedness in w component
    ///
    /// # Returns
    ///
    /// The modified vertex with the tangent added
    pub fn with_tangent_added(mut self, tangent: Vec4) -> Self {
        self.tangent = Some(tangent);
        self
    }

    /// Add a color to an existing vertex
    ///
    /// # Arguments
    ///
    /// * `color` - The RGB color to add
    ///
    /// # Returns
    ///
    /// The modified vertex with the color added
    pub fn with_color_added(mut self, color: Vec3) -> Self {
        self.color = Some(color);
        self
    }
}

/// Represents a triangle face using vertex indices
///
/// A triangle is defined by three indices into a mesh's vertex array.
/// These indices are stored in clockwise winding order by convention.
#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    /// Indices of the three vertices that make up this triangle
    pub indices: [usize; 3],
}

impl Triangle {
    /// Create a new triangle from three vertex indices
    ///
    /// # Arguments
    ///
    /// * `a`, `b`, `c` - Indices of the three vertices that make up this triangle
    pub fn new(a: usize, b: usize, c: usize) -> Self {
        Self { indices: [a, b, c] }
    }
}

/// Represents a polygon (face with arbitrary number of vertices)
///
/// A polygon is a face with an arbitrary number of vertices, which can be
/// triangulated into multiple triangles.
#[derive(Debug, Clone)]
pub struct Polygon {
    /// Indices of the vertices that make up this polygon
    pub indices: Vec<usize>,
}

impl Polygon {
    /// Create a new polygon from a list of vertex indices
    pub fn new(indices: Vec<usize>) -> Self {
        Self { indices }
    }

    /// Triangulate the polygon into a list of triangles
    ///
    /// Uses a simple fan triangulation which only works reliably for convex polygons.
    /// For concave polygons, a more sophisticated triangulation algorithm should be used.
    ///
    /// # Returns
    ///
    /// A vector of triangles that make up this polygon
    pub fn triangulate(&self) -> Vec<Triangle> {
        if self.indices.len() < 3 {
            return Vec::new();
        }

        let mut triangles = Vec::new();

        // Simple fan triangulation (only reliable for convex polygons)
        for i in 1..(self.indices.len() - 1) {
            triangles.push(Triangle::new(
                self.indices[0],
                self.indices[i],
                self.indices[i + 1],
            ));
        }

        triangles
    }
}

/// Represents an edge in the mesh
///
/// An edge connects two vertices in the mesh. Edges are stored with the
/// smaller index first for canonicalization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Edge {
    /// Indices of the two vertices that make up this edge
    pub vertices: [usize; 2],
}

impl Edge {
    /// Create a new edge between two vertices
    ///
    /// The edge is canonicalized so that the smaller vertex index is always first.
    ///
    /// # Arguments
    ///
    /// * `a`, `b` - Indices of the two vertices that make up this edge
    pub fn new(a: usize, b: usize) -> Self {
        if a < b {
            Self { vertices: [a, b] }
        } else {
            Self { vertices: [b, a] }
        }
    }
}

/// Attribute types that can be stored on a mesh
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AttributeType {
    /// Position (Vec3)
    Position,
    /// Normal vector (Vec3)
    Normal,
    /// Texture coordinates (Vec2)
    UV,
    /// Tangent vector (Vec4, with handedness in w)
    Tangent,
    /// Color (Vec3)
    Color,
    /// Custom attribute (identified by string name)
    Custom(String),
}

/// A mesh containing vertices, faces, and optional attributes
///
/// The mesh is the central data structure in this library, storing the geometry
/// and attributes of a 3D model. It supports various operations like adding vertices,
/// faces, and calculating normals.
///
/// # Examples
///
/// ```
/// use mesh_tools::{Mesh, Vertex, Triangle};
/// use glam::Vec3;
///
/// // Create an empty mesh
/// let mut mesh = Mesh::new();
///
/// // Add vertices
/// let v0 = mesh.add_vertex(Vertex::new(Vec3::new(0.0, 0.0, 0.0)));
/// let v1 = mesh.add_vertex(Vertex::new(Vec3::new(1.0, 0.0, 0.0)));
/// let v2 = mesh.add_vertex(Vertex::new(Vec3::new(0.0, 1.0, 0.0)));
///
/// // Add a triangle
/// mesh.add_triangle(v0, v1, v2).expect("Invalid vertex indices");
///
/// // Calculate normals
/// mesh.calculate_normals();
/// ```
#[derive(Debug, Clone)]
pub struct Mesh {
    /// Vertices in the mesh
    pub vertices: Vec<Vertex>,
    /// Triangles in the mesh
    pub triangles: Vec<Triangle>,
    /// Custom attributes
    pub custom_attributes: HashMap<String, Vec<Vec3>>,
    /// Material for the mesh (if any)
    pub material: Option<export::Material>,
}

impl Mesh {
    /// Create a new empty mesh
    ///
    /// # Returns
    ///
    /// An empty mesh with no vertices or triangles
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            triangles: Vec::new(),
            custom_attributes: HashMap::new(),
            material: None,
        }
    }

    /// Create a mesh from a list of vertices and triangles
    ///
    /// # Arguments
    ///
    /// * `vertices` - List of vertices in the mesh
    /// * `triangles` - List of triangles in the mesh
    ///
    /// # Returns
    ///
    /// A new mesh with the given vertices and triangles
    pub fn from_triangles(vertices: Vec<Vertex>, triangles: Vec<Triangle>) -> Self {
        Self {
            vertices,
            triangles,
            custom_attributes: HashMap::new(),
            material: None,
        }
    }

    /// Create a mesh from a list of vertices and polygons
    ///
    /// Each polygon is triangulated into one or more triangles.
    ///
    /// # Arguments
    ///
    /// * `vertices` - List of vertices in the mesh
    /// * `polygons` - List of polygons in the mesh
    ///
    /// # Returns
    ///
    /// A new mesh with the given vertices and triangulated polygons
    pub fn from_polygons(vertices: Vec<Vertex>, polygons: Vec<Polygon>) -> Self {
        let mut triangles = Vec::new();

        for polygon in polygons {
            triangles.extend(polygon.triangulate());
        }

        Self {
            vertices,
            triangles,
            custom_attributes: HashMap::new(),
            material: None,
        }
    }

    /// Add a vertex to the mesh
    ///
    /// # Arguments
    ///
    /// * `vertex` - The vertex to add
    ///
    /// # Returns
    ///
    /// The index of the added vertex
    pub fn add_vertex(&mut self, vertex: Vertex) -> usize {
        let index = self.vertices.len();
        self.vertices.push(vertex);
        index
    }

    /// Add a triangle to the mesh
    ///
    /// # Arguments
    ///
    /// * `a`, `b`, `c` - Indices of the three vertices that make up this triangle
    ///
    /// # Returns
    ///
    /// The index of the added triangle, or an error if any vertex index is invalid
    pub fn add_triangle(&mut self, a: usize, b: usize, c: usize) -> MeshResult<usize> {
        let max_index = self.vertices.len().saturating_sub(1);
        if a > max_index || b > max_index || c > max_index {
            return Err(MeshError::InvalidIndex(std::cmp::max(a, std::cmp::max(b, c))));
        }
        let index = self.triangles.len();
        self.triangles.push(Triangle::new(a, b, c));
        Ok(index)
    }

    /// Add a polygon to the mesh and triangulate it
    ///
    /// # Arguments
    ///
    /// * `indices` - Indices of the vertices that make up this polygon
    ///
    /// # Returns
    ///
    /// Ok(()) if successful, or an error if any vertex index is invalid
    pub fn add_polygon(&mut self, indices: Vec<usize>) -> MeshResult<()> {
        if indices.len() < 3 {
            return Err(MeshError::InvalidFaceFormat);
        }

        let max_index = self.vertices.len().saturating_sub(1);
        for &idx in &indices {
            if idx > max_index {
                return Err(MeshError::InvalidIndex(idx));
            }
        }

        let polygon = Polygon::new(indices);
        let triangles = polygon.triangulate();
        
        for triangle in triangles {
            self.triangles.push(triangle);
        }

        Ok(())
    }

    /// Check if the mesh has normals
    ///
    /// # Returns
    ///
    /// `true` if all vertices have normals, `false` otherwise
    pub fn has_normals(&self) -> bool {
        self.vertices.iter().all(|v| v.normal.is_some())
    }

    /// Check if the mesh has texture coordinates
    ///
    /// # Returns
    ///
    /// `true` if all vertices have UVs, `false` otherwise
    pub fn has_uvs(&self) -> bool {
        self.vertices.iter().all(|v| v.uv.is_some())
    }

    /// Calculate vertex normals for the mesh
    ///
    /// This computes face normals for each triangle and then averages
    /// them at each vertex to create smooth normals.
    pub fn calculate_normals(&mut self) {
        // Initialize normal accumulation
        let mut normal_sums = vec![Vec3::ZERO; self.vertices.len()];
        
        // Calculate face normals for each triangle
        for triangle in &self.triangles {
            if let [a, b, c] = triangle.indices {
                let va = self.vertices[a].position;
                let vb = self.vertices[b].position;
                let vc = self.vertices[c].position;
                
                // Calculate face normal using cross product
                let edge1 = vb - va;
                let edge2 = vc - va;
                let face_normal = edge1.cross(edge2).normalize();
                
                // Add face normal to all vertices of the triangle
                normal_sums[a] += face_normal;
                normal_sums[b] += face_normal;
                normal_sums[c] += face_normal;
            }
        }
        
        // Normalize and assign normals
        for (i, sum) in normal_sums.iter().enumerate() {
            if sum.length_squared() > 0.0 {
                self.vertices[i].normal = Some(sum.normalize());
            } else {
                // If no contributing faces, use a default up vector
                self.vertices[i].normal = Some(Vec3::new(0.0, 1.0, 0.0));
            }
        }
    }

    /// Get all edges in the mesh
    ///
    /// # Returns
    ///
    /// A vector of all unique edges in the mesh
    pub fn get_edges(&self) -> Vec<Edge> {
        let mut edges = std::collections::HashSet::new();
        
        // Extract edges from triangles
        for triangle in &self.triangles {
            if let [a, b, c] = triangle.indices {
                edges.insert(Edge::new(a, b));
                edges.insert(Edge::new(b, c));
                edges.insert(Edge::new(c, a));
            }
        }
        
        edges.into_iter().collect()
    }

    /// Add a custom attribute to the mesh
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the attribute
    /// * `values` - List of values, one per vertex
    ///
    /// # Returns
    ///
    /// Ok(()) if successful, or an error if the value count doesn't match the vertex count
    pub fn add_custom_attribute(&mut self, name: &str, values: Vec<Vec3>) -> MeshResult<()> {
        if values.len() != self.vertices.len() {
            return Err(MeshError::IncompatibleAttributeCounts);
        }
        
        self.custom_attributes.insert(name.to_string(), values);
        Ok(())
    }

    /// Set the material for this mesh
    ///
    /// # Arguments
    ///
    /// * `material` - The material to apply to this mesh
    pub fn with_material(&mut self, material: export::Material) -> &mut Self {
        self.material = Some(material);
        self
    }

    /// Get the material for this mesh, if one exists
    pub fn get_material(&self) -> Option<&export::Material> {
        self.material.as_ref()
    }
}

/// A meshlet is a small, cache-friendly subset of a larger mesh
///
/// Meshlets are used in GPU-driven rendering to improve cache locality
/// and enable efficient culling. Each meshlet contains a small number of
/// vertices and triangles.
#[derive(Debug, Clone)]
pub struct Meshlet {
    /// Local vertices in this meshlet
    pub vertices: Vec<usize>,
    /// Local triangles in this meshlet
    pub triangles: Vec<[u8; 3]>,
    /// Center of the meshlet for culling
    pub center: Vec3,
    /// Radius of the meshlet for culling
    pub radius: f32,
}

impl Meshlet {
    /// Create a new empty meshlet
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            triangles: Vec::new(),
            center: Vec3::ZERO,
            radius: 0.0,
        }
    }
}

/// A collection of meshlets that make up a complete mesh
///
/// MeshletGroup is used to break a mesh into small, GPU-friendly chunks
/// for efficient GPU-driven rendering.
#[derive(Debug, Clone)]
pub struct MeshletGroup {
    /// Original vertices of the whole mesh
    pub vertices: Vec<Vertex>,
    /// Collection of meshlets
    pub meshlets: Vec<Meshlet>,
}

impl MeshletGroup {
    /// Create meshlets from an existing mesh, useful for GPU-driven rendering
    ///
    /// This method breaks down a mesh into smaller groups of vertices and triangles
    /// (meshlets) that are optimized for GPU cache locality and culling.
    ///
    /// # Arguments
    ///
    /// * `mesh` - The original mesh to break into meshlets
    /// * `max_vertices` - Maximum number of vertices per meshlet
    /// * `max_triangles` - Maximum number of triangles per meshlet
    ///
    /// # Returns
    ///
    /// A MeshletGroup containing the original vertices and the constructed meshlets
    pub fn from_mesh(mesh: Mesh, max_vertices: usize, max_triangles: usize) -> Self {
        let mut meshlets = Vec::new();
        let mut current_meshlet = Meshlet::new();
        
        let mut vertex_remap = HashMap::new();
        
        for triangle in &mesh.triangles {
            // If this meshlet is full, finalize it and start a new one
            if current_meshlet.triangles.len() >= max_triangles ||
               current_meshlet.vertices.len() + 3 > max_vertices {
                
                if !current_meshlet.triangles.is_empty() {
                    // Calculate center and radius for culling
                    let mut center = Vec3::ZERO;
                    for &vi in &current_meshlet.vertices {
                        center += mesh.vertices[vi].position;
                    }
                    center /= current_meshlet.vertices.len() as f32;
                    
                    let mut radius = 0.0f32;
                    for &vi in &current_meshlet.vertices {
                        let distance = (mesh.vertices[vi].position - center).length();
                        radius = radius.max(distance);
                    }
                    
                    current_meshlet.center = center;
                    current_meshlet.radius = radius;
                    
                    meshlets.push(std::mem::replace(&mut current_meshlet, Meshlet::new()));
                    vertex_remap.clear();
                }
            }
            
            // Try to add this triangle to the current meshlet
            let mut local_indices = [0u8; 3];
            let mut all_vertices_mapped = true;
            
            for i in 0..3 {
                let global_idx = triangle.indices[i];
                
                if let Some(&local_idx) = vertex_remap.get(&global_idx) {
                    local_indices[i] = local_idx;
                } else {
                    all_vertices_mapped = false;
                }
            }
            
            // If not all vertices are already in the meshlet, add them
            if !all_vertices_mapped {
                for i in 0..3 {
                    let global_idx = triangle.indices[i];
                    
                    if !vertex_remap.contains_key(&global_idx) {
                        let local_idx = current_meshlet.vertices.len();
                        if local_idx >= 255 {
                            // Too many vertices for a meshlet (indices stored as u8)
                            // Finalize this meshlet and start a new one
                            
                            // Calculate center and radius
                            let mut center = Vec3::ZERO;
                            for &vi in &current_meshlet.vertices {
                                center += mesh.vertices[vi].position;
                            }
                            center /= current_meshlet.vertices.len() as f32;
                            
                            let mut radius = 0.0f32;
                            for &vi in &current_meshlet.vertices {
                                let distance = (mesh.vertices[vi].position - center).length();
                                radius = radius.max(distance);
                            }
                            
                            current_meshlet.center = center;
                            current_meshlet.radius = radius;
                            
                            meshlets.push(std::mem::replace(&mut current_meshlet, Meshlet::new()));
                            vertex_remap.clear();
                            
                            // This triangle will be processed in the next iteration
                            break;
                        }
                        
                        current_meshlet.vertices.push(global_idx);
                        vertex_remap.insert(global_idx, local_idx as u8);
                        local_indices[i] = local_idx as u8;
                    }
                }
                
                // Add the triangle if all vertices were successfully mapped
                if vertex_remap.contains_key(&triangle.indices[0]) &&
                   vertex_remap.contains_key(&triangle.indices[1]) &&
                   vertex_remap.contains_key(&triangle.indices[2]) {
                    current_meshlet.triangles.push(local_indices);
                }
            } else {
                // All vertices already in the meshlet, just add the triangle
                current_meshlet.triangles.push(local_indices);
            }
        }
        
        // Add final meshlet if not empty
        if !current_meshlet.triangles.is_empty() {
            // Calculate center and radius
            let mut center = Vec3::ZERO;
            for &vi in &current_meshlet.vertices {
                center += mesh.vertices[vi].position;
            }
            center /= current_meshlet.vertices.len() as f32;
            
            let mut radius = 0.0f32;
            for &vi in &current_meshlet.vertices {
                let distance = (mesh.vertices[vi].position - center).length();
                radius = radius.max(distance);
            }
            
            current_meshlet.center = center;
            current_meshlet.radius = radius;
            
            meshlets.push(current_meshlet);
        }
        
        Self {
            vertices: mesh.vertices,
            meshlets,
        }
    }
}

/// A scene container for multiple meshes with different materials
///
/// The Scene struct is used to organize multiple meshes into a single coherent
/// scene that can be exported as a whole.
#[derive(Debug, Clone)]
pub struct Scene {
    /// Name of the scene
    pub name: String,
    /// Collection of meshes in the scene
    pub meshes: Vec<Mesh>,
}

impl Scene {
    /// Create a new empty scene
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the scene
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            meshes: Vec::new(),
        }
    }

    /// Add a mesh to the scene
    ///
    /// # Arguments
    ///
    /// * `mesh` - The mesh to add to the scene
    pub fn add_mesh(&mut self, mesh: Mesh) -> &mut Self {
        self.meshes.push(mesh);
        self
    }

    /// Create a scene with a single mesh
    ///
    /// # Arguments
    ///
    /// * `mesh` - The mesh to include in the scene
    /// * `name` - Name of the scene
    pub fn from_mesh(mesh: Mesh, name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            meshes: vec![mesh],
        }
    }
}
