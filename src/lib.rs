use glam::{Vec2, Vec3, Vec4};
use std::collections::HashMap;
use thiserror::Error;

pub mod primitives;
pub mod modifiers;
pub mod export;

/// Errors that can occur when working with meshes
#[derive(Error, Debug)]
pub enum MeshError {
    #[error("Invalid index: {0}")]
    InvalidIndex(usize),
    #[error("Incompatible attribute counts")]
    IncompatibleAttributeCounts,
    #[error("Missing required attribute")]
    MissingRequiredAttribute,
    #[error("Invalid face format")]
    InvalidFaceFormat,
}

/// Result type for mesh operations
pub type MeshResult<T> = Result<T, MeshError>;

/// Represents a unique vertex with all its attributes
#[derive(Debug, Clone)]
pub struct Vertex {
    /// Position in 3D space
    pub position: Vec3,
    /// Normal vector (optional)
    pub normal: Option<Vec3>,
    /// Texture coordinates (optional)
    pub uv: Option<Vec2>,
    /// Tangent vector for normal mapping (optional)
    pub tangent: Option<Vec4>,
    /// Vertex color (optional)
    pub color: Option<Vec3>,
}

impl Vertex {
    /// Create a new vertex with just position
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
    pub fn with_normal(position: Vec3, normal: Vec3) -> Self {
        Self {
            position,
            normal: Some(normal),
            uv: None,
            tangent: None,
            color: None,
        }
    }

    /// Create a vertex with position and UV
    pub fn with_uv(position: Vec3, uv: Vec2) -> Self {
        Self {
            position,
            normal: None,
            uv: Some(uv),
            tangent: None,
            color: None,
        }
    }

    /// Create a fully-specified vertex
    pub fn with_all(position: Vec3, normal: Vec3, uv: Vec2) -> Self {
        Self {
            position,
            normal: Some(normal),
            uv: Some(uv),
            tangent: None,
            color: None,
        }
    }
    
    /// Set the normal for this vertex
    pub fn with_normal_added(mut self, normal: Vec3) -> Self {
        self.normal = Some(normal);
        self
    }
    
    /// Set the UV for this vertex
    pub fn with_uv_added(mut self, uv: Vec2) -> Self {
        self.uv = Some(uv);
        self
    }
    
    /// Set the tangent for this vertex
    pub fn with_tangent_added(mut self, tangent: Vec4) -> Self {
        self.tangent = Some(tangent);
        self
    }
    
    /// Set the color for this vertex
    pub fn with_color_added(mut self, color: Vec3) -> Self {
        self.color = Some(color);
        self
    }
}

/// Represents a triangle face using vertex indices
#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    /// Indices into the vertex array
    pub indices: [usize; 3],
}

impl Triangle {
    /// Create a new triangle from three vertex indices
    pub fn new(a: usize, b: usize, c: usize) -> Self {
        Self { indices: [a, b, c] }
    }
}

/// Represents a polygon (face with arbitrary number of vertices)
#[derive(Debug, Clone)]
pub struct Polygon {
    /// Indices into the vertex array
    pub indices: Vec<usize>,
}

impl Polygon {
    /// Create a new polygon from a list of vertex indices
    pub fn new(indices: Vec<usize>) -> Self {
        Self { indices }
    }

    /// Triangulate the polygon into a list of triangles
    /// Uses a simple fan triangulation which only works reliably for convex polygons
    pub fn triangulate(&self) -> Vec<Triangle> {
        if self.indices.len() < 3 {
            return Vec::new();
        }

        let mut triangles = Vec::with_capacity(self.indices.len() - 2);
        let base_index = self.indices[0];

        for i in 1..(self.indices.len() - 1) {
            triangles.push(Triangle::new(
                base_index,
                self.indices[i],
                self.indices[i + 1],
            ));
        }

        triangles
    }
}

/// Represents an edge in the mesh
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Edge {
    /// Vertex indices, always stored with the smaller index first
    pub vertices: [usize; 2],
}

impl Edge {
    /// Create a new edge between two vertices
    pub fn new(a: usize, b: usize) -> Self {
        if a <= b {
            Self { vertices: [a, b] }
        } else {
            Self { vertices: [b, a] }
        }
    }
}

/// Attribute types that can be stored on a mesh
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AttributeType {
    Position,
    Normal,
    TexCoord,
    Tangent,
    Color,
    Custom(u32),
}

/// A mesh containing vertices, faces, and optional attributes
#[derive(Debug, Clone)]
pub struct Mesh {
    /// The vertices in the mesh
    pub vertices: Vec<Vertex>,
    /// The faces in the mesh (triangulated)
    pub triangles: Vec<Triangle>,
    /// Original faces if the mesh was created with polygons
    pub polygons: Option<Vec<Polygon>>,
    /// Custom vertex attributes
    pub custom_attributes: HashMap<String, Vec<Vec3>>,
    /// Material information for this mesh (optional)
    pub material: Option<export::Material>,
    /// Whether the mesh has normals
    has_normals: bool,
    /// Whether the mesh has texture coordinates
    has_uvs: bool,
}

impl Mesh {
    /// Create a new empty mesh
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            triangles: Vec::new(),
            polygons: None,
            custom_attributes: HashMap::new(),
            material: None,
            has_normals: false,
            has_uvs: false,
        }
    }

    /// Create a mesh from a list of vertices and triangles
    pub fn from_triangles(vertices: Vec<Vertex>, triangles: Vec<Triangle>) -> Self {
        let has_normals = vertices.iter().any(|v| v.normal.is_some());
        let has_uvs = vertices.iter().any(|v| v.uv.is_some());
        
        Self {
            vertices,
            triangles,
            polygons: None,
            custom_attributes: HashMap::new(),
            material: None,
            has_normals,
            has_uvs,
        }
    }

    /// Create a mesh from a list of vertices and polygons
    pub fn from_polygons(vertices: Vec<Vertex>, polygons: Vec<Polygon>) -> Self {
        let has_normals = vertices.iter().any(|v| v.normal.is_some());
        let has_uvs = vertices.iter().any(|v| v.uv.is_some());
        
        let mut triangles = Vec::new();
        for polygon in &polygons {
            triangles.extend(polygon.triangulate());
        }
        
        Self {
            vertices,
            triangles,
            polygons: Some(polygons),
            custom_attributes: HashMap::new(),
            material: None,
            has_normals,
            has_uvs,
        }
    }

    /// Add a vertex to the mesh
    pub fn add_vertex(&mut self, vertex: Vertex) -> usize {
        let index = self.vertices.len();
        
        // Update normal and UV flags
        if vertex.normal.is_some() {
            self.has_normals = true;
        }
        if vertex.uv.is_some() {
            self.has_uvs = true;
        }
        
        self.vertices.push(vertex);
        index
    }

    /// Add a triangle to the mesh
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
    pub fn add_polygon(&mut self, indices: Vec<usize>) -> MeshResult<()> {
        let max_index = self.vertices.len().saturating_sub(1);
        if let Some(&idx) = indices.iter().find(|&&idx| idx > max_index) {
            return Err(MeshError::InvalidIndex(idx));
        }
        
        let polygon = Polygon::new(indices);
        
        // Triangulate the polygon
        let triangles = polygon.triangulate();
        for triangle in triangles {
            self.triangles.push(triangle);
        }
        
        // Add to polygons list if it exists
        if let Some(ref mut polygons) = self.polygons {
            polygons.push(polygon);
        } else {
            self.polygons = Some(vec![polygon]);
        }
        
        Ok(())
    }

    /// Check if the mesh has normals
    pub fn has_normals(&self) -> bool {
        self.has_normals
    }

    /// Check if the mesh has texture coordinates
    pub fn has_uvs(&self) -> bool {
        self.has_uvs
    }

    /// Calculate vertex normals for the mesh
    pub fn calculate_normals(&mut self) {
        // Create accumulators for each vertex
        let mut vertex_normals = vec![Vec3::ZERO; self.vertices.len()];
        
        // Accumulate face normals to vertices
        for triangle in &self.triangles {
            if let [a, b, c] = triangle.indices {
                if a >= self.vertices.len() || b >= self.vertices.len() || c >= self.vertices.len() {
                    continue;
                }
                
                let va = self.vertices[a].position;
                let vb = self.vertices[b].position;
                let vc = self.vertices[c].position;
                
                // Calculate face normal
                let normal = (vb - va).cross(vc - va).normalize_or_zero();
                
                // Add to each vertex's accumulated normal
                vertex_normals[a] += normal;
                vertex_normals[b] += normal;
                vertex_normals[c] += normal;
            }
        }
        
        // Normalize and set the vertex normals
        for (i, normal) in vertex_normals.iter().enumerate() {
            let normalized = normal.normalize_or_zero();
            if normalized != Vec3::ZERO {
                self.vertices[i].normal = Some(normalized);
            }
        }
        
        self.has_normals = true;
    }

    /// Get all edges in the mesh
    pub fn get_edges(&self) -> Vec<Edge> {
        let mut edges = std::collections::HashSet::new();
        
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
    pub fn add_custom_attribute(&mut self, name: &str, values: Vec<Vec3>) -> MeshResult<()> {
        if values.len() != self.vertices.len() {
            return Err(MeshError::IncompatibleAttributeCounts);
        }
        
        self.custom_attributes.insert(name.to_string(), values);
        Ok(())
    }
    
    /// Set the material for this mesh
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
#[derive(Debug, Clone)]
pub struct Meshlet {
    /// Local vertices (max 64 typically)
    pub vertices: Vec<usize>,
    /// Local triangles using indices into the local vertices array
    pub triangles: Vec<[u8; 3]>,
    /// Bounding information for culling
    pub bounds_center: Vec3,
    pub bounds_radius: f32,
}

impl Meshlet {
    /// Create a new empty meshlet
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            triangles: Vec::new(),
            bounds_center: Vec3::ZERO,
            bounds_radius: 0.0,
        }
    }
}

/// A collection of meshlets that make up a complete mesh
#[derive(Debug, Clone)]
pub struct MeshletGroup {
    /// The original mesh that the meshlets reference
    pub base_mesh: Mesh,
    /// The meshlets that make up the original mesh
    pub meshlets: Vec<Meshlet>,
}

impl MeshletGroup {
    /// Create meshlets from an existing mesh, useful for GPU-driven rendering
    pub fn from_mesh(mesh: Mesh, max_vertices: usize, max_triangles: usize) -> Self {
        // Simple implementation - in a real system, this would use a more sophisticated
        // algorithm for better cache locality and culling efficiency
        let mut meshlets = Vec::new();
        let mut current_meshlet = Meshlet::new();
        let mut vertex_remap = HashMap::new();
        
        for (triangle_idx, triangle) in mesh.triangles.iter().enumerate() {
            // Check if adding this triangle would exceed our limits
            if current_meshlet.triangles.len() >= max_triangles || 
               vertex_remap.len() + 3 > max_vertices {
                // Finish the current meshlet and start a new one
                if !current_meshlet.vertices.is_empty() {
                    // Calculate bounds
                    let mut center = Vec3::ZERO;
                    for &vi in &current_meshlet.vertices {
                        center += mesh.vertices[vi].position;
                    }
                    center /= current_meshlet.vertices.len() as f32;
                    
                    let mut radius = 0.0f32;
                    for &vi in &current_meshlet.vertices {
                        let dist = (mesh.vertices[vi].position - center).length();
                        radius = radius.max(dist);
                    }
                    
                    current_meshlet.bounds_center = center;
                    current_meshlet.bounds_radius = radius;
                    
                    meshlets.push(current_meshlet);
                }
                
                // Start a new meshlet
                current_meshlet = Meshlet::new();
                vertex_remap = HashMap::new();
            }
            
            // Add this triangle to the current meshlet
            let mut local_indices = [0u8; 3];
            for (i, &global_idx) in triangle.indices.iter().enumerate() {
                let local_idx = if let Some(&idx) = vertex_remap.get(&global_idx) {
                    idx
                } else {
                    let new_idx = current_meshlet.vertices.len();
                    current_meshlet.vertices.push(global_idx);
                    vertex_remap.insert(global_idx, new_idx);
                    new_idx
                };
                
                local_indices[i] = local_idx as u8;
            }
            
            current_meshlet.triangles.push(local_indices);
        }
        
        // Add the last meshlet if it has any triangles
        if !current_meshlet.triangles.is_empty() {
            // Calculate bounds
            let mut center = Vec3::ZERO;
            for &vi in &current_meshlet.vertices {
                center += mesh.vertices[vi].position;
            }
            center /= current_meshlet.vertices.len() as f32;
            
            let mut radius = 0.0f32;
            for &vi in &current_meshlet.vertices {
                let dist = (mesh.vertices[vi].position - center).length();
                radius = radius.max(dist);
            }
            
            current_meshlet.bounds_center = center;
            current_meshlet.bounds_radius = radius;
            
            meshlets.push(current_meshlet);
        }
        
        Self {
            base_mesh: mesh,
            meshlets,
        }
    }
}

/// A scene container for multiple meshes with different materials
#[derive(Debug, Clone)]
pub struct Scene {
    /// Meshes in the scene
    pub meshes: Vec<Mesh>,
    /// Name of the scene
    pub name: String,
}

impl Scene {
    /// Create a new empty scene
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            meshes: Vec::new(),
            name: name.into(),
        }
    }
    
    /// Add a mesh to the scene
    pub fn add_mesh(&mut self, mesh: Mesh) -> &mut Self {
        self.meshes.push(mesh);
        self
    }
    
    /// Create a scene with a single mesh
    pub fn from_mesh(mesh: Mesh, name: impl Into<String>) -> Self {
        let mut scene = Self::new(name);
        scene.add_mesh(mesh);
        scene
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_simple_mesh() {
        let mut mesh = Mesh::new();
        
        // Add vertices
        let v0 = mesh.add_vertex(Vertex::new(Vec3::new(0.0, 0.0, 0.0)));
        let v1 = mesh.add_vertex(Vertex::new(Vec3::new(1.0, 0.0, 0.0)));
        let v2 = mesh.add_vertex(Vertex::new(Vec3::new(0.0, 1.0, 0.0)));
        
        // Add a face
        let result = mesh.add_triangle(v0, v1, v2);
        assert!(result.is_ok());
        
        // Check mesh properties
        assert_eq!(mesh.vertices.len(), 3);
        assert_eq!(mesh.triangles.len(), 1);
        assert!(!mesh.has_normals());
        assert!(!mesh.has_uvs());
    }

    #[test]
    fn create_mesh_with_attributes() {
        let mut mesh = Mesh::new();
        
        // Add vertices with attributes
        let v0 = mesh.add_vertex(Vertex::with_all(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec2::new(0.0, 0.0),
        ));
        
        let v1 = mesh.add_vertex(Vertex::with_all(
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec2::new(1.0, 0.0),
        ));
        
        let v2 = mesh.add_vertex(Vertex::with_all(
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec2::new(0.0, 1.0),
        ));
        
        // Add a face
        let result = mesh.add_triangle(v0, v1, v2);
        assert!(result.is_ok());
        
        // Check mesh properties
        assert_eq!(mesh.vertices.len(), 3);
        assert_eq!(mesh.triangles.len(), 1);
        assert!(mesh.has_normals());
        assert!(mesh.has_uvs());
    }

    #[test]
    fn calculate_normals() {
        let mut mesh = Mesh::new();
        
        // Add vertices without normals
        let v0 = mesh.add_vertex(Vertex::new(Vec3::new(0.0, 0.0, 0.0)));
        let v1 = mesh.add_vertex(Vertex::new(Vec3::new(1.0, 0.0, 0.0)));
        let v2 = mesh.add_vertex(Vertex::new(Vec3::new(0.0, 1.0, 0.0)));
        
        // Add a face
        let _ = mesh.add_triangle(v0, v1, v2);
        
        // Calculate normals
        mesh.calculate_normals();
        
        // Check that normals were calculated
        assert!(mesh.has_normals());
        assert!(mesh.vertices[0].normal.is_some());
        assert!(mesh.vertices[1].normal.is_some());
        assert!(mesh.vertices[2].normal.is_some());
        
        // The normal should be (0, 0, 1) for this triangle
        let expected = Vec3::new(0.0, 0.0, 1.0);
        assert!((mesh.vertices[0].normal.unwrap() - expected).length() < 1e-5);
    }
}
