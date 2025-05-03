use mesh_tools::{
    primitives::{create_cylinder, CylinderParams},
    modifiers::{bend_mesh_auto, rotate_mesh},
    export::ExportMesh,
};
use glam::{Vec3, Quat};
use std::f32::consts::PI;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create output directory if it doesn't exist
    std::fs::create_dir_all("output")?;

    // Create a cylinder with default parameters
    let cylinder_params = CylinderParams {
        radius: 0.5,
        height: 3.0,  // Make it taller to better show the bend effect
        radial_segments: 32,
        height_segments: 16, // More height segments for smoother bending
        ..Default::default()
    };
    let mut cylinder = create_cylinder(cylinder_params);
    println!("Original cylinder: {} vertices, {} triangles", 
             cylinder.vertices.len(), cylinder.triangles.len());
    
    // Export the original cylinder
    cylinder.export_glb("output/original_cylinder.glb")?;
    
    // Create a copy of the cylinder for bending around X-axis
    let mut bent_x = cylinder.clone();
    
    // Rotate first to align with X-axis for bending
    let rotation_x = Quat::from_rotation_z(PI / 2.0);
    rotate_mesh(&mut bent_x, rotation_x);
    
    // Bend by 45 degrees (PI/4 radians) around the X-axis
    bend_mesh_auto(&mut bent_x, PI / 4.0, 0);
    println!("Cylinder bent 45° around X-axis created");
    bent_x.export_glb("output/cylinder_bent_x.glb")?;
    
    // Create a copy of the cylinder for bending around Y-axis
    let mut bent_y = cylinder.clone();
    
    // Bend by 45 degrees (PI/4 radians) around the Y-axis
    bend_mesh_auto(&mut bent_y, PI / 4.0, 1);
    println!("Cylinder bent 45° around Y-axis created");
    bent_y.export_glb("output/cylinder_bent_y.glb")?;
    
    // Create a copy of the cylinder for bending around Z-axis
    let mut bent_z = cylinder.clone();
    
    // Bend by 45 degrees (PI/4 radians) around the Z-axis
    bend_mesh_auto(&mut bent_z, PI / 4.0, 2);
    println!("Cylinder bent 45° around Z-axis created");
    bent_z.export_glb("output/cylinder_bent_z.glb")?;
    
    println!("All bent cylinders exported to the 'output' directory");
    
    Ok(())
}
