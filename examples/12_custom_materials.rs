use mesh_tools::{
    primitives::{create_cube, create_sphere, create_torus, TorusParams},
    modifiers::{scale_mesh, generate_smooth_normals},
    export::{ExportMesh, GlbExportOptions, Material},
};
use std::fs::create_dir_all;
use glam::Vec3;
use std::f32::consts::PI;

/// This example demonstrates how to use custom material names when exporting meshes to GLB
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create output directory if it doesn't exist
    create_dir_all("output")?;
    
    // Example 1: Create a red sphere with a custom material name
    let mut red_sphere = create_sphere(1.0, 32, 16);
    generate_smooth_normals(&mut red_sphere);
    
    // Create custom material options 
    let red_material_options = GlbExportOptions {
        name: "RedSphere".to_string(),
        export_normals: true,
        export_uvs: true,
        export_tangents: false,
        export_colors: false,
        material: Material {
            name: "GlowingRedMaterial".to_string(),
            base_color: [0.9, 0.1, 0.1],
            metallic: 0.7,
            roughness: 0.2,
            emissive: [0.5, 0.0, 0.0],
        },
    };
    
    // Export with the custom material name
    red_sphere.export_glb_with_options("output/red_material_sphere.glb", red_material_options)?;
    println!("Exported red sphere with 'GlowingRedMaterial' material name");
    
    // Example 2: Create a blue cube with a different material name
    let mut blue_cube = create_cube(1.0, 1.0, 1.0);
    
    let blue_material_options = GlbExportOptions {
        name: "BlueCube".to_string(),
        export_normals: true,
        export_uvs: true,
        export_tangents: false,
        export_colors: false,
        material: Material {
            name: "MetallicBlueMaterial".to_string(),
            base_color: [0.1, 0.3, 0.9],
            metallic: 0.9,
            roughness: 0.1,
            emissive: [0.0, 0.0, 0.2],
        },
    };
    
    blue_cube.export_glb_with_options("output/blue_material_cube.glb", blue_material_options)?;
    println!("Exported blue cube with 'MetallicBlueMaterial' material name");
    
    // Example 3: Create a gold torus with yet another material name
    let mut gold_torus = create_torus(TorusParams {
        radius: 1.0,
        tube_radius: 0.3,
        radial_segments: 32,
        tubular_segments: 16,
        ..Default::default()
    });
    scale_mesh(&mut gold_torus, Vec3::new(0.5, 0.5, 0.5));
    generate_smooth_normals(&mut gold_torus);
    
    let gold_material_options = GlbExportOptions {
        name: "GoldTorus".to_string(),
        export_normals: true,
        export_uvs: true,
        export_tangents: false,
        export_colors: false,
        material: Material {
            name: "PolishedGold".to_string(),
            base_color: [1.0, 0.85, 0.0],
            metallic: 1.0,
            roughness: 0.1,
            emissive: [0.2, 0.2, 0.0],
        },
    };
    
    gold_torus.export_glb_with_options("output/gold_material_torus.glb", gold_material_options)?;
    println!("Exported gold torus with 'PolishedGold' material name");
    
    println!("\nAll models exported successfully to the 'output' directory.");
    println!("The GLB files contain the following custom material names:");
    println!("  - GlowingRedMaterial (red sphere)");
    println!("  - MetallicBlueMaterial (blue cube)");
    println!("  - PolishedGold (gold torus)");
    println!("\nYou can verify the material names by opening the GLB files in a 3D viewer that shows material names.");
    
    Ok(())
}
