//! Core mesh data structures for mesh-tools
//!
//! This module provides the primary data structures for representing 3D meshes:
//! - Vertex: Stores position and optional attributes (normal, UV, tangent, color)
//! - Triangle: Represents a face with three vertex indices
//! - Edge: Represents a connection between two vertices
//! - Mesh: Container for vertices and triangles

use glam::{Vec2, Vec3, Vec4};
use std::collections::HashMap;
use thiserror::Error;

/// Error type for mesh operations
#[derive(Error, Debug)]
pub enum MeshError {
    /// The mesh has no vertices
    #[error("Mesh has no vertices")]
    NoVertices,
    
    /// The mesh has no triangles
    #[error("Mesh has no triangles")]
    NoTriangles,
    
    /// Invalid vertex index (out of bounds)
    #[error("Invalid vertex index {0}, mesh has {1} vertices")]
    InvalidVertexIndex(u32, usize),
    
    /// Invalid triangle index (out of bounds)
    #[error("Invalid triangle index {0}, mesh has {1} triangles")]
    InvalidTriangleIndex(u32, usize),
    
    /// Operation requires normals but mesh has none
    #[error("Operation requires normals but mesh has none")]
    MissingNormals,
    
    /// Operation requires UVs but mesh has none
    #[error("Operation requires UVs but mesh has none")]
    MissingUVs,
    
    /// Operation requires tangents but mesh has none
    #[error("Operation requires tangents but mesh has none")]
    MissingTangents,
    
    /// Generic mesh error with custom message
    #[error("{0}")]
    Generic(String),
}

/// A vertex in a 3D mesh
///
/// A vertex consists of a position (required) and optional attributes:
/// - normal: Surface normal direction
/// - uv: Texture coordinates
/// - tangent: Tangent vector for normal mapping (Vec4 where w is handedness)
/// - color: Vertex color (RGBA)
#[derive(Debug, Clone)]
pub struct Vertex {
    /// Position of the vertex (required)
    pub position: Vec3,
    /// Surface normal (optional)
    pub normal: Option<Vec3>,
    /// Texture coordinates (optional)
    pub uv: Option<Vec2>,
    /// Tangent vector with handedness in w component (optional)
    pub tangent: Option<Vec4>,
    /// Vertex color (optional)
    pub color: Option<Vec4>,
}

impl Vertex {
    /// Create a new vertex with just a position
    pub fn new(position: Vec3) -> Self {
        Self {
            position,
            normal: None,
            uv: None,
            tangent: None,
            color: None,
        }
    }
    
    /// Create a new vertex with position and normal
    pub fn with_normal(position: Vec3, normal: Vec3) -> Self {
        Self {
            position,
            normal: Some(normal),
            uv: None,
            tangent: None,
            color: None,
        }
    }
    
    /// Create a new vertex with position, normal, and UV
    pub fn with_normal_uv(position: Vec3, normal: Vec3, uv: Vec2) -> Self {
        Self {
            position,
            normal: Some(normal),
            uv: Some(uv),
            tangent: None,
            color: None,
        }
    }
    
    /// Set the normal for this vertex
    pub fn with_normal_mut(&mut self, normal: Vec3) -> &mut Self {
        self.normal = Some(normal);
        self
    }
    
    /// Set the UV coordinates for this vertex
    pub fn with_uv_mut(&mut self, uv: Vec2) -> &mut Self {
        self.uv = Some(uv);
        self
    }
    
    /// Set the tangent for this vertex
    pub fn with_tangent_mut(&mut self, tangent: Vec4) -> &mut Self {
        self.tangent = Some(tangent);
        self
    }
    
    /// Set the color for this vertex
    pub fn with_color_mut(&mut self, color: Vec4) -> &mut Self {
        self.color = Some(color);
        self
    }
}

/// A triangle in a mesh, defined by three vertex indices
#[derive(Debug, Clone, Copy)]
pub struct Triangle(pub u32, pub u32, pub u32);

impl Triangle {
    /// Create a new triangle from three vertex indices
    pub fn new(a: u32, b: u32, c: u32) -> Self {
        Self(a, b, c)
    }
    
    /// Get the indices as an array
    pub fn indices(&self) -> [u32; 3] {
        [self.0, self.1, self.2]
    }
}

/// An edge in a mesh, defined by two vertex indices
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Edge(pub u32, pub u32);

impl Edge {
    /// Create a new edge from two vertex indices
    ///
    /// The indices are automatically sorted so that Edge(a,b) == Edge(b,a)
    pub fn new(a: u32, b: u32) -> Self {
        if a <= b {
            Self(a, b)
        } else {
            Self(b, a)
        }
    }
    
    /// Get the indices as an array
    pub fn indices(&self) -> [u32; 2] {
        [self.0, self.1]
    }
}

/// A mesh consisting of vertices and triangles
#[derive(Debug, Clone)]
pub struct Mesh {
    /// Vertices of the mesh
    pub vertices: Vec<Vertex>,
    /// Triangles of the mesh
    pub triangles: Vec<Triangle>,
    /// Optional name for the mesh
    pub name: Option<String>,
}

impl Mesh {
    /// Create a new empty mesh
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            triangles: Vec::new(),
            name: None,
        }
    }
    
    /// Create a new mesh with a name
    pub fn with_name(name: &str) -> Self {
        Self {
            vertices: Vec::new(),
            triangles: Vec::new(),
            name: Some(name.to_string()),
        }
    }
    
    /// Create a new mesh with pre-allocated capacity
    pub fn with_capacity(vertex_capacity: usize, triangle_capacity: usize) -> Self {
        Self {
            vertices: Vec::with_capacity(vertex_capacity),
            triangles: Vec::with_capacity(triangle_capacity),
            name: None,
        }
    }
    
    /// Get the number of vertices in the mesh
    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }
    
    /// Get the number of triangles in the mesh
    pub fn triangle_count(&self) -> usize {
        self.triangles.len()
    }
    
    /// Add a vertex to the mesh and return its index
    pub fn add_vertex(&mut self, vertex: Vertex) -> u32 {
        let index = self.vertices.len() as u32;
        self.vertices.push(vertex);
        index
    }
    
    /// Add a triangle to the mesh
    ///
    /// Returns an error if any vertex index is out of bounds
    pub fn add_triangle(&mut self, triangle: Triangle) -> Result<(), MeshError> {
        let vertex_count = self.vertices.len() as u32;
        
        if triangle.0 >= vertex_count {
            return Err(MeshError::InvalidVertexIndex(triangle.0, self.vertices.len()));
        }
        if triangle.1 >= vertex_count {
            return Err(MeshError::InvalidVertexIndex(triangle.1, self.vertices.len()));
        }
        if triangle.2 >= vertex_count {
            return Err(MeshError::InvalidVertexIndex(triangle.2, self.vertices.len()));
        }
        
        self.triangles.push(triangle);
        Ok(())
    }
    
    /// Get all edges in the mesh
    pub fn edges(&self) -> Vec<Edge> {
        let mut edges = HashMap::new();
        
        for triangle in &self.triangles {
            let e1 = Edge::new(triangle.0, triangle.1);
            let e2 = Edge::new(triangle.1, triangle.2);
            let e3 = Edge::new(triangle.2, triangle.0);
            
            edges.insert(e1, e1);
            edges.insert(e2, e2);
            edges.insert(e3, e3);
        }
        
        edges.values().cloned().collect()
    }
    
    /// Calculate normals for the mesh
    ///
    /// This computes smooth (averaged) normals for each vertex
    pub fn calculate_normals(&mut self) {
        // Initialize all normals to zero
        for vertex in &mut self.vertices {
            vertex.normal = Some(Vec3::ZERO);
        }
        
        // Accumulate face normals
        for triangle in &self.triangles {
            let v0 = self.vertices[triangle.0 as usize].position;
            let v1 = self.vertices[triangle.1 as usize].position;
            let v2 = self.vertices[triangle.2 as usize].position;
            
            let edge1 = v1 - v0;
            let edge2 = v2 - v0;
            let normal = edge1.cross(edge2);
            
            // Add to each vertex normal
            if let Some(ref mut n) = self.vertices[triangle.0 as usize].normal {
                *n += normal;
            }
            if let Some(ref mut n) = self.vertices[triangle.1 as usize].normal {
                *n += normal;
            }
            if let Some(ref mut n) = self.vertices[triangle.2 as usize].normal {
                *n += normal;
            }
        }
        
        // Normalize all normals
        for vertex in &mut self.vertices {
            if let Some(ref mut normal) = vertex.normal {
                if normal.length_squared() > 0.0 {
                    *normal = normal.normalize();
                } else {
                    *normal = Vec3::new(0.0, 1.0, 0.0); // Default to up if zero
                }
            }
        }
    }
    
    /// Calculate tangents for the mesh
    ///
    /// This requires UVs to be present on all vertices
    pub fn calculate_tangents(&mut self) -> Result<(), MeshError> {
        // Check if all vertices have UVs
        if !self.vertices.iter().all(|v| v.uv.is_some()) {
            return Err(MeshError::MissingUVs);
        }
        
        // Ensure normals are calculated
        if !self.vertices.iter().all(|v| v.normal.is_some()) {
            self.calculate_normals();
        }
        
        // Initialize tangent and bitangent accumulators
        let mut tangents = vec![Vec3::ZERO; self.vertices.len()];
        let mut bitangents = vec![Vec3::ZERO; self.vertices.len()];
        
        // Calculate tangent space for each triangle
        for triangle in &self.triangles {
            let i0 = triangle.0 as usize;
            let i1 = triangle.1 as usize;
            let i2 = triangle.2 as usize;
            
            let v0 = self.vertices[i0].position;
            let v1 = self.vertices[i1].position;
            let v2 = self.vertices[i2].position;
            
            let uv0 = self.vertices[i0].uv.unwrap();
            let uv1 = self.vertices[i1].uv.unwrap();
            let uv2 = self.vertices[i2].uv.unwrap();
            
            let delta_pos1 = v1 - v0;
            let delta_pos2 = v2 - v0;
            
            let delta_uv1 = uv1 - uv0;
            let delta_uv2 = uv2 - uv0;
            
            // Calculate tangent and bitangent
            let r = 1.0 / (delta_uv1.x * delta_uv2.y - delta_uv1.y * delta_uv2.x);
            
            let tangent = (delta_pos1 * delta_uv2.y - delta_pos2 * delta_uv1.y) * r;
            let bitangent = (delta_pos2 * delta_uv1.x - delta_pos1 * delta_uv2.x) * r;
            
            // Accumulate
            tangents[i0] += tangent;
            tangents[i1] += tangent;
            tangents[i2] += tangent;
            
            bitangents[i0] += bitangent;
            bitangents[i1] += bitangent;
            bitangents[i2] += bitangent;
        }
        
        // Calculate final tangent (with correct handedness)
        for i in 0..self.vertices.len() {
            let normal = self.vertices[i].normal.unwrap();
            let t = tangents[i];
            let b = bitangents[i];
            
            // Gram-Schmidt orthogonalization
            let tangent = (t - normal * normal.dot(t)).normalize();
            
            // Calculate handedness (w component)
            let w = if normal.cross(tangent).dot(b) < 0.0 { -1.0 } else { 1.0 };
            
            // Store tangent with handedness
            self.vertices[i].tangent = Some(Vec4::new(tangent.x, tangent.y, tangent.z, w));
        }
        
        Ok(())
    }
    
    /// Get the bounding box of the mesh
    pub fn bounding_box(&self) -> (Vec3, Vec3) {
        if self.vertices.is_empty() {
            return (Vec3::ZERO, Vec3::ZERO);
        }
        
        let mut min = Vec3::splat(f32::MAX);
        let mut max = Vec3::splat(f32::MIN);
        
        for vertex in &self.vertices {
            min = min.min(vertex.position);
            max = max.max(vertex.position);
        }
        
        (min, max)
    }
    
    /// Get the center of the mesh
    pub fn center(&self) -> Vec3 {
        let (min, max) = self.bounding_box();
        (min + max) * 0.5
    }
    
    /// Create a transformed copy of this mesh
    pub fn transformed(&self, transform: &glam::Mat4) -> Self {
        let mut result = self.clone();
        result.transform(transform);
        result
    }
    
    /// Transform this mesh in place
    pub fn transform(&mut self, transform: &glam::Mat4) {
        // Get the normal matrix (inverse transpose of the 3x3 portion of the transform)
        let normal_matrix = transform.inverse().transpose().mat3();
        
        for vertex in &mut self.vertices {
            // Transform position
            vertex.position = transform.transform_point3(vertex.position);
            
            // Transform normal
            if let Some(ref mut normal) = vertex.normal {
                *normal = normal_matrix.transform_vector3(*normal).normalize();
            }
            
            // Transform tangent
            if let Some(ref mut tangent) = vertex.tangent {
                let t = normal_matrix.transform_vector3(Vec3::new(tangent.x, tangent.y, tangent.z)).normalize();
                tangent.x = t.x;
                tangent.y = t.y;
                tangent.z = t.z;
                // Note: w component (handedness) doesn't change
            }
        }
    }
}

impl Default for Mesh {
    fn default() -> Self {
        Self::new()
    }
}