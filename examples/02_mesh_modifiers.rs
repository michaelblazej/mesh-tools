use mesh_tools::{
    primitives::create_cube,
    modifiers::{scale_mesh, rotate_mesh, translate_mesh, flip_normals, subdivide_mesh},
    export::ExportMesh,
};
use glam::{Vec3, Quat};
use std::f32::consts::PI;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create output directory if it doesn't exist
    std::fs::create_dir_all("output")?;

    // Start with a basic cube
    let mut cube = create_cube(1.0, 1.0, 1.0);
    println!("Original cube: {} vertices, {} triangles", 
             cube.vertices.len(), cube.triangles.len());
    cube.export_glb("output/original_cube.glb")?;
    
    // Example 1: Scale the cube non-uniformly
    let mut scaled_cube = cube.clone();
    scale_mesh(&mut scaled_cube, Vec3::new(2.0, 0.5, 1.0));
    println!("Scaled cube created");
    scaled_cube.export_glb("output/scaled_cube.glb")?;
    
    // Example 2: Rotate the cube
    let mut rotated_cube = cube.clone();
    let rotation = Quat::from_rotation_y(PI / 4.0) * Quat::from_rotation_x(PI / 6.0);
    rotate_mesh(&mut rotated_cube, rotation);
    println!("Rotated cube created");
    rotated_cube.export_glb("output/rotated_cube.glb")?;
    
    // Example 3: Translate the cube
    let mut translated_cube = cube.clone();
    translate_mesh(&mut translated_cube, Vec3::new(2.0, 1.0, 0.0));
    println!("Translated cube created");
    translated_cube.export_glb("output/translated_cube.glb")?;
    
    // Example 4: Flip normals
    let mut flipped_cube = cube.clone();
    flip_normals(&mut flipped_cube);
    println!("Cube with flipped normals created");
    flipped_cube.export_glb("output/flipped_cube.glb")?;
    
    // Example 5: Subdivide the cube (makes it smoother)
    let mut subdivided_cube = cube.clone();
    subdivide_mesh(&mut subdivided_cube)?;
    println!("Subdivided cube created: {} vertices, {} triangles", 
             subdivided_cube.vertices.len(), subdivided_cube.triangles.len());
    subdivided_cube.export_glb("output/subdivided_cube.glb")?;
    
    // Example 6: Subdivision + smoothing creates a "sphere-like" object
    let mut smooth_subdivided_cube = subdivided_cube.clone();
    subdivide_mesh(&mut smooth_subdivided_cube)?; // Second subdivision
    for vertex in &mut smooth_subdivided_cube.vertices {
        // Normalize all vertices to create a spherical shape
        vertex.position = vertex.position.normalize();
    }
    // Recalculate normals to match the new shape
    smooth_subdivided_cube.calculate_normals();
    println!("Smooth subdivided cube created (approaching a sphere)");
    smooth_subdivided_cube.export_glb("output/smooth_subdivided_cube.glb")?;
    
    println!("All modified meshes exported to the 'output' directory");
    
    Ok(())
}
