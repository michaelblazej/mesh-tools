use gltf_export::GltfBuilder;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Create a new glTF builder
    let mut builder = GltfBuilder::new();
    
    // Create materials for our shapes
    let red_material = builder.create_basic_material(
        Some("Red Material".to_string()),
        [1.0, 0.0, 0.0, 1.0], // Red color
    );
    
    let green_material = builder.create_basic_material(
        Some("Green Material".to_string()),
        [0.0, 1.0, 0.0, 1.0], // Green color
    );
    
    // Set the green material to be double-sided
    if let Some(materials) = &mut builder.gltf.materials {
        if let Some(material) = materials.get_mut(green_material) {
            material.doubleSided = Some(true);
        }
    }
    
    let blue_material = builder.create_basic_material(
        Some("Blue Material".to_string()),
        [0.0, 0.0, 1.0, 1.0], // Blue color
    );
    
    let gold_material = builder.create_metallic_material(
        Some("Gold Material".to_string()),
        [1.0, 0.84, 0.0, 1.0],  // Gold color
        0.9, // High metallic factor
        0.1, // Low roughness factor (shiny)
    );
    
    let purple_material = builder.create_basic_material(
        Some("Purple Material".to_string()),
        [0.5, 0.0, 0.5, 1.0], // Purple color
    );
    
    let cyan_material = builder.create_basic_material(
        Some("Cyan Material".to_string()),
        [0.0, 1.0, 1.0, 1.0], // Cyan color
    );
    
    // Create various primitive shapes
    
    // 1. Plane
    let plane_mesh = builder.create_plane(
        3.0,  // width
        3.0,  // depth
        10,   // width segments
        10,   // depth segments
        Some(green_material)
    );
    
    // 2. Sphere
    let sphere_mesh = builder.create_sphere(
        1.0,  // radius
        32,   // width segments
        16,   // height segments
        Some(blue_material)
    );
    
    // 3. Cylinder
    let cylinder_mesh = builder.create_cylinder(
        0.5,  // radius top
        0.5,  // radius bottom
        2.0,  // height
        32,   // radial segments
        2,    // height segments
        false, // not open ended (with caps)
        Some(red_material)
    );
    
    // 4. Cone
    let cone_mesh = builder.create_cone(
        0.5,  // radius
        2.0,  // height
        32,   // radial segments
        2,    // height segments
        false, // not open ended (with base)
        Some(gold_material)
    );
    
    // 5. Torus
    let torus_mesh = builder.create_torus(
        1.0,  // radius
        0.3,  // tube
        24,   // radial segments
        32,   // tubular segments
        Some(purple_material)
    );
    
    // 6. Icosahedron
    let icosahedron_mesh = builder.create_icosahedron(
        0.75, // radius
        Some(cyan_material)
    );
    
    // Position objects in the scene
    
    // Place the plane at the bottom
    let plane_node = builder.add_node(
        Some("PlaneNode".to_string()),
        Some(plane_mesh),
        Some([0.0, -2.0, 0.0]), // Lower position
        None,
        None,
    );
    
    // Place other shapes in a circle
    let sphere_node = builder.add_node(
        Some("SphereNode".to_string()),
        Some(sphere_mesh),
        Some([3.0, 0.0, 0.0]), // Right position
        None,
        None,
    );
    
    let cylinder_node = builder.add_node(
        Some("CylinderNode".to_string()),
        Some(cylinder_mesh),
        Some([1.5, 0.0, 2.6]), // Right-front position
        None,
        None,
    );
    
    let cone_node = builder.add_node(
        Some("ConeNode".to_string()),
        Some(cone_mesh),
        Some([-1.5, 0.0, 2.6]), // Left-front position
        None,
        None,
    );
    
    let torus_node = builder.add_node(
        Some("TorusNode".to_string()),
        Some(torus_mesh),
        Some([-3.0, 0.0, 0.0]), // Left position
        Some([0.0, 0.0, 0.0, 1.0]), // No rotation
        None,
    );
    
    let icosahedron_node = builder.add_node(
        Some("IcosahedronNode".to_string()),
        Some(icosahedron_mesh),
        Some([0.0, 0.0, -3.0]), // Back position
        None,
        None,
    );
    
    // Create a scene with all shapes
    builder.add_scene(
        Some("PrimitivesScene".to_string()),
        Some(vec![
            plane_node,
            sphere_node,
            cylinder_node,
            cone_node,
            torus_node,
            icosahedron_node,
        ]),
    );
    
    // Export the GLB file
    let output_path = "primitives_demo.glb";
    builder.export_glb(output_path)?;
    
    println!("Successfully exported GLB file with primitive shapes: {}", output_path);
    println!("");
    println!("This example demonstrates these primitive shapes:");
    println!("1. Plane (green)");
    println!("2. Sphere (blue)");
    println!("3. Cylinder (red)");
    println!("4. Cone (gold)");
    println!("5. Torus (purple)");
    println!("6. Icosahedron (cyan)");
    
    Ok(())
}
