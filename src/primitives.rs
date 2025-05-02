//! Utilities for creating primitive mesh shapes
//! 
//! This module provides functions to generate common primitive 3D shapes like
//! cubes, spheres, cylinders, planes, cones, torus, and more.

use crate::{Mesh, Vertex, Triangle, MeshResult};
use glam::{Vec2, Vec3};
use std::f32::consts::PI;

/// Creates a cube mesh centered at the origin with the given dimensions
pub fn create_cube(width: f32, height: f32, depth: f32) -> Mesh {
    let half_width = width / 2.0;
    let half_height = height / 2.0;
    let half_depth = depth / 2.0;
    
    let mut mesh = Mesh::new();
    
    // Create vertices for all 8 corners of the cube
    // Front face (positive z)
    let v0 = mesh.add_vertex(Vertex::with_all(
        Vec3::new(-half_width, -half_height, half_depth),
        Vec3::new(0.0, 0.0, 1.0),
        Vec2::new(0.0, 0.0),
    ));
    let v1 = mesh.add_vertex(Vertex::with_all(
        Vec3::new(half_width, -half_height, half_depth),
        Vec3::new(0.0, 0.0, 1.0),
        Vec2::new(1.0, 0.0),
    ));
    let v2 = mesh.add_vertex(Vertex::with_all(
        Vec3::new(half_width, half_height, half_depth),
        Vec3::new(0.0, 0.0, 1.0),
        Vec2::new(1.0, 1.0),
    ));
    let v3 = mesh.add_vertex(Vertex::with_all(
        Vec3::new(-half_width, half_height, half_depth),
        Vec3::new(0.0, 0.0, 1.0),
        Vec2::new(0.0, 1.0),
    ));
    
    // Back face (negative z)
    let v4 = mesh.add_vertex(Vertex::with_all(
        Vec3::new(-half_width, -half_height, -half_depth),
        Vec3::new(0.0, 0.0, -1.0),
        Vec2::new(1.0, 0.0),
    ));
    let v5 = mesh.add_vertex(Vertex::with_all(
        Vec3::new(half_width, -half_height, -half_depth),
        Vec3::new(0.0, 0.0, -1.0),
        Vec2::new(0.0, 0.0),
    ));
    let v6 = mesh.add_vertex(Vertex::with_all(
        Vec3::new(half_width, half_height, -half_depth),
        Vec3::new(0.0, 0.0, -1.0),
        Vec2::new(0.0, 1.0),
    ));
    let v7 = mesh.add_vertex(Vertex::with_all(
        Vec3::new(-half_width, half_height, -half_depth),
        Vec3::new(0.0, 0.0, -1.0),
        Vec2::new(1.0, 1.0),
    ));
    
    // Top face (positive y)
    let v8 = mesh.add_vertex(Vertex::with_all(
        Vec3::new(-half_width, half_height, -half_depth),
        Vec3::new(0.0, 1.0, 0.0),
        Vec2::new(0.0, 0.0),
    ));
    let v9 = mesh.add_vertex(Vertex::with_all(
        Vec3::new(half_width, half_height, -half_depth),
        Vec3::new(0.0, 1.0, 0.0),
        Vec2::new(1.0, 0.0),
    ));
    let v10 = mesh.add_vertex(Vertex::with_all(
        Vec3::new(half_width, half_height, half_depth),
        Vec3::new(0.0, 1.0, 0.0),
        Vec2::new(1.0, 1.0),
    ));
    let v11 = mesh.add_vertex(Vertex::with_all(
        Vec3::new(-half_width, half_height, half_depth),
        Vec3::new(0.0, 1.0, 0.0),
        Vec2::new(0.0, 1.0),
    ));
    
    // Bottom face (negative y)
    let v12 = mesh.add_vertex(Vertex::with_all(
        Vec3::new(-half_width, -half_height, -half_depth),
        Vec3::new(0.0, -1.0, 0.0),
        Vec2::new(0.0, 1.0),
    ));
    let v13 = mesh.add_vertex(Vertex::with_all(
        Vec3::new(half_width, -half_height, -half_depth),
        Vec3::new(0.0, -1.0, 0.0),
        Vec2::new(1.0, 1.0),
    ));
    let v14 = mesh.add_vertex(Vertex::with_all(
        Vec3::new(half_width, -half_height, half_depth),
        Vec3::new(0.0, -1.0, 0.0),
        Vec2::new(1.0, 0.0),
    ));
    let v15 = mesh.add_vertex(Vertex::with_all(
        Vec3::new(-half_width, -half_height, half_depth),
        Vec3::new(0.0, -1.0, 0.0),
        Vec2::new(0.0, 0.0),
    ));
    
    // Right face (positive x)
    let v16 = mesh.add_vertex(Vertex::with_all(
        Vec3::new(half_width, -half_height, -half_depth),
        Vec3::new(1.0, 0.0, 0.0),
        Vec2::new(0.0, 0.0),
    ));
    let v17 = mesh.add_vertex(Vertex::with_all(
        Vec3::new(half_width, half_height, -half_depth),
        Vec3::new(1.0, 0.0, 0.0),
        Vec2::new(0.0, 1.0),
    ));
    let v18 = mesh.add_vertex(Vertex::with_all(
        Vec3::new(half_width, half_height, half_depth),
        Vec3::new(1.0, 0.0, 0.0),
        Vec2::new(1.0, 1.0),
    ));
    let v19 = mesh.add_vertex(Vertex::with_all(
        Vec3::new(half_width, -half_height, half_depth),
        Vec3::new(1.0, 0.0, 0.0),
        Vec2::new(1.0, 0.0),
    ));
    
    // Left face (negative x)
    let v20 = mesh.add_vertex(Vertex::with_all(
        Vec3::new(-half_width, -half_height, -half_depth),
        Vec3::new(-1.0, 0.0, 0.0),
        Vec2::new(1.0, 0.0),
    ));
    let v21 = mesh.add_vertex(Vertex::with_all(
        Vec3::new(-half_width, half_height, -half_depth),
        Vec3::new(-1.0, 0.0, 0.0),
        Vec2::new(1.0, 1.0),
    ));
    let v22 = mesh.add_vertex(Vertex::with_all(
        Vec3::new(-half_width, half_height, half_depth),
        Vec3::new(-1.0, 0.0, 0.0),
        Vec2::new(0.0, 1.0),
    ));
    let v23 = mesh.add_vertex(Vertex::with_all(
        Vec3::new(-half_width, -half_height, half_depth),
        Vec3::new(-1.0, 0.0, 0.0),
        Vec2::new(0.0, 0.0),
    ));
    
    // Add triangles for each face (2 triangles per face)
    // Front face
    let _ = mesh.add_triangle(v0, v1, v2);
    let _ = mesh.add_triangle(v0, v2, v3);
    
    // Back face
    let _ = mesh.add_triangle(v4, v6, v5);
    let _ = mesh.add_triangle(v4, v7, v6);
    
    // Top face
    let _ = mesh.add_triangle(v8, v9, v10);
    let _ = mesh.add_triangle(v8, v10, v11);
    
    // Bottom face
    let _ = mesh.add_triangle(v12, v14, v13);
    let _ = mesh.add_triangle(v12, v15, v14);
    
    // Right face
    let _ = mesh.add_triangle(v16, v17, v18);
    let _ = mesh.add_triangle(v16, v18, v19);
    
    // Left face
    let _ = mesh.add_triangle(v20, v22, v21);
    let _ = mesh.add_triangle(v20, v23, v22);
    
    mesh
}

/// Creates a plane mesh on the XZ plane with the given dimensions
pub fn create_plane(width: f32, depth: f32, width_segments: u32, depth_segments: u32) -> Mesh {
    let mut mesh = Mesh::new();
    
    let half_width = width / 2.0;
    let half_depth = depth / 2.0;
    
    let width_segment_size = width / width_segments as f32;
    let depth_segment_size = depth / depth_segments as f32;
    
    // Create a grid of vertices
    let mut vertex_grid = Vec::with_capacity((width_segments + 1) as usize * (depth_segments + 1) as usize);
    
    for z in 0..=depth_segments {
        for x in 0..=width_segments {
            let x_pos = -half_width + x as f32 * width_segment_size;
            let z_pos = -half_depth + z as f32 * depth_segment_size;
            
            let u = x as f32 / width_segments as f32;
            let v = z as f32 / depth_segments as f32;
            
            let vertex_idx = mesh.add_vertex(Vertex::with_all(
                Vec3::new(x_pos, 0.0, z_pos),
                Vec3::new(0.0, 1.0, 0.0),
                Vec2::new(u, v),
            ));
            
            vertex_grid.push(vertex_idx);
        }
    }
    
    // Create triangles
    for z in 0..depth_segments {
        for x in 0..width_segments {
            let stride = width_segments + 1;
            let idx = z * stride + x;
            
            let v0 = vertex_grid[idx as usize];
            let v1 = vertex_grid[(idx + 1) as usize];
            let v2 = vertex_grid[(idx + stride + 1) as usize];
            let v3 = vertex_grid[(idx + stride) as usize];
            
            // Add two triangles to form a quad
            let _ = mesh.add_triangle(v0, v1, v2);
            let _ = mesh.add_triangle(v0, v2, v3);
        }
    }
    
    mesh
}

/// Creates a sphere mesh using UV-sphere approach
pub fn create_sphere(radius: f32, segments: u32, rings: u32) -> Mesh {
    let mut mesh = Mesh::new();
    
    // Create vertices
    // First, top and bottom vertices (poles)
    let top_idx = mesh.add_vertex(Vertex::with_all(
        Vec3::new(0.0, radius, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec2::new(0.5, 0.0),
    ));
    
    let bottom_idx = mesh.add_vertex(Vertex::with_all(
        Vec3::new(0.0, -radius, 0.0),
        Vec3::new(0.0, -1.0, 0.0),
        Vec2::new(0.5, 1.0),
    ));
    
    // Create rings of vertices
    let mut ring_vertices = Vec::with_capacity((rings - 1) as usize * segments as usize);
    
    for ring in 1..rings {
        let phi = PI * ring as f32 / rings as f32;
        let y = radius * (-phi.cos());
        let ring_radius = radius * phi.sin();
        
        for segment in 0..segments {
            let theta = 2.0 * PI * segment as f32 / segments as f32;
            let x = ring_radius * theta.cos();
            let z = ring_radius * theta.sin();
            
            let normal = Vec3::new(x, y, z).normalize();
            let u = segment as f32 / segments as f32;
            let v = ring as f32 / rings as f32;
            
            let idx = mesh.add_vertex(Vertex::with_all(
                Vec3::new(x, y, z),
                normal,
                Vec2::new(u, v),
            ));
            
            ring_vertices.push(idx);
        }
    }
    
    // Create triangles
    // Top cap
    for segment in 0..segments {
        let next_segment = (segment + 1) % segments;
        let v0 = top_idx;
        let v1 = ring_vertices[segment as usize];
        let v2 = ring_vertices[next_segment as usize];
        
        let _ = mesh.add_triangle(v0, v1, v2);
    }
    
    // Middle rings
    for ring in 0..(rings - 2) {
        let ring_start = ring * segments;
        let next_ring_start = (ring + 1) * segments;
        
        for segment in 0..segments {
            let next_segment = (segment + 1) % segments;
            
            let v0 = ring_vertices[(ring_start + segment) as usize];
            let v1 = ring_vertices[(ring_start + next_segment) as usize];
            let v2 = ring_vertices[(next_ring_start + next_segment) as usize];
            let v3 = ring_vertices[(next_ring_start + segment) as usize];
            
            let _ = mesh.add_triangle(v0, v1, v2);
            let _ = mesh.add_triangle(v0, v2, v3);
        }
    }
    
    // Bottom cap
    let last_ring_start = (rings - 2) * segments;
    for segment in 0..segments {
        let next_segment = (segment + 1) % segments;
        let v0 = bottom_idx;
        let v1 = ring_vertices[(last_ring_start + next_segment) as usize];
        let v2 = ring_vertices[(last_ring_start + segment) as usize];
        
        let _ = mesh.add_triangle(v0, v1, v2);
    }
    
    mesh
}

/// Parameters for cone creation
pub struct ConeParams {
    /// Radius of the base
    pub radius: f32, 
    /// Height of the cone
    pub height: f32,
    /// Number of segments around the base
    pub segments: u32,
    /// Whether to include the base cap
    pub capped: bool,
}

impl Default for ConeParams {
    fn default() -> Self {
        Self {
            radius: 1.0,
            height: 1.0,
            segments: 32,
            capped: true,
        }
    }
}

/// Creates a cone mesh with the specified parameters
pub fn create_cone(params: ConeParams) -> Mesh {
    let mut mesh = Mesh::new();
    
    // Create tip vertex at the top
    let tip_idx = mesh.add_vertex(Vertex::with_all(
        Vec3::new(0.0, params.height / 2.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec2::new(0.5, 0.0),
    ));
    
    // Create vertices for the base
    let mut base_vertices = Vec::with_capacity(params.segments as usize);
    let center_idx = if params.capped {
        Some(mesh.add_vertex(Vertex::with_all(
            Vec3::new(0.0, -params.height / 2.0, 0.0),
            Vec3::new(0.0, -1.0, 0.0),
            Vec2::new(0.5, 1.0),
        )))
    } else {
        None
    };
    
    // Create the base circle
    for i in 0..params.segments {
        let theta = 2.0 * PI * i as f32 / params.segments as f32;
        let x = params.radius * theta.cos();
        let z = params.radius * theta.sin();
        
        // Compute the normal for the side of the cone
        let side_normal = Vec3::new(x, params.height, z).normalize();
        
        let idx = mesh.add_vertex(Vertex::with_all(
            Vec3::new(x, -params.height / 2.0, z),
            if params.capped { Vec3::new(0.0, -1.0, 0.0) } else { side_normal },
            Vec2::new(i as f32 / params.segments as f32, 1.0),
        ));
        
        if !params.capped {
            let side_idx = mesh.add_vertex(Vertex::with_all(
                Vec3::new(x, -params.height / 2.0, z),
                side_normal,
                Vec2::new(i as f32 / params.segments as f32, 1.0),
            ));
            base_vertices.push(side_idx);
        } else {
            base_vertices.push(idx);
        }
    }
    
    // Create triangles for the cone sides
    for i in 0..params.segments {
        let next = (i + 1) % params.segments;
        let _ = mesh.add_triangle(tip_idx, base_vertices[next as usize], base_vertices[i as usize]);
    }
    
    // Create triangles for the base if capped
    if params.capped && center_idx.is_some() {
        let center = center_idx.unwrap();
        for i in 0..params.segments {
            let next = (i + 1) % params.segments;
            let _ = mesh.add_triangle(center, base_vertices[i as usize], base_vertices[next as usize]);
        }
    }
    
    mesh
}

/// Parameters for cylinder creation
pub struct CylinderParams {
    /// Radius of the cylinder
    pub radius: f32,
    /// Height of the cylinder
    pub height: f32,
    /// Number of segments around the circumference
    pub radial_segments: u32,
    /// Number of segments along the height
    pub height_segments: u32,
    /// Whether to include the top cap
    pub top_cap: bool,
    /// Whether to include the bottom cap
    pub bottom_cap: bool,
}

impl Default for CylinderParams {
    fn default() -> Self {
        Self {
            radius: 1.0,
            height: 2.0,
            radial_segments: 32,
            height_segments: 1,
            top_cap: true,
            bottom_cap: true,
        }
    }
}

/// Creates a cylinder mesh with the specified parameters
pub fn create_cylinder(params: CylinderParams) -> Mesh {
    let mut mesh = Mesh::new();
    
    let half_height = params.height / 2.0;
    
    // Create vertices for the main body
    let mut body_vertices = Vec::with_capacity(
        (params.height_segments + 1) as usize * params.radial_segments as usize
    );
    
    for h in 0..=params.height_segments {
        let y = -half_height + params.height * (h as f32 / params.height_segments as f32);
        let v = h as f32 / params.height_segments as f32;
        
        for r in 0..params.radial_segments {
            let theta = 2.0 * PI * r as f32 / params.radial_segments as f32;
            let x = params.radius * theta.cos();
            let z = params.radius * theta.sin();
            
            let normal = Vec3::new(x, 0.0, z).normalize();
            let u = r as f32 / params.radial_segments as f32;
            
            let idx = mesh.add_vertex(Vertex::with_all(
                Vec3::new(x, y, z),
                normal,
                Vec2::new(u, v),
            ));
            
            body_vertices.push(idx);
        }
    }
    
    // Create triangles for the cylinder body
    for h in 0..params.height_segments {
        for r in 0..params.radial_segments {
            let next_r = (r + 1) % params.radial_segments;
            let idx1 = h * params.radial_segments + r;
            let idx2 = h * params.radial_segments + next_r;
            let idx3 = (h + 1) * params.radial_segments + next_r;
            let idx4 = (h + 1) * params.radial_segments + r;
            
            let v1 = body_vertices[idx1 as usize];
            let v2 = body_vertices[idx2 as usize];
            let v3 = body_vertices[idx3 as usize];
            let v4 = body_vertices[idx4 as usize];
            
            let _ = mesh.add_triangle(v1, v2, v3);
            let _ = mesh.add_triangle(v1, v3, v4);
        }
    }
    
    // Create top cap if needed
    if params.top_cap {
        let center_idx = mesh.add_vertex(Vertex::with_all(
            Vec3::new(0.0, half_height, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec2::new(0.5, 0.5),
        ));
        
        let mut cap_vertices = Vec::with_capacity(params.radial_segments as usize);
        
        for r in 0..params.radial_segments {
            let theta = 2.0 * PI * r as f32 / params.radial_segments as f32;
            let x = params.radius * theta.cos();
            let z = params.radius * theta.sin();
            
            let u = 0.5 + 0.5 * theta.cos();
            let v = 0.5 + 0.5 * theta.sin();
            
            let idx = mesh.add_vertex(Vertex::with_all(
                Vec3::new(x, half_height, z),
                Vec3::new(0.0, 1.0, 0.0),
                Vec2::new(u, v),
            ));
            
            cap_vertices.push(idx);
        }
        
        for r in 0..params.radial_segments {
            let next = (r + 1) % params.radial_segments;
            let _ = mesh.add_triangle(center_idx, cap_vertices[r as usize], cap_vertices[next as usize]);
        }
    }
    
    // Create bottom cap if needed
    if params.bottom_cap {
        let center_idx = mesh.add_vertex(Vertex::with_all(
            Vec3::new(0.0, -half_height, 0.0),
            Vec3::new(0.0, -1.0, 0.0),
            Vec2::new(0.5, 0.5),
        ));
        
        let mut cap_vertices = Vec::with_capacity(params.radial_segments as usize);
        
        for r in 0..params.radial_segments {
            let theta = 2.0 * PI * r as f32 / params.radial_segments as f32;
            let x = params.radius * theta.cos();
            let z = params.radius * theta.sin();
            
            let u = 0.5 + 0.5 * theta.cos();
            let v = 0.5 + 0.5 * theta.sin();
            
            let idx = mesh.add_vertex(Vertex::with_all(
                Vec3::new(x, -half_height, z),
                Vec3::new(0.0, -1.0, 0.0),
                Vec2::new(u, v),
            ));
            
            cap_vertices.push(idx);
        }
        
        for r in 0..params.radial_segments {
            let next = (r + 1) % params.radial_segments;
            let _ = mesh.add_triangle(center_idx, cap_vertices[next as usize], cap_vertices[r as usize]);
        }
    }
    
    mesh
}

/// Parameters for torus creation
pub struct TorusParams {
    /// Major radius (radius from the center to the middle of the tube)
    pub radius: f32,
    /// Tube radius (radius of the tube itself)
    pub tube_radius: f32,
    /// Number of segments around the major radius
    pub radial_segments: u32,
    /// Number of segments around the tube
    pub tubular_segments: u32,
}

impl Default for TorusParams {
    fn default() -> Self {
        Self {
            radius: 1.0,
            tube_radius: 0.4,
            radial_segments: 32,
            tubular_segments: 32,
        }
    }
}

/// Creates a torus (donut shape) mesh with the specified parameters
pub fn create_torus(params: TorusParams) -> Mesh {
    let mut mesh = Mesh::new();
    
    // Create vertices
    let mut vertex_grid = Vec::with_capacity(
        (params.radial_segments + 1) as usize * (params.tubular_segments + 1) as usize
    );
    
    for radial in 0..=params.radial_segments {
        let phi = 2.0 * PI * radial as f32 / params.radial_segments as f32;
        
        for tubular in 0..=params.tubular_segments {
            let theta = 2.0 * PI * tubular as f32 / params.tubular_segments as f32;
            
            // Calculate position
            let x = (params.radius + params.tube_radius * theta.cos()) * phi.cos();
            let y = params.tube_radius * theta.sin();
            let z = (params.radius + params.tube_radius * theta.cos()) * phi.sin();
            
            // Calculate normal
            let nx = theta.cos() * phi.cos();
            let ny = theta.sin();
            let nz = theta.cos() * phi.sin();
            
            // Calculate texture coordinates
            let u = radial as f32 / params.radial_segments as f32;
            let v = tubular as f32 / params.tubular_segments as f32;
            
            let idx = mesh.add_vertex(Vertex::with_all(
                Vec3::new(x, y, z),
                Vec3::new(nx, ny, nz).normalize(),
                Vec2::new(u, v),
            ));
            
            vertex_grid.push(idx);
        }
    }
    
    // Create triangles
    for radial in 0..params.radial_segments {
        for tubular in 0..params.tubular_segments {
            let a = (params.tubular_segments + 1) * radial + tubular;
            let b = (params.tubular_segments + 1) * (radial + 1) + tubular;
            let c = (params.tubular_segments + 1) * (radial + 1) + tubular + 1;
            let d = (params.tubular_segments + 1) * radial + tubular + 1;
            
            let v1 = vertex_grid[a as usize];
            let v2 = vertex_grid[b as usize];
            let v3 = vertex_grid[c as usize];
            let v4 = vertex_grid[d as usize];
            
            let _ = mesh.add_triangle(v1, v2, v3);
            let _ = mesh.add_triangle(v1, v3, v4);
        }
    }
    
    mesh
}

/// Parameters for icosphere (subdivision-based sphere) creation
pub struct IcosphereParams {
    /// Radius of the sphere
    pub radius: f32,
    /// Number of subdivision iterations (0 = icosahedron, each increment quadruples the number of faces)
    pub subdivisions: u32,
}

impl Default for IcosphereParams {
    fn default() -> Self {
        Self {
            radius: 1.0,
            subdivisions: 2,
        }
    }
}

/// Creates an icosphere (a sphere based on an icosahedron with subdivisions)
/// This provides a more even distribution of vertices than a UV sphere
pub fn create_icosphere(params: IcosphereParams) -> Mesh {
    let mut mesh = Mesh::new();
    
    // Golden ratio constants used for icosahedron
    let t = (1.0 + 5.0_f32.sqrt()) / 2.0;
    
    // Create the 12 vertices of the icosahedron
    let vertices = [
        Vec3::new(-1.0, t, 0.0),
        Vec3::new(1.0, t, 0.0),
        Vec3::new(-1.0, -t, 0.0),
        Vec3::new(1.0, -t, 0.0),
        
        Vec3::new(0.0, -1.0, t),
        Vec3::new(0.0, 1.0, t),
        Vec3::new(0.0, -1.0, -t),
        Vec3::new(0.0, 1.0, -t),
        
        Vec3::new(t, 0.0, -1.0),
        Vec3::new(t, 0.0, 1.0),
        Vec3::new(-t, 0.0, -1.0),
        Vec3::new(-t, 0.0, 1.0),
    ];
    
    // Normalize and add vertices to the mesh
    let mut vertex_indices = Vec::with_capacity(12);
    for pos in vertices.iter() {
        let normalized = pos.normalize() * params.radius;
        
        // Calculate UV coordinates based on spherical mapping
        let phi = normalized.z.atan2(normalized.x);
        let theta = normalized.y.acos();
        let u = 1.0 - (phi / (2.0 * PI) + 0.5);
        let v = theta / PI;
        
        let idx = mesh.add_vertex(Vertex::with_all(
            normalized,
            normalized.normalize(),
            Vec2::new(u, v),
        ));
        
        vertex_indices.push(idx);
    }
    
    // Define the 20 triangular faces of the icosahedron
    let faces = [
        [0, 11, 5], [0, 5, 1], [0, 1, 7], [0, 7, 10], [0, 10, 11],
        [1, 5, 9], [5, 11, 4], [11, 10, 2], [10, 7, 6], [7, 1, 8],
        [3, 9, 4], [3, 4, 2], [3, 2, 6], [3, 6, 8], [3, 8, 9],
        [4, 9, 5], [2, 4, 11], [6, 2, 10], [8, 6, 7], [9, 8, 1],
    ];
    
    // Add initial faces to the mesh
    let mut triangles = Vec::new();
    for &[a, b, c] in faces.iter() {
        triangles.push([
            vertex_indices[a], 
            vertex_indices[b], 
            vertex_indices[c]
        ]);
    }
    
    // Helper function to find or add a midpoint vertex
    let mut midpoint_cache = std::collections::HashMap::new();
    let mut get_midpoint = |a: usize, b: usize, mesh: &mut Mesh| {
        let key = if a < b { (a, b) } else { (b, a) };
        
        if let Some(&idx) = midpoint_cache.get(&key) {
            return idx;
        }
        
        let v1 = mesh.vertices[a].position;
        let v2 = mesh.vertices[b].position;
        
        let midpoint = ((v1 + v2) * 0.5).normalize() * params.radius;
        
        // Calculate UV coordinates based on spherical mapping
        let phi = midpoint.z.atan2(midpoint.x);
        let theta = midpoint.y.acos();
        let u = 1.0 - (phi / (2.0 * PI) + 0.5);
        let v = theta / PI;
        
        let idx = mesh.add_vertex(Vertex::with_all(
            midpoint,
            midpoint.normalize(),
            Vec2::new(u, v),
        ));
        
        midpoint_cache.insert(key, idx);
        idx
    };
    
    // Perform subdivisions
    for _ in 0..params.subdivisions {
        let mut new_triangles = Vec::new();
        
        for &[a, b, c] in triangles.iter() {
            // Get midpoints
            let ab = get_midpoint(a, b, &mut mesh);
            let bc = get_midpoint(b, c, &mut mesh);
            let ca = get_midpoint(c, a, &mut mesh);
            
            // Create four triangles from the original
            new_triangles.push([a, ab, ca]);
            new_triangles.push([b, bc, ab]);
            new_triangles.push([c, ca, bc]);
            new_triangles.push([ab, bc, ca]);
        }
        
        triangles = new_triangles;
    }
    
    // Add all triangles to the mesh
    for [a, b, c] in triangles {
        let _ = mesh.add_triangle(a, b, c);
    }
    
    mesh
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Mesh;

    #[test]
    fn test_create_cube() {
        let cube = create_cube(2.0, 2.0, 2.0);
        
        // A cube should have 24 vertices (4 per face * 6 faces)
        assert_eq!(cube.vertices.len(), 24);
        
        // A cube should have 12 triangles (2 per face * 6 faces)
        assert_eq!(cube.triangles.len(), 12);
        
        // Check that normals and UVs are present
        assert!(cube.has_normals());
        assert!(cube.has_uvs());
    }

    #[test]
    fn test_create_plane() {
        let plane = create_plane(4.0, 4.0, 2, 2);
        
        // A 2x2 segmented plane should have 9 vertices (3x3 grid)
        assert_eq!(plane.vertices.len(), 9);
        
        // A 2x2 segmented plane should have 8 triangles (2 per grid cell * 4 cells)
        assert_eq!(plane.triangles.len(), 8);
        
        // Check that normals and UVs are present
        assert!(plane.has_normals());
        assert!(plane.has_uvs());
    }

    #[test]
    fn test_create_sphere() {
        let sphere = create_sphere(1.0, 8, 4);
        
        // Verify the number of vertices: 2 poles + (rings-1)*segments = 2 + 3*8 = 26
        assert_eq!(sphere.vertices.len(), 26);
        
        // Verify the number of triangles: 2*segments + (rings-2)*2*segments = 2*8 + 2*2*8 = 48
        assert_eq!(sphere.triangles.len(), 48);
        
        // Check that normals and UVs are present
        assert!(sphere.has_normals());
        assert!(sphere.has_uvs());
    }

    #[test]
    fn test_create_cone() {
        let params = ConeParams {
            radius: 1.0,
            height: 2.0,
            segments: 8,
            capped: true,
        };
        
        let cone = create_cone(params);
        
        // Verify the number of vertices: 1 tip + 1 center + segments = 10
        assert_eq!(cone.vertices.len(), 10);
        
        // Verify the number of triangles: segments (for the sides) + segments (for the cap) = 16
        assert_eq!(cone.triangles.len(), 16);
        
        // Check that normals and UVs are present
        assert!(cone.has_normals());
        assert!(cone.has_uvs());
    }

    #[test]
    fn test_create_cylinder() {
        let params = CylinderParams {
            radius: 1.0,
            height: 2.0,
            radial_segments: 8,
            height_segments: 1,
            top_cap: true,
            bottom_cap: true,
        };
        
        let cylinder = create_cylinder(params);
        
        // Verify the number of vertices: 
        // Body: radial_segments * (height_segments+1) = 8 * 2 = 16
        // Top cap: 1 center + radial_segments = 9
        // Bottom cap: 1 center + radial_segments = 9
        // Total: 16 + 9 + 9 = 34
        assert_eq!(cylinder.vertices.len(), 34);
        
        // Verify the number of triangles:
        // Body: radial_segments * height_segments * 2 = 8 * 1 * 2 = 16
        // Top cap: radial_segments = 8
        // Bottom cap: radial_segments = 8
        // Total: 16 + 8 + 8 = 32
        assert_eq!(cylinder.triangles.len(), 32);
        
        // Check that normals and UVs are present
        assert!(cylinder.has_normals());
        assert!(cylinder.has_uvs());
    }

    #[test]
    fn test_create_torus() {
        let params = TorusParams {
            radius: 1.0,
            tube_radius: 0.4,
            radial_segments: 8,
            tubular_segments: 6,
        };
        
        let torus = create_torus(params);
        
        // Verify the number of vertices: (radial_segments+1) * (tubular_segments+1) = 9 * 7 = 63
        assert_eq!(torus.vertices.len(), 63);
        
        // Verify the number of triangles: radial_segments * tubular_segments * 2 = 8 * 6 * 2 = 96
        assert_eq!(torus.triangles.len(), 96);
        
        // Check that normals and UVs are present
        assert!(torus.has_normals());
        assert!(torus.has_uvs());
    }

    #[test]
    fn test_create_icosphere() {
        let params = IcosphereParams {
            radius: 1.0,
            subdivisions: 1, // Just one subdivision for the test
        };
        
        let icosphere = create_icosphere(params);
        
        // After 1 subdivision, each face is divided into 4, and each edge creates 1 new vertex
        // Initial icosahedron: 12 vertices, 20 faces, 30 edges
        // After subdivision: 12 + 30 = 42 vertices, 20 * 4 = 80 faces
        assert_eq!(icosphere.vertices.len(), 42);
        assert_eq!(icosphere.triangles.len(), 80);
        
        // Check that normals and UVs are present
        assert!(icosphere.has_normals());
        assert!(icosphere.has_uvs());
    }
}
