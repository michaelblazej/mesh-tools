use mesh_tools::{
    Scene,
    primitives::{create_sphere, create_torus, create_cube, TorusParams},
    modifiers::translate_mesh,
    export::{ExportMesh, GlbExportOptions, Material, ExportScene},
};
use glam::Vec3;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create output directory if it doesn't exist
    std::fs::create_dir_all("output")?;

    // Example 1: Export with custom material properties
    let sphere = create_sphere(1.0, 32, 16);
    
    // Create a shiny red material
    let shiny_red = GlbExportOptions {
        name: "RedSphere".to_string(),
        export_normals: true,
        export_uvs: true,
        material: Material {
            name: "ShinyRedMaterial".to_string(),
            base_color: [0.9, 0.1, 0.1], // Red
            metallic: 0.7,               // Quite metallic
            roughness: 0.2,              // Smooth/shiny
            emissive: [0.0, 0.0, 0.0],   // No emission
        },
        ..Default::default()
    };
    
    sphere.export_glb_with_options("output/red_sphere.glb", shiny_red)?;
    println!("Exported sphere with custom red metallic material");
    
    // Example 2: Create a blue emissive torus
    let torus_params = TorusParams {
        radius: 1.0,
        tube_radius: 0.3,
        radial_segments: 32,
        tubular_segments: 24,
    };
    let torus = create_torus(torus_params);
    
    // Export with a glowing blue material
    let emissive_blue = GlbExportOptions {
        name: "GlowingTorus".to_string(),
        export_normals: true,
        export_uvs: true,
        material: Material {
            name: "EmissiveBlueMaterial".to_string(),
            base_color: [0.2, 0.2, 0.8], // Blue
            metallic: 0.0,               // Non-metallic
            roughness: 0.5,              // Medium roughness
            emissive: [0.0, 0.3, 0.8],   // Blue emission
        },
        ..Default::default()
    };
    
    torus.export_glb_with_options("output/glowing_torus.glb", emissive_blue)?;
    println!("Exported torus with emissive blue material");
    
    // Example 3: Create a scene with multiple objects, each with its own material
    println!("\nExample 3: Creating a scene with different materials");
    
    // Create a multi-material scene
    let mut multi_material_scene = Scene::new("MaterialShowcase");
    
    // Create and add a cube with metal material
    let mut cube = create_cube(1.0, 1.0, 1.0);
    translate_mesh(&mut cube, Vec3::new(2.0, 0.0, 0.0));
    
    // Set gold material for the cube
    cube.with_material(Material {
        name: "GoldMaterial".to_string(),
        base_color: [1.0, 0.85, 0.0], // Gold color
        metallic: 0.9,                // Very metallic
        roughness: 0.1,               // Very smooth
        emissive: [0.1, 0.1, 0.0],    // Slight emission
    });
    
    // Add cube to scene
    multi_material_scene.add_mesh(cube);
    
    // Create and add a sphere with glass-like material
    let mut sphere = create_sphere(0.8, 16, 8);
    translate_mesh(&mut sphere, Vec3::new(-2.0, 0.0, 0.0));
    
    // Set glass material for the sphere
    sphere.with_material(Material {
        name: "GlassMaterial".to_string(),
        base_color: [0.9, 0.9, 1.0],  // Slight blue tint
        metallic: 0.0,                // Non-metallic
        roughness: 0.0,               // Very smooth
        emissive: [0.0, 0.0, 0.0],    // No emission
    });
    
    // Add sphere to scene
    multi_material_scene.add_mesh(sphere);
    
    // Create and add a torus with glow material
    let mut torus = create_torus(TorusParams::default());
    translate_mesh(&mut torus, Vec3::new(0.0, 0.0, 2.0));
    
    // Set glowing material for the torus
    torus.with_material(Material {
        name: "GlowingGreenMaterial".to_string(),
        base_color: [0.2, 0.8, 0.2],  // Green
        metallic: 0.0,                // Non-metallic
        roughness: 0.5,               // Medium roughness
        emissive: [0.0, 0.6, 0.0],    // Green emission
    });
    
    // Add torus to scene
    multi_material_scene.add_mesh(torus);
    
    // Export the scene with multiple materials in a single GLB file
    multi_material_scene.export_scene_glb("output/multi_material_scene.glb")?;
    println!("Exported single GLB file with multiple objects and materials");
    println!("The scene contains:");
    println!("  - Gold cube (GoldMaterial)");
    println!("  - Glass-like sphere (GlassMaterial)");
    println!("  - Glowing green torus (GlowingGreenMaterial)");
    
    // Example 4: Export same model with different quality settings
    for (quality, segments) in [("low", 8), ("medium", 16), ("high", 32)] {
        let detailed_sphere = create_sphere(1.0, segments, segments / 2);
        println!("{} quality sphere: {} vertices, {} triangles",
                 quality, detailed_sphere.vertices.len(), detailed_sphere.triangles.len());
        detailed_sphere.export_glb(&format!("output/sphere_{}.glb", quality))?;
    }
    
    // Example 5: Create a simple height map terrain
    let mut terrain = mesh_tools::Mesh::new();
    let size = 4.0;
    let segments = 32;
    let segment_size = size / segments as f32;
    
    // Create a height map using a simple function
    let height_map = |x: f32, z: f32| -> f32 {
        let freq = 1.5;
        0.4 * ((x * freq).sin() * (z * freq).cos() + (x * freq * 0.5).cos() * (z * freq * 0.5).sin())
    };
    
    // Create grid of vertices
    let mut vertex_grid = vec![];
    for z in 0..=segments {
        for x in 0..=segments {
            let x_pos = -size / 2.0 + x as f32 * segment_size;
            let z_pos = -size / 2.0 + z as f32 * segment_size;
            
            // Calculate height using the height map function
            let y_pos = height_map(x_pos, z_pos);
            
            // Calculate texture coordinates
            let u = x as f32 / segments as f32;
            let v = z as f32 / segments as f32;
            
            let idx = terrain.add_vertex(mesh_tools::Vertex::with_uv(
                Vec3::new(x_pos, y_pos, z_pos),
                glam::Vec2::new(u, v),
            ));
            
            vertex_grid.push(idx);
        }
    }
    
    // Create triangles
    for z in 0..segments {
        for x in 0..segments {
            let stride = segments + 1;
            let idx = z * stride + x;
            
            let v0 = vertex_grid[idx as usize];
            let v1 = vertex_grid[(idx + 1) as usize];
            let v2 = vertex_grid[(idx + stride + 1) as usize];
            let v3 = vertex_grid[(idx + stride) as usize];
            
            // Add two triangles to form a quad
            terrain.add_triangle(v0, v1, v2)?;
            terrain.add_triangle(v0, v2, v3)?;
        }
    }
    
    // Calculate normals for the terrain
    terrain.calculate_normals();
    
    // Export with a green material for the terrain
    let terrain_material = GlbExportOptions {
        name: "TerrainMesh".to_string(),
        export_normals: true,
        export_uvs: true,
        material: Material {
            name: "TerrainMaterial".to_string(),
            base_color: [0.3, 0.8, 0.3], // Green
            metallic: 0.0,               // Non-metallic
            roughness: 0.8,              // Rough
            emissive: [0.0, 0.0, 0.0],   // No emission
        },
        ..Default::default()
    };
    
    terrain.export_glb_with_options("output/terrain.glb", terrain_material)?;
    println!("Created terrain mesh with {} vertices and {} triangles", 
             terrain.vertices.len(), terrain.triangles.len());
    
    println!("All examples exported to the 'output' directory");
    
    Ok(())
}
