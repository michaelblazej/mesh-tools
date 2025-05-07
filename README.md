# glTF Exporter for Rust

A Rust library for exporting 3D models to the glTF 2.0 format (GLB binary variant), inspired by the Blender glTF exporter.

## Features

- Core glTF 2.0 data structures implemented in Rust
- Export meshes to binary GLB format
- Create simple primitives (currently supports box geometry)
- Proper handling of geometry data (vertices, indices, normals)

## Usage

```rust
use gltf_export::GltfBuilder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new glTF builder
    let mut builder = GltfBuilder::new();
    
    // Create a box mesh with a size of 1.0
    let box_mesh = builder.create_box(1.0);
    
    // Create a node referencing the box mesh
    let box_node = builder.add_node(
        Some("BoxNode".to_string()),
        Some(box_mesh),
        Some([0.0, 0.0, 0.0]),   // translation
        None,                     // rotation
        None,                     // scale
    );
    
    // Create a scene with the box node
    builder.add_scene(
        Some("Scene".to_string()),
        Some(vec![box_node]),
    );
    
    // Export the GLB file
    builder.export_glb("box.glb")?;
    
    println!("Successfully exported GLB file: box.glb");
    
    Ok(())
}
```

## Building and Running

To build the library:

```bash
cargo build
```

To run the example that creates a box:

```bash
cargo run
```

This will create a `box.glb` file in the current directory, which can be loaded in any glTF viewer or 3D application that supports the glTF format.

## Dependencies

- `serde` and `serde_json` - For JSON serialization
- `byteorder` - For binary data handling
- `base64` - For encoding binary data
- `thiserror` - For error handling
- `nalgebra` - For math operations (matrix/vector)

## Future Improvements

- Support for materials and textures
- Support for animations
- Support for more complex geometries
- Support for custom properties and extensions
- Importing glTF files into Rust objects

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
