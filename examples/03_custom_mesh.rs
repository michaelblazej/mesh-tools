use mesh_tools::{Mesh, Vertex, Triangle, MeshResult, export::ExportMesh};
use glam::{Vec2, Vec3};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create output directory if it doesn't exist
    std::fs::create_dir_all("output")?;

    // Create a custom pyramid mesh from scratch
    let pyramid = create_pyramid()?;
    println!("Custom pyramid created with {} vertices and {} triangles", 
             pyramid.vertices.len(), pyramid.triangles.len());
    pyramid.export_glb("output/pyramid.glb")?;
    
    // Create a more complex custom grid mesh
    let grid = create_grid(5, 5, 2.0)?;
    println!("Grid mesh created with {} vertices and {} triangles", 
             grid.vertices.len(), grid.triangles.len());
    grid.export_glb("output/grid.glb")?;
    
    // Create a custom mesh with texture coordinates for a checker pattern
    let textured_quad = create_textured_quad()?;
    println!("Textured quad created with UV coordinates");
    textured_quad.export_glb("output/textured_quad.glb")?;
    
    println!("All custom meshes exported to the 'output' directory");
    
    Ok(())
}

// Create a simple pyramid mesh (square base with triangular sides)
fn create_pyramid() -> MeshResult<Mesh> {
    let mut mesh = Mesh::new();
    
    // Add vertices (5 vertices: 4 for base, 1 for top)
    // Base vertices
    let v0 = mesh.add_vertex(Vertex::with_uv(
        Vec3::new(-1.0, 0.0, -1.0),
        Vec2::new(0.0, 0.0),
    ));
    
    let v1 = mesh.add_vertex(Vertex::with_uv(
        Vec3::new(1.0, 0.0, -1.0),
        Vec2::new(1.0, 0.0),
    ));
    
    let v2 = mesh.add_vertex(Vertex::with_uv(
        Vec3::new(1.0, 0.0, 1.0),
        Vec2::new(1.0, 1.0),
    ));
    
    let v3 = mesh.add_vertex(Vertex::with_uv(
        Vec3::new(-1.0, 0.0, 1.0),
        Vec2::new(0.0, 1.0),
    ));
    
    // Top vertex
    let v4 = mesh.add_vertex(Vertex::new(Vec3::new(0.0, 2.0, 0.0)));
    
    // Add triangles for the base (2 triangles to form a quad)
    mesh.add_triangle(v0, v2, v1)?; // Base triangle 1
    mesh.add_triangle(v0, v3, v2)?; // Base triangle 2
    
    // Add triangles for the sides
    mesh.add_triangle(v0, v1, v4)?; // Side 1
    mesh.add_triangle(v1, v2, v4)?; // Side 2
    mesh.add_triangle(v2, v3, v4)?; // Side 3
    mesh.add_triangle(v3, v0, v4)?; // Side 4
    
    // Calculate normals
    mesh.calculate_normals();
    
    Ok(mesh)
}

// Create a grid mesh with variable dimensions
fn create_grid(width_segments: u32, depth_segments: u32, size: f32) -> MeshResult<Mesh> {
    let mut mesh = Mesh::new();
    
    let half_size = size / 2.0;
    let width_segment_size = size / width_segments as f32;
    let depth_segment_size = size / depth_segments as f32;
    
    // Create a 2D array of vertex indices
    let mut vertex_grid = Vec::with_capacity((width_segments + 1) as usize * (depth_segments + 1) as usize);
    
    // Create vertices
    for z in 0..=depth_segments {
        for x in 0..=width_segments {
            let x_pos = -half_size + x as f32 * width_segment_size;
            let z_pos = -half_size + z as f32 * depth_segment_size;
            
            // Add a height function to create some variation (a sine wave pattern)
            let y_pos = 0.2 * (x_pos.sin() * z_pos.cos());
            
            // Calculate texture coordinates
            let u = x as f32 / width_segments as f32;
            let v = z as f32 / depth_segments as f32;
            
            // Add the vertex
            let vertex_idx = mesh.add_vertex(Vertex::with_uv(
                Vec3::new(x_pos, y_pos, z_pos),
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
            mesh.add_triangle(v0, v1, v2)?;
            mesh.add_triangle(v0, v2, v3)?;
        }
    }
    
    // Calculate normals
    mesh.calculate_normals();
    
    Ok(mesh)
}

// Create a quad with detailed texture coordinates
fn create_textured_quad() -> MeshResult<Mesh> {
    let mut mesh = Mesh::new();
    
    // Create four vertices with carefully assigned UVs
    let v0 = mesh.add_vertex(Vertex::with_all(
        Vec3::new(-1.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0), // Normal pointing up
        Vec2::new(0.0, 0.0),      // Bottom-left corner of texture
    ));
    
    let v1 = mesh.add_vertex(Vertex::with_all(
        Vec3::new(1.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec2::new(1.0, 0.0),      // Bottom-right corner of texture
    ));
    
    let v2 = mesh.add_vertex(Vertex::with_all(
        Vec3::new(1.0, 0.0, 1.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec2::new(1.0, 1.0),      // Top-right corner of texture
    ));
    
    let v3 = mesh.add_vertex(Vertex::with_all(
        Vec3::new(-1.0, 0.0, 1.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec2::new(0.0, 1.0),      // Top-left corner of texture
    ));
    
    // Add two triangles to form the quad
    mesh.add_triangle(v0, v1, v2)?;
    mesh.add_triangle(v0, v2, v3)?;
    
    Ok(mesh)
}
