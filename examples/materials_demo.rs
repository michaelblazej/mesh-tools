use gltf_export::GltfBuilder;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Create a new glTF builder
    let mut builder = GltfBuilder::new();
    
    // Create materials for our box
    let red_material = builder.create_basic_material(
        Some("Red Material".to_string()),
        [1.0, 0.0, 0.0, 1.0], // Red color
    );
    
    let golden_material = builder.create_metallic_material(
        Some("Golden Material".to_string()),
        [1.0, 0.84, 0.0, 1.0],  // Gold color
        0.9,                    // High metallic factor
        0.1,                    // Low roughness factor (shiny)
    );
    
    // Create a box mesh with the red material
    let red_box_mesh = builder.create_box_with_material(1.0, Some(red_material));
    
    // Create a box mesh with the gold material
    let gold_box_mesh = builder.create_box_with_material(0.5, Some(golden_material));
    
    // Create nodes referencing the box meshes
    let red_box_node = builder.add_node(
        Some("RedBoxNode".to_string()),
        Some(red_box_mesh),
        Some([-1.5, 0.0, 0.0]),  // Left position
        None,                     // No rotation
        None,                     // Default scale
    );
    
    let gold_box_node = builder.add_node(
        Some("GoldBoxNode".to_string()),
        Some(gold_box_mesh),
        Some([1.5, 0.0, 0.0]),   // Right position
        None,                     // No rotation
        None,                     // Default scale
    );
    
    // Create a scene with both nodes
    builder.add_scene(
        Some("Scene".to_string()),
        Some(vec![red_box_node, gold_box_node]),
    );
    
    // Export the GLB file
    let output_path = "materials_demo.glb";
    builder.export_glb(output_path)?;
    
    println!("Successfully exported GLB file with materials: {}", output_path);
    
    Ok(())
}
