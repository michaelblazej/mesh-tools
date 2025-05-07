use mesh_tools::GltfBuilder;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Create a new glTF builder
    let mut builder = GltfBuilder::new();
    
    // Create a basic material for our custom mesh
    let red_material = builder.create_basic_material(
        Some("Red Material".to_string()),
        [1.0, 0.0, 0.0, 1.0], // Red color
    );
    
    // Create a textured material
    let checker_texture = builder.create_checkerboard_texture(
        512,                // width
        512,                // height
        64,                 // cell size
        [255, 255, 255],    // white
        [0, 0, 0],          // black
    )?;
    
    let checker_material = builder.create_textured_material(
        Some("Checker Material".to_string()),
        checker_texture
    );
    
    // Define a simple triangle mesh with custom UV mapping
    let positions = [
        // Position data for 3 vertices (triangle)
        -1.0, -1.0, 0.0,  // Vertex 0: bottom-left
         1.0, -1.0, 0.0,  // Vertex 1: bottom-right
         0.0,  1.0, 0.0,  // Vertex 2: top-center
    ];
    
    let indices = [
        0, 1, 2  // Single triangle
    ];
    
    let normals = [
        // Normal data for 3 vertices (all facing forward)
        0.0, 0.0, 1.0,
        0.0, 0.0, 1.0,
        0.0, 0.0, 1.0,
    ];
    
    let texcoords = [
        // UV data for 3 vertices
        0.0, 1.0,  // Vertex 0: bottom-left
        1.0, 1.0,  // Vertex 1: bottom-right
        0.5, 0.0,  // Vertex 2: top-center
    ];
    
    // Create the custom triangle mesh using create_simple_mesh (single UV channel)
    let triangle_mesh = builder.create_simple_mesh(
        Some("Triangle".to_string()),
        &positions,
        &indices,
        Some(&normals),
        Some(&texcoords),
        Some(checker_material)
    );
    
    // Define a quad mesh with multiple UV channels
    let quad_positions = [
        // Position data for 4 vertices (quad)
        -1.0, -1.0, 0.0,  // Vertex 0: bottom-left
         1.0, -1.0, 0.0,  // Vertex 1: bottom-right
         1.0,  1.0, 0.0,  // Vertex 2: top-right
        -1.0,  1.0, 0.0,  // Vertex 3: top-left
    ];
    
    let quad_indices = [
        // Two triangles forming a quad
        0, 1, 2,  // Triangle 1
        0, 2, 3   // Triangle 2
    ];
    
    let quad_normals = [
        // Normal data for 4 vertices (all facing forward)
        0.0, 0.0, 1.0,
        0.0, 0.0, 1.0,
        0.0, 0.0, 1.0,
        0.0, 0.0, 1.0,
    ];
    
    // Primary UV channel (TEXCOORD_0) - standard mapping
    let quad_texcoords_0 = vec![
        // Standard UV mapping (full texture)
        0.0, 1.0,  // Vertex 0: bottom-left
        1.0, 1.0,  // Vertex 1: bottom-right
        1.0, 0.0,  // Vertex 2: top-right
        0.0, 0.0,  // Vertex 3: top-left
    ];
    
    // Secondary UV channel (TEXCOORD_1) - tiled mapping
    let quad_texcoords_1 = vec![
        // Tiled UV mapping (repeated texture)
        0.0, 2.0,  // Vertex 0: bottom-left
        2.0, 2.0,  // Vertex 1: bottom-right
        2.0, 0.0,  // Vertex 2: top-right
        0.0, 0.0,  // Vertex 3: top-left
    ];
    
    // Create a textured material that uses the second UV set
    // Note: In a real application, this would be a different texture or map type
    // that's configured to use TEXCOORD_1 instead of TEXCOORD_0
    let uv_test_texture = builder.create_uv_test_texture(
        512,    // width
        512     // height
    )?;
    
    let uv_test_material = builder.create_textured_material(
        Some("UV Test Material".to_string()),
        uv_test_texture
    );
    
    // Pack multiple UV sets into a vector
    let texcoord_sets = Some(vec![quad_texcoords_0, quad_texcoords_1]);
    
    // Create the custom quad mesh using create_custom_mesh (multiple UV channels)
    let quad_mesh = builder.create_custom_mesh(
        Some("Quad".to_string()),
        &quad_positions,
        &quad_indices,
        Some(&quad_normals),
        texcoord_sets,
        Some(uv_test_material)
    );
    
    // Create a cube with fully custom UV mapping for each face
    let cube_positions = [
        // Front face - 4 vertices
        -1.0, -1.0,  1.0,  // 0: bottom-left
         1.0, -1.0,  1.0,  // 1: bottom-right
         1.0,  1.0,  1.0,  // 2: top-right
        -1.0,  1.0,  1.0,  // 3: top-left
        
        // Back face - 4 vertices
        -1.0, -1.0, -1.0,  // 4: bottom-left
         1.0, -1.0, -1.0,  // 5: bottom-right
         1.0,  1.0, -1.0,  // 6: top-right
        -1.0,  1.0, -1.0,  // 7: top-left
        
        // Top face - 4 vertices
        -1.0,  1.0, -1.0,  // 8: back-left
         1.0,  1.0, -1.0,  // 9: back-right
         1.0,  1.0,  1.0,  // 10: front-right
        -1.0,  1.0,  1.0,  // 11: front-left
        
        // Bottom face - 4 vertices
        -1.0, -1.0, -1.0,  // 12: back-left
         1.0, -1.0, -1.0,  // 13: back-right
         1.0, -1.0,  1.0,  // 14: front-right
        -1.0, -1.0,  1.0,  // 15: front-left
        
        // Right face - 4 vertices
         1.0, -1.0, -1.0,  // 16: bottom-back
         1.0,  1.0, -1.0,  // 17: top-back
         1.0,  1.0,  1.0,  // 18: top-front
         1.0, -1.0,  1.0,  // 19: bottom-front
        
        // Left face - 4 vertices
        -1.0, -1.0, -1.0,  // 20: bottom-back
        -1.0, -1.0,  1.0,  // 21: bottom-front
        -1.0,  1.0,  1.0,  // 22: top-front
        -1.0,  1.0, -1.0,  // 23: top-back
    ];
    
    let cube_indices = [
        // Front face
        0, 1, 2, 0, 2, 3,
        // Back face
        4, 7, 6, 4, 6, 5,
        // Top face
        8, 9, 10, 8, 10, 11,
        // Bottom face
        12, 15, 14, 12, 14, 13,
        // Right face
        16, 19, 18, 16, 18, 17,
        // Left face
        20, 23, 22, 20, 22, 21
    ];
    
    // Custom UV mapping where each face has a unique pattern
    let cube_texcoords = [
        // Front face - standard UVs
        0.0, 1.0,  1.0, 1.0,  1.0, 0.0,  0.0, 0.0,
        // Back face - flipped horizontally
        1.0, 1.0,  0.0, 1.0,  0.0, 0.0,  1.0, 0.0,
        // Top face - rotated 90 degrees
        0.0, 0.0,  0.0, 1.0,  1.0, 1.0,  1.0, 0.0,
        // Bottom face - centered smaller square
        0.25, 0.75,  0.75, 0.75,  0.75, 0.25,  0.25, 0.25,
        // Right face - tiled (2x2)
        0.0, 2.0,  0.0, 0.0,  2.0, 0.0,  2.0, 2.0,
        // Left face - showing just top-right quadrant
        0.5, 1.0,  1.0, 1.0,  1.0, 0.5,  0.5, 0.5
    ];
    
    // Create the custom cube mesh
    let cube_mesh = builder.create_simple_mesh(
        Some("CustomUVCube".to_string()),
        &cube_positions,
        &cube_indices,
        None, // No normals specified (could be calculated)
        Some(&cube_texcoords),
        Some(checker_material)
    );
    
    // Add the meshes to the scene
    
    // Position the triangle
    let triangle_node = builder.add_node(
        Some("TriangleNode".to_string()),
        Some(triangle_mesh),
        Some([-3.0, 0.0, 0.0]), // Left position
        None,
        None,
    );
    
    // Position the quad
    let quad_node = builder.add_node(
        Some("QuadNode".to_string()),
        Some(quad_mesh),
        Some([0.0, 0.0, 0.0]), // Center position
        None,
        None,
    );
    
    // Position the cube
    let cube_node = builder.add_node(
        Some("CubeNode".to_string()),
        Some(cube_mesh),
        Some([3.0, 0.0, 0.0]), // Right position
        None,
        None,
    );
    
    // Create a scene with all the custom meshes
    builder.add_scene(
        Some("CustomMeshScene".to_string()),
        Some(vec![triangle_node, quad_node, cube_node]),
    );
    
    // Export the GLB file
    let output_path = "custom_mesh_demo.glb";
    builder.export_glb(output_path)?;
    
    println!("Successfully exported GLB file with custom meshes: {}", output_path);
    println!("");
    println!("This example demonstrates:");
    println!("1. A triangle with simple custom UV mapping (using create_simple_mesh)");
    println!("2. A quad with multiple UV channels (using create_custom_mesh)");
    println!("3. A cube with unique UV mapping for each face (using create_simple_mesh)");
    println!("");
    println!("Note: Most glTF viewers only display the first UV channel (TEXCOORD_0).");
    println!("The second UV channel in the quad (TEXCOORD_1) is included to demonstrate");
    println!("the API's capability but would need a special material to utilize it.");
    
    Ok(())
}
