use mesh_tools::{
    Mesh, Vertex,
    primitives::{create_cube, create_sphere},
    modifiers::{weld_vertices, remove_unused_vertices, remove_degenerate_triangles, WeldParameters},
    export::ExportMesh,
};
use std::fs::create_dir_all;
use glam::{Vec3, Vec2};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create output directory if it doesn't exist
    create_dir_all("output")?;
    
    // Example 1: Create a mesh with duplicated vertices along a seam
    let mut seamed_cube = Mesh::new();
    
    // Front face (with duplicated vertices along right edge)
    seamed_cube.add_vertex(Vertex::with_all(Vec3::new(-0.5, -0.5, 0.5), Vec3::new(0.0, 0.0, 1.0), Vec2::new(0.0, 0.0)));
    seamed_cube.add_vertex(Vertex::with_all(Vec3::new(0.5, -0.5, 0.5), Vec3::new(0.0, 0.0, 1.0), Vec2::new(1.0, 0.0)));
    seamed_cube.add_vertex(Vertex::with_all(Vec3::new(0.5, 0.5, 0.5), Vec3::new(0.0, 0.0, 1.0), Vec2::new(1.0, 1.0)));
    seamed_cube.add_vertex(Vertex::with_all(Vec3::new(-0.5, 0.5, 0.5), Vec3::new(0.0, 0.0, 1.0), Vec2::new(0.0, 1.0)));
    
    // Right face (with duplicated vertices along left edge, same positions as front face right edge)
    seamed_cube.add_vertex(Vertex::with_all(Vec3::new(0.5, -0.5, 0.5), Vec3::new(1.0, 0.0, 0.0), Vec2::new(0.0, 0.0)));
    seamed_cube.add_vertex(Vertex::with_all(Vec3::new(0.5, -0.5, -0.5), Vec3::new(1.0, 0.0, 0.0), Vec2::new(1.0, 0.0)));
    seamed_cube.add_vertex(Vertex::with_all(Vec3::new(0.5, 0.5, -0.5), Vec3::new(1.0, 0.0, 0.0), Vec2::new(1.0, 1.0)));
    seamed_cube.add_vertex(Vertex::with_all(Vec3::new(0.5, 0.5, 0.5), Vec3::new(1.0, 0.0, 0.0), Vec2::new(0.0, 1.0)));
    
    // Add triangles - front face
    seamed_cube.add_triangle(0, 1, 2)?;
    seamed_cube.add_triangle(0, 2, 3)?;
    
    // Add triangles - right face
    seamed_cube.add_triangle(4, 5, 6)?;
    seamed_cube.add_triangle(4, 6, 7)?;
    
    println!("Seamed cube before welding: {} vertices, {} triangles", 
             seamed_cube.vertices.len(), seamed_cube.triangles.len());
    seamed_cube.export_glb("output/seamed_cube_before_welding.glb")?;
    
    // Apply vertex welding
    let mut welded_cube = seamed_cube.clone();
    let params = WeldParameters {
        threshold: 0.001,
        respect_normals: false,
        respect_uvs: false,
    };
    let vertices_welded = weld_vertices(&mut welded_cube, params)?;
    
    println!("Welded cube after welding: {} vertices, {} triangles (welded {} vertices)", 
             welded_cube.vertices.len(), welded_cube.triangles.len(), vertices_welded);
    welded_cube.export_glb("output/welded_cube.glb")?;
    
    // Example 2: Create a mesh with unused vertices
    let mut redundant_mesh = create_cube(1.0, 1.0, 1.0);
    
    // Add several unused vertices
    for i in 0..10 {
        redundant_mesh.add_vertex(Vertex::new(Vec3::new(2.0, i as f32 * 0.1, 0.0)));
    }
    
    println!("Mesh with unused vertices: {} vertices, {} triangles", 
             redundant_mesh.vertices.len(), redundant_mesh.triangles.len());
    redundant_mesh.export_glb("output/redundant_mesh.glb")?;
    
    // Remove unused vertices
    let mut optimized_mesh = redundant_mesh.clone();
    let removed = remove_unused_vertices(&mut optimized_mesh);
    
    println!("Optimized mesh: {} vertices, {} triangles (removed {} vertices)", 
             optimized_mesh.vertices.len(), optimized_mesh.triangles.len(), removed);
    optimized_mesh.export_glb("output/optimized_mesh.glb")?;
    
    // Example 3: Create a mesh with degenerate triangles
    let mut degenerate_mesh = create_sphere(1.0, 16, 8);
    
    // Create some degenerate triangles (where two vertices are the same)
    degenerate_mesh.add_triangle(0, 0, 1)?; // Degenerate - two same vertices
    degenerate_mesh.add_triangle(2, 3, 2)?; // Degenerate - two same vertices
    degenerate_mesh.add_triangle(4, 5, 5)?; // Degenerate - two same vertices
    
    println!("Mesh with degenerate triangles: {} vertices, {} triangles", 
             degenerate_mesh.vertices.len(), degenerate_mesh.triangles.len());
    degenerate_mesh.export_glb("output/degenerate_mesh.glb")?;
    
    // Remove degenerate triangles
    let mut cleaned_mesh = degenerate_mesh.clone();
    let removed = remove_degenerate_triangles(&mut cleaned_mesh);
    
    println!("Cleaned mesh: {} vertices, {} triangles (removed {} triangles)", 
             cleaned_mesh.vertices.len(), cleaned_mesh.triangles.len(), removed);
    cleaned_mesh.export_glb("output/cleaned_mesh.glb")?;
    
    println!("All mesh welding and optimization examples exported to the 'output' directory");
    Ok(())
}
