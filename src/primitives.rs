//! Primitive shape generators for creating basic 3D shapes
//!
//! This module provides functions to create primitive shapes:
//! - Box/cube: A six-sided rectangular prism
//! - Cylinder: A cylinder with configurable radius and height
//! - Disk: A flat circular shape
//! - Plane: A flat rectangular shape

use glam::{Vec2, Vec3, Vec4};
use std::f32::consts::PI;

use crate::mesh::{Mesh, Triangle, Vertex};

/// Parameters for creating a box primitive
#[derive(Debug, Clone, Copy)]
pub struct BoxParameters {
    /// Width along the x-axis
    pub width: f32,
    /// Height along the y-axis
    pub height: f32,
    /// Depth along the z-axis
    pub depth: f32,
    /// Number of width segments
    pub width_segments: u32,
    /// Number of height segments
    pub height_segments: u32,
    /// Number of depth segments
    pub depth_segments: u32,
}

impl Default for BoxParameters {
    fn default() -> Self {
        Self {
            width: 1.0,
            height: 1.0,
            depth: 1.0,
            width_segments: 1,
            height_segments: 1,
            depth_segments: 1,
        }
    }
}

/// Parameters for creating a cylinder primitive
#[derive(Debug, Clone, Copy)]
pub struct CylinderParameters {
    /// Radius at the top
    pub radius_top: f32,
    /// Radius at the bottom
    pub radius_bottom: f32,
    /// Height of the cylinder
    pub height: f32,
    /// Number of radial segments
    pub radial_segments: u32,
    /// Number of height segments
    pub height_segments: u32,
    /// Whether to include top cap
    pub open_ended: bool,
    /// Angle of the cylinder sector in radians (2π for full cylinder)
    pub theta_length: f32,
}

impl Default for CylinderParameters {
    fn default() -> Self {
        Self {
            radius_top: 0.5,
            radius_bottom: 0.5,
            height: 1.0,
            radial_segments: 32,
            height_segments: 1,
            open_ended: false,
            theta_length: 2.0 * PI,
        }
    }
}

/// Parameters for creating a disk primitive
#[derive(Debug, Clone, Copy)]
pub struct DiskParameters {
    /// Inner radius
    pub inner_radius: f32,
    /// Outer radius
    pub outer_radius: f32,
    /// Number of segments around the circumference
    pub theta_segments: u32,
    /// Number of segments between inner and outer radius
    pub radial_segments: u32,
    /// Starting angle in radians
    pub theta_start: f32,
    /// Central angle in radians (2π for full disk)
    pub theta_length: f32,
}

impl Default for DiskParameters {
    fn default() -> Self {
        Self {
            inner_radius: 0.0,
            outer_radius: 0.5,
            theta_segments: 32,
            radial_segments: 1,
            theta_start: 0.0,
            theta_length: 2.0 * PI,
        }
    }
}

/// Parameters for creating a plane primitive
#[derive(Debug, Clone, Copy)]
pub struct PlaneParameters {
    /// Width along the x-axis
    pub width: f32,
    /// Height along the y-axis
    pub height: f32,
    /// Number of width segments
    pub width_segments: u32,
    /// Number of height segments
    pub height_segments: u32,
}

impl Default for PlaneParameters {
    fn default() -> Self {
        Self {
            width: 1.0,
            height: 1.0,
            width_segments: 1,
            height_segments: 1,
        }
    }
}

/// Create a box/cube primitive mesh
pub fn create_box(params: &BoxParameters) -> Mesh {
    let mut mesh = Mesh::with_name("box");
    
    let half_width = params.width / 2.0;
    let half_height = params.height / 2.0;
    let half_depth = params.depth / 2.0;
    
    let width_segments = params.width_segments.max(1);
    let height_segments = params.height_segments.max(1);
    let depth_segments = params.depth_segments.max(1);
    
    // Create a grid for each face of the box
    create_box_face(
        &mut mesh,
        width_segments,
        height_segments,
        params.width,
        params.height,
        |_u, v| Vec3::new(half_width, v * params.height - half_height, -half_depth),
        |_, _| Vec3::new(0.0, 0.0, -1.0),
        false,
    ); // Front face (-Z)
    
    create_box_face(
        &mut mesh,
        width_segments,
        height_segments,
        params.width,
        params.height,
        |_u, v| Vec3::new(half_width, v * params.height - half_height, half_depth),
        |_, _| Vec3::new(0.0, 0.0, 1.0),
        true,
    ); // Back face (+Z)
    
    create_box_face(
        &mut mesh,
        depth_segments,
        height_segments,
        params.depth,
        params.height,
        |u, v| Vec3::new(-half_width, v * params.height - half_height, u * params.depth - half_depth),
        |_, _| Vec3::new(-1.0, 0.0, 0.0),
        false,
    ); // Left face (-X)
    
    create_box_face(
        &mut mesh,
        depth_segments,
        height_segments,
        params.depth,
        params.height,
        |u, v| Vec3::new(half_width, v * params.height - half_height, half_depth - u * params.depth),
        |_, _| Vec3::new(1.0, 0.0, 0.0),
        false,
    ); // Right face (+X)
    
    create_box_face(
        &mut mesh,
        width_segments,
        depth_segments,
        params.width,
        params.depth,
        |u, v| Vec3::new(u * params.width - half_width, -half_height, v * params.depth - half_depth),
        |_, _| Vec3::new(0.0, -1.0, 0.0),
        false,
    ); // Bottom face (-Y)
    
    create_box_face(
        &mut mesh,
        width_segments,
        depth_segments,
        params.width,
        params.depth,
        |u, v| Vec3::new(u * params.width - half_width, half_height, half_depth - v * params.depth),
        |_, _| Vec3::new(0.0, 1.0, 0.0),
        false,
    ); // Top face (+Y)
    
    mesh
}

/// Helper function to create a single face of a box
fn create_box_face<P, N>(
    mesh: &mut Mesh,
    width_segments: u32,
    height_segments: u32,
    width: f32,
    height: f32,
    position_fn: P,
    normal_fn: N,
    flip: bool,
) where
    P: Fn(f32, f32) -> Vec3,
    N: Fn(f32, f32) -> Vec3,
{
    let first_vertex_index = mesh.vertices.len() as u32;
    
    // Create vertices
    for iy in 0..=height_segments {
        let v = iy as f32 / height_segments as f32;
        
        for ix in 0..=width_segments {
            let u = ix as f32 / width_segments as f32;
            
            let position = position_fn(u, v);
            let normal = normal_fn(u, v);
            
            mesh.vertices.push(Vertex {
                position,
                normal: Some(normal),
                uv: Some(Vec2::new(u, v)),
                tangent: None,
                color: None,
            });
        }
    }
    
    // Create triangles
    let stride = width_segments + 1;
    
    for iy in 0..height_segments as usize {
        for ix in 0..width_segments as usize {
            let a = first_vertex_index + ix as u32 + (iy as u32 * stride);
            let b = first_vertex_index + ix as u32 + ((iy as u32 + 1) * stride);
            let c = first_vertex_index + (ix as u32 + 1) + ((iy as u32 + 1) * stride);
            let d = first_vertex_index + (ix as u32 + 1) + (iy as u32 * stride);
            
            if flip {
                mesh.triangles.push(Triangle::new(a, c, b));
                mesh.triangles.push(Triangle::new(a, d, c));
            } else {
                mesh.triangles.push(Triangle::new(a, b, c));
                mesh.triangles.push(Triangle::new(a, c, d));
            }
        }
    }
}

/// Create a cylinder primitive mesh
pub fn create_cylinder(params: &CylinderParameters) -> Mesh {
    let mut mesh = Mesh::with_name("cylinder");
    
    let radial_segments = params.radial_segments.max(3);
    let height_segments = params.height_segments.max(1);
    
    let half_height = params.height / 2.0;
    let slope = (params.radius_bottom - params.radius_top) / params.height;
    
    let mut vertex_indices = Vec::new();
    
    // Create vertices for sides
    for iy in 0..=height_segments {
        let v = iy as f32 / height_segments as f32;
        let y = params.height * v - half_height;
        let radius = params.radius_top + (params.radius_bottom - params.radius_top) * v;
        
        let mut row = Vec::new();
        
        for ix in 0..=radial_segments {
            let u = ix as f32 / radial_segments as f32;
            let theta = params.theta_length * u;
            
            let sin_theta = theta.sin();
            let cos_theta = theta.cos();
            
            // Position
            let position = Vec3::new(
                radius * sin_theta,
                y,
                radius * cos_theta,
            );
            
            // Normal
            let normal = Vec3::new(
                sin_theta,
                slope,
                cos_theta,
            ).normalize();
            
            // UVs
            let uv = Vec2::new(u, v);
            
            // Add vertex
            let index = mesh.vertices.len() as u32;
            mesh.vertices.push(Vertex {
                position,
                normal: Some(normal),
                uv: Some(uv),
                tangent: None,
                color: None,
            });
            
            row.push(index);
        }
        
        vertex_indices.push(row);
    }
    
    // Create side triangles
    for iy in 0..height_segments as usize {
        for ix in 0..radial_segments as usize {
            let a = vertex_indices[iy][ix];
            let b = vertex_indices[iy + 1][ix];
            let c = vertex_indices[iy + 1][ix + 1];
            let d = vertex_indices[iy][ix + 1];
            
            mesh.triangles.push(Triangle::new(a, b, d));
            mesh.triangles.push(Triangle::new(b, c, d));
        }
    }
    
    // Create top and bottom caps if needed
    if !params.open_ended {
        if params.radius_top > 0.0 {
            create_cylinder_cap(
                &mut mesh,
                true,
                params.radius_top,
                half_height,
                params.radial_segments,
                params.theta_length,
            );
        }
        
        if params.radius_bottom > 0.0 {
            create_cylinder_cap(
                &mut mesh,
                false,
                params.radius_bottom,
                -half_height,
                params.radial_segments,
                params.theta_length,
            );
        }
    }
    
    mesh
}

/// Helper function to create a cap for the cylinder
fn create_cylinder_cap(
    mesh: &mut Mesh,
    is_top: bool,
    radius: f32,
    y: f32,
    radial_segments: u32,
    theta_length: f32,
) {
    let center_index = mesh.vertices.len() as u32;
    
    // Central vertex
    let normal = Vec3::new(0.0, if is_top { 1.0 } else { -1.0 }, 0.0);
    
    mesh.vertices.push(Vertex {
        position: Vec3::new(0.0, y, 0.0),
        normal: Some(normal),
        uv: Some(Vec2::new(0.5, 0.5)),
        tangent: None,
        color: None,
    });
    
    // Edge vertices
    let first_vertex = mesh.vertices.len() as u32;
    
    for ix in 0..=radial_segments {
        let u = ix as f32 / radial_segments as f32;
        let theta = theta_length * u;
        
        let sin_theta = theta.sin();
        let cos_theta = theta.cos();
        
        let position = Vec3::new(
            radius * sin_theta,
            y,
            radius * cos_theta,
        );
        
        let uv = Vec2::new(
            sin_theta * 0.5 + 0.5,
            cos_theta * 0.5 + 0.5,
        );
        
        mesh.vertices.push(Vertex {
            position,
            normal: Some(normal),
            uv: Some(uv),
            tangent: None,
            color: None,
        });
    }
    
    // Cap triangles
    for ix in 0..radial_segments as usize {
        let a = center_index;
        let b = first_vertex + ix as u32;
        let c = first_vertex + (ix + 1) as u32;
        
        if is_top {
            mesh.triangles.push(Triangle::new(a, b, c));
        } else {
            mesh.triangles.push(Triangle::new(a, c, b));
        }
    }
}

/// Create a disk primitive mesh
pub fn create_disk(params: &DiskParameters) -> Mesh {
    let mut mesh = Mesh::with_name("disk");
    
    let theta_segments = params.theta_segments.max(3);
    let radial_segments = params.radial_segments.max(1);
    
    let mut vertex_indices = Vec::new();
    
    // Create vertices
    for ir in 0..=radial_segments {
        let r = ir as f32 / radial_segments as f32;
        let radius = params.inner_radius + r * (params.outer_radius - params.inner_radius);
        
        let mut row = Vec::new();
        
        for it in 0..=theta_segments {
            let t = it as f32 / theta_segments as f32;
            let theta = params.theta_start + t * params.theta_length;
            
            let sin_theta = theta.sin();
            let cos_theta = theta.cos();
            
            // Position
            let position = Vec3::new(
                radius * cos_theta,
                0.0,
                radius * sin_theta,
            );
            
            // Normal always points up for disk
            let normal = Vec3::new(0.0, 1.0, 0.0);
            
            // UVs (transform from polar to cartesian coordinates)
            let uv = Vec2::new(
                (position.x / params.outer_radius + 1.0) / 2.0,
                (position.z / params.outer_radius + 1.0) / 2.0,
            );
            
            // Add vertex
            let index = mesh.vertices.len() as u32;
            mesh.vertices.push(Vertex {
                position,
                normal: Some(normal),
                uv: Some(uv),
                tangent: None,
                color: None,
            });
            
            row.push(index);
        }
        
        vertex_indices.push(row);
    }
    
    // Create triangles
    for ir in 0..radial_segments as usize {
        for it in 0..theta_segments as usize {
            let a = vertex_indices[ir][it];
            let b = vertex_indices[ir + 1][it];
            let c = vertex_indices[ir + 1][it + 1];
            let d = vertex_indices[ir][it + 1];
            
            mesh.triangles.push(Triangle::new(a, b, d));
            mesh.triangles.push(Triangle::new(b, c, d));
        }
    }
    
    mesh
}

/// Create a plane primitive mesh
pub fn create_plane(params: &PlaneParameters) -> Mesh {
    let mut mesh = Mesh::with_name("plane");
    
    let width_segments = params.width_segments.max(1);
    let height_segments = params.height_segments.max(1);
    
    let width_half = params.width / 2.0;
    let height_half = params.height / 2.0;
    
    let grid_x = width_segments;
    let grid_y = height_segments;
    
    let segment_width = params.width / grid_x as f32;
    let segment_height = params.height / grid_y as f32;
    
    // Create vertices
    for iy in 0..=grid_y {
        let y = iy as f32 * segment_height - height_half;
        
        for ix in 0..=grid_x {
            let x = ix as f32 * segment_width - width_half;
            
            let position = Vec3::new(x, 0.0, y);
            let normal = Vec3::new(0.0, 1.0, 0.0);
            let uv = Vec2::new(ix as f32 / grid_x as f32, 1.0 - (iy as f32 / grid_y as f32));
            
            mesh.vertices.push(Vertex {
                position,
                normal: Some(normal),
                uv: Some(uv),
                tangent: None,
                color: None,
            });
        }
    }
    
    // Create triangles
    for iy in 0..grid_y as usize {
        for ix in 0..grid_x as usize {
            let a = (ix + (grid_x as usize + 1) * iy) as u32;
            let b = (ix + (grid_x as usize + 1) * (iy + 1)) as u32;
            let c = ((ix + 1) + (grid_x as usize + 1) * (iy + 1)) as u32;
            let d = ((ix + 1) + (grid_x as usize + 1) * iy) as u32;
            
            mesh.triangles.push(Triangle::new(a, b, d));
            mesh.triangles.push(Triangle::new(b, c, d));
        }
    }
    
    mesh
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_box() {
        let params = BoxParameters::default();
        let mesh = create_box(&params);
        
        // A default cube should have 24 vertices (4 per face * 6 faces)
        // and 12 triangles (2 per face * 6 faces)
        assert_eq!(mesh.vertex_count(), 24);
        assert_eq!(mesh.triangle_count(), 12);
    }
    
    #[test]
    fn test_create_box_segmented() {
        let params = BoxParameters {
            width_segments: 2,
            height_segments: 2,
            depth_segments: 2,
            ..BoxParameters::default()
        };
        let mesh = create_box(&params);
        
        // A segmented cube should have 6 faces with 3x3 grid of vertices each
        // 6 faces * ((2+1)*(2+1)) vertices = 6 * 9 = 54 vertices
        // 6 faces * (2*2*2) triangles = 6 * 8 = 48 triangles
        assert_eq!(mesh.vertex_count(), 54);
        assert_eq!(mesh.triangle_count(), 48);
    }
    
    #[test]
    fn test_create_cylinder() {
        let params = CylinderParameters::default();
        let mesh = create_cylinder(&params);
        
        // Default cylinder has 32 radial segments, 1 height segment
        // Vertices: (32+1)*(1+1) + 1 (center top) + 32+1 (rim top) + 1 (center bottom) + 32+1 (rim bottom)
        // Triangles: 32*2 (sides) + 32 (top) + 32 (bottom)
        assert_eq!(mesh.vertex_count(), (33 * 2) + 1 + 33 + 1 + 33);
        assert_eq!(mesh.triangle_count(), 32 * 2 + 32 + 32);
    }
    
    #[test]
    fn test_create_disk() {
        let params = DiskParameters::default();
        let mesh = create_disk(&params);
        
        // Default disk has 32 theta segments, 1 radial segment
        // Vertices: (32+1)*(1+1) = 66
        // Triangles: 32*2 = 64
        assert_eq!(mesh.vertex_count(), 66);
        assert_eq!(mesh.triangle_count(), 64);
    }
    
    #[test]
    fn test_create_plane() {
        let params = PlaneParameters::default();
        let mesh = create_plane(&params);
        
        // Default plane has 1 width segment, 1 height segment
        // Vertices: (1+1)*(1+1) = 4
        // Triangles: 1*1*2 = 2
        assert_eq!(mesh.vertex_count(), 4);
        assert_eq!(mesh.triangle_count(), 2);
    }
}
