use mesh_tools::{
    Mesh, Scene,
    primitives::{create_cube, create_sphere, create_plane, create_torus, TorusParams},
    modifiers::{translate_mesh, scale_mesh, generate_smooth_normals},
    export::{Material, ExportScene},
};
use std::fs::create_dir_all;
use glam::Vec3;

/// This example demonstrates creating a scene with multiple objects, 
/// each having its own custom material.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create output directory if it doesn't exist
    create_dir_all("output")?;
    
    // Create a scene with multiple objects, each with its own material
    let mut scene = Scene::new("MaterialShowcase");
    
    // 1. Add a gold cube
    let mut gold_cube = create_cube(1.0, 1.0, 1.0);
    translate_mesh(&mut gold_cube, Vec3::new(2.0, 0.0, 0.0));
    
    // Set material for the cube
    gold_cube.with_material(Material {
        name: "GoldMaterial".to_string(),
        base_color: [1.0, 0.85, 0.0], // Gold color
        metallic: 0.95,               // Very metallic
        roughness: 0.1,               // Very smooth
        emissive: [0.1, 0.1, 0.0],    // Slight emission
    });
    
    // Add cube to scene
    scene.add_mesh(gold_cube);
    
    // 2. Add a glass-like blue sphere
    let mut glass_sphere = create_sphere(0.8, 24, 12);
    translate_mesh(&mut glass_sphere, Vec3::new(-2.0, 0.0, 0.0));
    generate_smooth_normals(&mut glass_sphere);
    
    // Set material for the sphere
    glass_sphere.with_material(Material {
        name: "GlassMaterial".to_string(),
        base_color: [0.4, 0.6, 0.9],  // Blue tint
        metallic: 0.0,                // Non-metallic
        roughness: 0.05,              // Very smooth
        emissive: [0.0, 0.0, 0.0],    // No emission
    });
    
    // Add sphere to scene
    scene.add_mesh(glass_sphere);
    
    // 3. Add a glowing green torus
    let mut glow_torus = create_torus(TorusParams {
        radius: 1.0,
        tube_radius: 0.3,
        radial_segments: 24,
        tubular_segments: 16,
    });
    translate_mesh(&mut glow_torus, Vec3::new(0.0, 0.0, 2.0));
    generate_smooth_normals(&mut glow_torus);
    
    // Set material for the torus
    glow_torus.with_material(Material {
        name: "GlowingGreenMaterial".to_string(),
        base_color: [0.2, 0.8, 0.2],  // Green
        metallic: 0.3,                // Slightly metallic
        roughness: 0.5,               // Medium roughness
        emissive: [0.0, 0.4, 0.0],    // Green emission
    });
    
    // Add torus to scene
    scene.add_mesh(glow_torus);
    
    // 4. Add a matte red plane as a base
    let mut base_plane = create_plane(10.0, 10.0, 1, 1);
    translate_mesh(&mut base_plane, Vec3::new(0.0, -1.0, 0.0));
    
    // Set material for the plane
    base_plane.with_material(Material {
        name: "MatteSurface".to_string(),
        base_color: [0.7, 0.2, 0.2],  // Red
        metallic: 0.0,                // Non-metallic
        roughness: 0.9,               // Very rough
        emissive: [0.0, 0.0, 0.0],    // No emission
    });
    
    // Add plane to scene
    scene.add_mesh(base_plane);
    
    // Export the scene with all objects and their materials
    scene.export_scene_glb("output/multi_material_scene.glb")?;
    println!("Exported scene with multiple materials to output/multi_material_scene.glb");
    println!("The scene contains:");
    println!("  - Gold metallic cube");
    println!("  - Glass-like blue sphere");
    println!("  - Glowing green torus");
    println!("  - Matte red plane");
    println!("\nEach object has its own distinct material properties.");
    
    Ok(())
}
