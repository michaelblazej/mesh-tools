# mesh-tools

A Rust library for creating, manipulating, and exporting 3D meshes with support for the glTF 2.0 format (including binary GLB).

## Features

- Core mesh data structures for working with 3D geometry
- Primitive shape generators (box, plane, sphere, cylinder, cone, torus, icosahedron)
- Material creation with PBR properties (base color, metallic, roughness, emissive)
- Scene hierarchy and node transformation support
- Export to binary GLB format with Blender compatibility
- Comprehensive vertex attribute handling (positions, normals, UVs, tangents, colors)

## Primitive Shapes

The library provides generators for these common 3D shapes:

- Box/Cube with configurable dimensions
- Plane with width and depth segments
- Sphere with configurable radius and segments
- Cylinder with top/bottom radii, height, and segment options
- Cone (special case of cylinder with zero top radius)
- Torus with main radius and tube radius settings
- Icosahedron (20-sided polyhedron)

## Usage

### Creating a Simple Box

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

### Working with Materials

The library supports creating basic and PBR metallic materials:

```rust
// Create a basic colored material
let red_material = builder.create_basic_material(
    Some("Red Material".to_string()),
    [1.0, 0.0, 0.0, 1.0], // Red color
);

// Create a metallic material (e.g., gold)
let gold_material = builder.create_metallic_material(
    Some("Gold Material".to_string()),
    [1.0, 0.84, 0.0, 1.0],  // Gold color
    0.9, // High metallic factor
    0.1, // Low roughness factor (shiny)
);

// Apply material to a mesh
let sphere_mesh = builder.create_sphere(
    1.0,  // radius
    32,   // width segments
    16,   // height segments
    Some(gold_material)
);
```

## Mesh Export

The library provides GLB (binary glTF) export functionality that is compatible with Blender and other 3D software:

- Proper handling of chunk types and alignment
- Binary data padding and structure according to glTF spec
- Support for all vertex attributes (positions, normals, UVs, etc.)

## Building and Running

To build the library:

```bash
cargo build
```

To run one of the examples:

```bash
cargo run --example primitives_demo
```

## Examples

The library includes several examples demonstrating different features:

- `simple_box.rs`: Basic box creation and export
- `primitives_demo.rs`: All supported primitive shapes with different materials
- `materials_demo.rs`: Various material types and properties
- `texture_demo.rs`: Texture mapping and image handling
- `custom_mesh_demo.rs`: Creating custom meshes from vertex data
- `hierarchy_demo.rs`: Building scene hierarchies with multiple nodes

### Primitives Demo

Below is a screenshot of the primitives demo showing the various shapes with different materials that can be generated with the library:

![Primitives Demo](/docs/primitives_demo.png)

This image shows a plane (green), sphere (blue), cylinder (red), cone (gold), torus (purple), and icosahedron (cyan) arranged in a scene.

## glTF 2.0 Specification Coverage

The following table shows the current coverage of the [glTF 2.0 specification](https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html) features in this library:

| Feature Category | Feature | Support Level | Notes |
|-----------------|---------|---------------|-------|
| **Core** |||||
| | Asset Information | ✅ Full | Version, generator, copyright metadata |
| | Scenes | ✅ Full | Multiple scenes, default scenes |
| | Nodes | ✅ Full | Hierarchies, transforms (TRS) |
| | Buffers | ✅ Full | Binary data handling, GLB chunks |
| | Buffer Views | ✅ Full | Stride, target support |
| | Accessors | ✅ Full | All types and component types |
| **Geometry** |||||
| | Meshes | ✅ Full | Multiple primitives per mesh |
| | Primitive Types | ⚠️ Partial | Triangles only, no points or lines |
| | Morph Targets | ❌ None | Not yet implemented |
| **Materials** |||||
| | PBR Materials | ✅ Full | Base color, metallic, roughness |
| | Alpha Modes | ✅ Full | Opaque, mask, blend modes |
| | Double Sided | ✅ Full | Flag for double-sided rendering |
| | Material Variations | ⚠️ Partial | No specular workflow, only metallic |
| **Textures** |||||
| | Samplers | ✅ Full | Filter modes, wrap modes |
| | Images | ✅ Full | Embedded and external references |
| | Texture Coordinates | ✅ Full | Multiple UV sets |
| **Animation** |||||
| | Animation | ❌ None | Not yet implemented |
| | Skinning | ❌ None | Not yet implemented |
| **Cameras** |||||
| | Cameras | ❌ None | Not yet implemented |
| **Extensions** |||||
| | KHR_materials_unlit | ❌ None | Not yet implemented |
| | KHR_texture_transform | ❌ None | Not yet implemented |
| | KHR_mesh_quantization | ❌ None | Not yet implemented |
| | Custom Extensions | ❌ None | Not yet implemented |

### Legend

- ✅ **Full**: Complete implementation according to the spec
- ⚠️ **Partial**: Basic functionality implemented with some limitations
- ❌ **None**: Feature not implemented yet

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
