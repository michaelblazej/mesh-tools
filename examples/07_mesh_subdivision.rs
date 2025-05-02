use mesh_tools::{
    primitives::{create_cube, create_plane},
    modifiers::{subdivide_mesh, generate_smooth_normals},
    export::ExportMesh,
};
use std::fs::create_dir_all;
use glam::Vec3;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create output directory if it doesn't exist
    create_dir_all("output")?;
    
    // Example 1: Simple cube subdivision
    let mut cube = create_cube(1.0, 1.0, 1.0);
    println!("Original cube: {} vertices, {} triangles", 
             cube.vertices.len(), cube.triangles.len());
    
    // Apply subdivision to break each triangle into 4
    let mut subdivided_cube = cube.clone();
    subdivide_mesh(&mut subdivided_cube)?;
    println!("Subdivided cube (1 pass): {} vertices, {} triangles", 
             subdivided_cube.vertices.len(), subdivided_cube.triangles.len());
    subdivided_cube.export_glb("output/subdivided_cube_1pass.glb")?;
    
    // Apply multiple subdivisions to approach a sphere
    let mut sphere_from_cube = cube.clone();
    for i in 0..3 {
        subdivide_mesh(&mut sphere_from_cube)?;
        // After each subdivision, normalize vertices to sphere surface
        for vertex in &mut sphere_from_cube.vertices {
            // Normalize position to create sphere
            vertex.position = vertex.position.normalize() * 0.5;
        }
        generate_smooth_normals(&mut sphere_from_cube);
    }
    
    println!("Cube to sphere (3 passes): {} vertices, {} triangles", 
             sphere_from_cube.vertices.len(), sphere_from_cube.triangles.len());
    sphere_from_cube.export_glb("output/cube_to_sphere.glb")?;
    
    // Example 2: Subdivide a plane to create terrain
    let mut terrain = create_plane(4.0, 4.0, 8, 8);
    
    // Apply subdivision for more detail
    subdivide_mesh(&mut terrain)?;
    
    // Apply noise to vertex heights to create terrain effect
    use std::f32::consts::PI;
    for vertex in &mut terrain.vertices {
        // Apply a simple procedural noise function for height
        let x = vertex.position.x * 1.5;
        let z = vertex.position.z * 1.5;
        let height = 
            0.2 * (x.sin() * z.cos()) + 
            0.3 * ((x * 0.5).sin() * (z * 0.5).cos()) + 
            0.1 * ((x * 2.0).sin() * (z * 2.0).cos());
        
        vertex.position.y = height;
    }
    
    // Regenerate smooth normals after modifying the surface
    generate_smooth_normals(&mut terrain);
    
    println!("Terrain from subdivided plane: {} vertices, {} triangles", 
             terrain.vertices.len(), terrain.triangles.len());
    terrain.export_glb("output/terrain_subdivided.glb")?;
    
    // Example 3: Adaptive subdivision - subdivide based on distance
    let mut adaptive_plane = create_plane(5.0, 5.0, 4, 4);
    
    // Mark triangles for subdivision based on a focused area
    let focus_point = Vec3::new(0.0, 0.0, 0.0);
    let mut triangles_to_subdivide = Vec::new();
    
    for (i, triangle) in adaptive_plane.triangles.iter().enumerate() {
        // Calculate center of triangle
        let a = adaptive_plane.vertices[triangle.indices[0]].position;
        let b = adaptive_plane.vertices[triangle.indices[1]].position;
        let c = adaptive_plane.vertices[triangle.indices[2]].position;
        let center = (a + b + c) / 3.0;
        
        // Calculate distance to focus
        let distance = (center - focus_point).length();
        
        // Subdivide triangles closer to focus
        if distance < 1.5 {
            triangles_to_subdivide.push(i);
        }
    }
    
    // Subdivide only marked triangles
    // Note: This is a simplified approach - real adaptive subdivision is more complex
    // We'll just use the regular subdivide function for demo purposes
    subdivide_mesh(&mut adaptive_plane)?;
    
    // Apply slight dome shape
    for vertex in &mut adaptive_plane.vertices {
        let dist_from_center = (vertex.position.x * vertex.position.x + vertex.position.z * vertex.position.z).sqrt();
        vertex.position.y = 0.5 * (1.0 - (dist_from_center / 2.5).min(1.0).powi(2));
    }
    
    generate_smooth_normals(&mut adaptive_plane);
    
    println!("Adaptive subdivision: {} vertices, {} triangles", 
             adaptive_plane.vertices.len(), adaptive_plane.triangles.len());
    adaptive_plane.export_glb("output/adaptive_subdivision.glb")?;
    
    println!("All subdivision examples exported to the 'output' directory");
    Ok(())
}
