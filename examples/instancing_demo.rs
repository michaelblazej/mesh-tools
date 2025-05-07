use mesh_tools::GltfBuilder;
use std::error::Error;
use std::f32::consts::PI;

fn main() -> Result<(), Box<dyn Error>> {
    // Create a new glTF builder
    let mut builder = GltfBuilder::new();
    
    // Create some materials for our instances
    let blue_material = builder.create_basic_material(
        Some("Blue Material".to_string()),
        [0.0, 0.0, 1.0, 1.0], // Blue color
    );
    
    let red_material = builder.create_basic_material(
        Some("Red Material".to_string()),
        [1.0, 0.0, 0.0, 1.0], // Red color
    );
    
    let gold_material = builder.create_metallic_material(
        Some("Gold Material".to_string()),
        [1.0, 0.84, 0.0, 1.0],  // Gold color
        0.9, // High metallic factor
        0.1, // Low roughness factor (shiny)
    );
    
    let green_material = builder.create_basic_material(
        Some("Green Material".to_string()),
        [0.0, 1.0, 0.0, 1.0], // Green color
    );
    
    // Create basic meshes with different materials that will be instanced multiple times
    // We'll use a torus for this example as it has a distinctive shape
    let blue_torus = builder.create_torus(
        0.5,  // radius
        0.2,  // tube
        16,   // radial segments
        24,   // tubular segments
        Some(blue_material)
    );
    
    let red_torus = builder.create_torus(
        0.5,  // radius
        0.2,  // tube
        16,   // radial segments
        24,   // tubular segments
        Some(red_material)
    );
    
    let gold_torus = builder.create_torus(
        0.5,  // radius
        0.2,  // tube
        16,   // radial segments
        24,   // tubular segments
        Some(gold_material)
    );
    
    let green_torus = builder.create_torus(
        0.5,  // radius
        0.2,  // tube
        16,   // radial segments
        24,   // tubular segments
        Some(green_material)
    );
    
    // Create a grid of instances, arranged in rows and columns
    let rows = 5;
    let columns = 5;
    let spacing = 1.5;
    
    let mut instance_nodes = Vec::new();
    
    // Create instances in a grid pattern
    for row in 0..rows {
        for col in 0..columns {
            // Calculate position
            let x = (col as f32 - (columns as f32 / 2.0)) * spacing;
            let z = (row as f32 - (rows as f32 / 2.0)) * spacing;
            
            // Determine which mesh to use based on position
            let torus_mesh = if (row + col) % 4 == 0 {
                blue_torus
            } else if (row + col) % 4 == 1 {
                red_torus
            } else if (row + col) % 4 == 2 {
                gold_torus
            } else {
                green_torus
            };
            
            // Calculate a unique rotation for each instance
            let rotation = [
                0.0, 
                ((row as f32 * 0.2) + (col as f32 * 0.3)) * PI, 
                0.0, 
                1.0
            ];
            
            // Calculate a scale variation for each instance
            let scale_factor = 0.6 + (((row + col) % 3) as f32 * 0.2);
            let scale = [scale_factor, scale_factor, scale_factor];
            
            // Create a node with the instanced mesh
            let instance_node = builder.add_node(
                Some(format!("TorusInstance_{}_{}", row, col)),
                Some(torus_mesh),
                Some([x, 0.0, z]),    // position
                Some(rotation),       // rotation
                Some(scale),          // scale
            );
            
            instance_nodes.push(instance_node);
        }
    }
    
    // Create a second example: a ring of cylinders all using the same mesh
    // This demonstrates true instancing - multiple nodes referencing the same mesh
    let cylinder_mesh = builder.create_cylinder(
        0.1,   // radius top
        0.1,   // radius bottom
        1.0,   // height
        12,    // radial segments
        1,     // height segments
        false, // not open ended
        Some(blue_material)
    );
    
    // Create a ring of cylinders around the center
    let cylinder_count = 12;
    let radius = 8.0;
    
    for i in 0..cylinder_count {
        // Calculate position on a circle
        let angle = (i as f32 / cylinder_count as f32) * (2.0 * PI);
        let x = radius * angle.cos();
        let z = radius * angle.sin();
        
        // Make cylinders point to the center
        let rotation = [0.0, angle + (PI / 2.0), 0.0, 1.0];
        
        // Create a node with the instanced cylinder mesh
        // All these nodes reference the same mesh but with different transformations
        let cylinder_node = builder.add_node(
            Some(format!("CylinderInstance_{}", i)),
            Some(cylinder_mesh),
            Some([x, 0.0, z]),  // position
            Some(rotation),     // rotation
            None,               // use default scale
        );
        
        instance_nodes.push(cylinder_node);
    }
    
    // Add a camera viewpoint
    let viewpoint = builder.add_node(
        Some("Viewpoint".to_string()),
        None, // No mesh for viewpoint
        Some([0.0, 10.0, 12.0]), // Position for a good view
        Some([-0.4, 0.0, 0.0, 0.92]), // Rotation quaternion looking down at scene
        None,
    );
    
    // Create a scene with all our instances
    builder.add_scene(
        Some("Instancing Demo".to_string()),
        Some(instance_nodes),
    );
    
    // Create a default scene that includes the viewpoint
    builder.add_scene(
        Some("Camera View".to_string()),
        Some(vec![viewpoint]),
    );
    
    // Set the default scene to be the first scene
    if let Some(scenes) = &mut builder.gltf.scenes {
        if scenes.len() > 0 {
            builder.gltf.scene = Some(0);
        }
    }
    
    // Export the GLB file
    let output_path = "instancing_demo.glb";
    builder.export_glb(output_path)?;
    
    println!("Successfully exported instancing demo: {}", output_path);
    println!("");
    println!("This example demonstrates mesh instancing with:");
    println!("1. A 5x5 grid of torus instances with:");
    println!("   - Different colors (blue, red, gold, green)");
    println!("   - Different rotations");
    println!("   - Different scales");
    println!("2. A ring of 12 cylinder instances all using the same mesh:");
    println!("   - Arranged in a circle");
    println!("   - Rotated to point toward the center");
    println!("");
    println!("This example shows two different approaches to instancing:");
    println!("1. Creating separate meshes with different materials for the torus grid");
    println!("2. True instancing by reusing the same cylinder mesh multiple times");
    println!("   with different transforms but the same material.");
    
    Ok(())
}
