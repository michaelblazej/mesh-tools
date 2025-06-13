use mesh_tools::GltfBuilder;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Create a new glTF builder
    let mut builder = GltfBuilder::new();
    
    // Create a variety of PBR materials using the add_material function
    
    // 1. Shiny blue metal
    let blue_metal = builder.add_material(
        Some("Blue Metal".to_string()),
        Some([0.1, 0.2, 0.8, 1.0]),  // Blue color
        Some(0.9),                    // Highly metallic
        Some(0.2),                    // Low roughness (shiny)
        Some(false)                   // Single-sided
    );
    
    // 2. Rough red plastic
    let red_plastic = builder.add_material(
        Some("Red Plastic".to_string()),
        Some([0.8, 0.1, 0.1, 1.0]),  // Red color
        Some(0.0),                    // Non-metallic
        Some(0.8),                    // High roughness
        Some(true)                    // Double-sided
    );
    
    // 3. Semi-glossy green plastic
    let green_plastic = builder.add_material(
        Some("Green Plastic".to_string()),
        Some([0.1, 0.8, 0.2, 1.0]),  // Green color
        Some(0.0),                    // Non-metallic
        Some(0.4),                    // Medium roughness
        None                          // Default single-sided
    );
    
    // 4. Gold metal
    let gold_metal = builder.add_material(
        Some("Gold Metal".to_string()),
        Some([1.0, 0.765, 0.0, 1.0]), // Gold color
        Some(1.0),                     // Fully metallic
        Some(0.1),                     // Very smooth
        None                           // Default single-sided
    );
    
    // 5. Brushed aluminum
    let brushed_aluminum = builder.add_material(
        Some("Brushed Aluminum".to_string()),
        Some([0.91, 0.92, 0.92, 1.0]), // Silver-like color
        Some(0.95),                     // Highly metallic
        Some(0.5),                      // Medium roughness
        None                            // Default single-sided
    );
    
    // Create different primitive shapes with the materials
    
    // Blue metal sphere
    let sphere = builder.create_sphere(1.0, 32, 16, Some(blue_metal));
    
    // Red plastic box
    let box_mesh = builder.create_box_with_material(1.0, Some(red_plastic));
    
    // Green plastic cylinder
    let cylinder = builder.create_cylinder(0.5, 0.5, 1.5, 32, 1, false, Some(green_plastic));
    
    // Gold metal torus
    let torus = builder.create_torus(0.7, 0.3, 32, 16, Some(gold_metal));
    
    // Brushed aluminum cone
    let cone = builder.create_cone(0.7, 1.5, 32, 1, false, Some(brushed_aluminum));
    
    // Arrange them in a circle
    let sphere_node = builder.add_node(
        Some("Sphere".to_string()),
        Some(sphere),
        Some([0.0, 0.0, -3.0]),  // North position
        None,
        None,
    );
    
    let box_node = builder.add_node(
        Some("Box".to_string()),
        Some(box_mesh),
        Some([3.0, 0.0, 0.0]),  // East position
        None,
        None,
    );
    
    let cylinder_node = builder.add_node(
        Some("Cylinder".to_string()),
        Some(cylinder),
        Some([0.0, 0.0, 3.0]),  // South position
        None,
        None,
    );
    
    let torus_node = builder.add_node(
        Some("Torus".to_string()),
        Some(torus),
        Some([-3.0, 0.0, 0.0]),  // West position
        None,
        None,
    );
    
    let cone_node = builder.add_node(
        Some("Cone".to_string()),
        Some(cone),
        Some([0.0, 0.0, 0.0]),  // Center position
        None,
        None,
    );
    
    // Create a scene with all nodes
    builder.add_scene(
        Some("PBR Materials Demo".to_string()),
        Some(vec![sphere_node, box_node, cylinder_node, torus_node, cone_node]),
    );
    
    // Export the GLB file
    let output_path = "pbr_materials_demo.glb";
    builder.export_glb(output_path)?;
    
    println!("Successfully exported GLB file with PBR materials: {}", output_path);
    
    Ok(())
}
