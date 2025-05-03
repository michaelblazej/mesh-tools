
use glam::{Vec2, Vec3, Vec4};
use std::collections::HashMap;
use thiserror::Error;

pub mod primitives;
pub mod modifiers;
pub mod export;

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

pub type MeshResult<T> = Result<T, MeshError>;

#[derive(Debug, Clone)]
pub struct Vertex {
    pub position: Vec3,
    pub normal: Option<Vec3>,
    pub uv: Option<Vec2>,
    pub tangent: Option<Vec4>,
    pub color: Option<Vec3>,
}

impl Vertex {
    pub fn new(position: Vec3) -> Self {
        Self {
            position,
            normal: None,
            uv: None,
            tangent: None,
            color: None,
        }
    }

    pub fn with_normal(position: Vec3, normal: Vec3) -> Self {
        Self {
            position,
            normal: Some(normal),
            uv: None,
            tangent: None,
            color: None,
        }
    }

    pub fn with_uv(position: Vec3, uv: Vec2) -> Self {
        Self {
            position,
            normal: None,
            uv: Some(uv),
            tangent: None,
            color: None,
        }
    }

    pub fn with_all(position: Vec3, normal: Vec3, uv: Vec2) -> Self {
        Self {
            position,
            normal: Some(normal),
            uv: Some(uv),
            tangent: None,
            color: None,
        }
    }

    pub fn with_normal_added(mut self, normal: Vec3) -> Self {
        self.normal = Some(normal);
        self
    }

    pub fn with_uv_added(mut self, uv: Vec2) -> Self {
        self.uv = Some(uv);
        self
    }

    pub fn with_tangent_added(mut self, tangent: Vec4) -> Self {
        self.tangent = Some(tangent);
        self
    }

    pub fn with_color_added(mut self, color: Vec3) -> Self {
        self.color = Some(color);
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub indices: [usize; 3],
}

impl Triangle {
    pub fn new(a: usize, b: usize, c: usize) -> Self {
        Self { indices: [a, b, c] }
    }
}

#[derive(Debug, Clone)]
pub struct Polygon {
    pub indices: Vec<usize>,
}

impl Polygon {
    pub fn new(indices: Vec<usize>) -> Self {
        Self { indices }
    }

    pub fn triangulate(&self) -> Vec<Triangle> {
        if self.indices.len() < 3 {
            return Vec::new();
        }

        let mut triangles = Vec::new();

        
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Edge {
    pub vertices: [usize; 2],
}

impl Edge {
    pub fn new(a: usize, b: usize) -> Self {
        if a < b {
            Self { vertices: [a, b] }
        } else {
            Self { vertices: [b, a] }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AttributeType {
    Position,
    Normal,
    UV,
    Tangent,
    Color,
    Custom(String),
}

#[derive(Debug, Clone)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub triangles: Vec<Triangle>,
    pub custom_attributes: HashMap<String, Vec<Vec3>>,
    pub material: Option<export::Material>,
}

impl Mesh {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            triangles: Vec::new(),
            custom_attributes: HashMap::new(),
            material: None,
        }
    }

    pub fn from_triangles(vertices: Vec<Vertex>, triangles: Vec<Triangle>) -> Self {
        Self {
            vertices,
            triangles,
            custom_attributes: HashMap::new(),
            material: None,
        }
    }

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

    pub fn add_vertex(&mut self, vertex: Vertex) -> usize {
        let index = self.vertices.len();
        self.vertices.push(vertex);
        index
    }

    pub fn add_triangle(&mut self, a: usize, b: usize, c: usize) -> MeshResult<usize> {
        let max_index = self.vertices.len().saturating_sub(1);
        if a > max_index || b > max_index || c > max_index {
            return Err(MeshError::InvalidIndex(std::cmp::max(a, std::cmp::max(b, c))));
        }
        let index = self.triangles.len();
        self.triangles.push(Triangle::new(a, b, c));
        Ok(index)
    }

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

    pub fn has_normals(&self) -> bool {
        self.vertices.iter().all(|v| v.normal.is_some())
    }

    pub fn has_uvs(&self) -> bool {
        self.vertices.iter().all(|v| v.uv.is_some())
    }

    pub fn calculate_normals(&mut self) {
        
        let mut normal_sums = vec![Vec3::ZERO; self.vertices.len()];
        
        
        for triangle in &self.triangles {
            if let [a, b, c] = triangle.indices {
                let va = self.vertices[a].position;
                let vb = self.vertices[b].position;
                let vc = self.vertices[c].position;
                
                
                let edge1 = vb - va;
                let edge2 = vc - va;
                let face_normal = edge1.cross(edge2).normalize();
                
                
                normal_sums[a] += face_normal;
                normal_sums[b] += face_normal;
                normal_sums[c] += face_normal;
            }
        }
        
        
        for (i, sum) in normal_sums.iter().enumerate() {
            if sum.length_squared() > 0.0 {
                self.vertices[i].normal = Some(sum.normalize());
            } else {
                
                self.vertices[i].normal = Some(Vec3::new(0.0, 1.0, 0.0));
            }
        }
    }

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

    pub fn add_custom_attribute(&mut self, name: &str, values: Vec<Vec3>) -> MeshResult<()> {
        if values.len() != self.vertices.len() {
            return Err(MeshError::IncompatibleAttributeCounts);
        }
        
        self.custom_attributes.insert(name.to_string(), values);
        Ok(())
    }

    pub fn with_material(&mut self, material: export::Material) -> &mut Self {
        self.material = Some(material);
        self
    }

    pub fn get_material(&self) -> Option<&export::Material> {
        self.material.as_ref()
    }
}

#[derive(Debug, Clone)]
pub struct Meshlet {
    pub vertices: Vec<usize>,
    pub triangles: Vec<[u8; 3]>,
    pub center: Vec3,
    pub radius: f32,
}

impl Meshlet {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            triangles: Vec::new(),
            center: Vec3::ZERO,
            radius: 0.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MeshletGroup {
    pub vertices: Vec<Vertex>,
    pub meshlets: Vec<Meshlet>,
}

impl MeshletGroup {
    pub fn from_mesh(mesh: Mesh, max_vertices: usize, max_triangles: usize) -> Self {
        let mut meshlets = Vec::new();
        let mut current_meshlet = Meshlet::new();
        
        let mut vertex_remap = HashMap::new();
        
        for triangle in &mesh.triangles {
            
            if current_meshlet.triangles.len() >= max_triangles ||
               current_meshlet.vertices.len() + 3 > max_vertices {
                
                if !current_meshlet.triangles.is_empty() {
                    
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
            
            
            if !all_vertices_mapped {
                for i in 0..3 {
                    let global_idx = triangle.indices[i];
                    
                    if !vertex_remap.contains_key(&global_idx) {
                        let local_idx = current_meshlet.vertices.len();
                        if local_idx >= 255 {
                            
                            
                            
                            
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
                            
                            
                            break;
                        }
                        
                        current_meshlet.vertices.push(global_idx);
                        vertex_remap.insert(global_idx, local_idx as u8);
                        local_indices[i] = local_idx as u8;
                    }
                }
                
                
                if vertex_remap.contains_key(&triangle.indices[0]) &&
                   vertex_remap.contains_key(&triangle.indices[1]) &&
                   vertex_remap.contains_key(&triangle.indices[2]) {
                    current_meshlet.triangles.push(local_indices);
                }
            } else {
                
                current_meshlet.triangles.push(local_indices);
            }
        }
        
        
        if !current_meshlet.triangles.is_empty() {
            
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

#[derive(Debug, Clone)]
pub struct Scene {
    pub name: String,
    pub meshes: Vec<Mesh>,
}

impl Scene {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            meshes: Vec::new(),
        }
    }

    pub fn add_mesh(&mut self, mesh: Mesh) -> &mut Self {
        self.meshes.push(mesh);
        self
    }

    pub fn from_mesh(mesh: Mesh, name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            meshes: vec![mesh],
        }
    }
}
