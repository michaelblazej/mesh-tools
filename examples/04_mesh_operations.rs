use mesh_tools::{
    primitives::{create_sphere, create_cube},
    modifiers::{weld_vertices, WeldParameters, remove_unused_vertices, remove_degenerate_triangles},
    export::ExportMesh,
};
use glam::Vec3;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create output directory if it doesn't exist
    std::fs::create_dir_all("output")?;

    // Example 1: Demonstrate vertex welding on a noisy mesh
    let mut noisy_mesh = create_sphere(1.0, 16, 8);
    
    // Add some "noise" to create duplicate vertices
    for vertex in &mut noisy_mesh.vertices {
        // Add small random offsets to vertex positions
        vertex.position += Vec3::new(
            (rand() - 0.5) * 0.01,
            (rand() - 0.5) * 0.01,
            (rand() - 0.5) * 0.01,
        );
    }
    
    println!("Noisy sphere created with {} vertices", noisy_mesh.vertices.len());
    noisy_mesh.export_glb("output/noisy_sphere.glb")?;
    
    // Weld vertices within a small threshold
    let weld_params = WeldParameters {
        threshold: 0.02,
        respect_normals: false,
        respect_uvs: false,
    };
    
    let welded_count = weld_vertices(&mut noisy_mesh, weld_params)?;
    println!("Welded {} duplicate vertices", welded_count);
    println!("Optimized sphere now has {} vertices", noisy_mesh.vertices.len());
    noisy_mesh.export_glb("output/welded_sphere.glb")?;
    
    // Example 2: Remove unused vertices
    let mut cube = create_cube(1.0, 1.0, 1.0);
    
    // Add some extra unused vertices
    for i in 0..10 {
        cube.add_vertex(
            mesh_tools::Vertex::new(Vec3::new(10.0 + i as f32, 0.0, 0.0))
        );
    }
    
    println!("Cube with unused vertices: {} vertices", cube.vertices.len());
    let removed = remove_unused_vertices(&mut cube);
    println!("Removed {} unused vertices", removed);
    println!("Optimized cube now has {} vertices", cube.vertices.len());
    cube.export_glb("output/optimized_cube.glb")?;
    
    // Example 3: Remove degenerate triangles
    let mut sphere = create_sphere(1.0, 8, 4);
    let original_triangle_count = sphere.triangles.len();
    
    // Create some degenerate triangles by duplicating indices
    sphere.triangles.push(mesh_tools::Triangle::new(0, 0, 1));
    sphere.triangles.push(mesh_tools::Triangle::new(1, 2, 2));
    
    println!("Sphere with degenerate triangles: {} triangles", sphere.triangles.len());
    let removed = remove_degenerate_triangles(&mut sphere);
    println!("Removed {} degenerate triangles", removed);
    println!("Cleaned sphere now has {} triangles", sphere.triangles.len());
    assert_eq!(sphere.triangles.len(), original_triangle_count);
    sphere.export_glb("output/cleaned_sphere.glb")?;
    
    println!("All mesh optimization examples completed");
    
    Ok(())
}

// Simple random number generator between 0 and 1
fn rand() -> f32 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    (seed % 10000) as f32 / 10000.0
}
