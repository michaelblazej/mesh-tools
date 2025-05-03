# mesh-tools

A Rust library for creating, manipulating, and exporting 3D mesh geometry.

[![Crates.io](https://img.shields.io/crates/v/mesh-tools.svg)](https://crates.io/crates/mesh-tools)
[![Documentation](https://docs.rs/mesh-tools/badge.svg)](https://docs.rs/mesh-tools)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Overview

`mesh-tools` is a comprehensive library for working with 3D mesh data in Rust. It provides intuitive APIs for procedural mesh generation, mesh manipulation, and export to industry-standard formats.

## Features

- **Core Mesh Types**: Flexible `Mesh` struct with support for vertices, triangles, normals, UVs, tangents, and colors
- **Primitive Generators**: Create common shapes like cubes, spheres, cylinders, cones, and more
- **Mesh Modifiers**: Transform, scale, rotate, bend, taper, subdivide, and more
- **Export Capabilities**: Export to GLB (binary glTF) format for use in 3D applications
- **Scene Management**: Group multiple meshes with materials into scenes

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
mesh-tools = "0.1.0"
```

## Quick Examples

### Creating a Simple Cube

```rust
use mesh_tools::{primitives::create_cube, export::ExportMesh};

fn main() {
    // Create a 2x2x2 cube
    let cube = create_cube(2.0, 2.0, 2.0);
    
    // Export to GLB format
    cube.export_glb("cube.glb").expect("Failed to export cube");
}
```

### Applying Modifiers

```rust
use mesh_tools::{
    primitives::create_cylinder,
    modifiers::{bend_mesh, BendParams},
    export::ExportMesh
};
use glam::Vec3;

fn main() {
    // Create a cylinder
    let mut cylinder = create_cylinder(0.5, 2.0, 32, 1);
    
    // Bend it along the Y axis
    let params = BendParams {
        angle: std::f32::consts::PI / 4.0, // 45 degrees
        axis: Vec3::Y,
        direction: Vec3::X,
        ..Default::default()
    };
    
    bend_mesh(&mut cylinder, &params);
    
    // Export the bent cylinder
    cylinder.export_glb("bent_cylinder.glb").expect("Failed to export");
}
```

### Creating and Exporting a Scene

```rust
use mesh_tools::{
    Scene,
    primitives::{create_sphere, create_cube},
    modifiers::translate_mesh,
    export::{ExportScene, Material}
};
use glam::Vec3;

fn main() {
    // Create a scene
    let mut scene = Scene::new("Example Scene");
    
    // Create a red sphere
    let mut sphere = create_sphere(1.0, 32, 16);
    sphere.material = Some(Material {
        name: "Red Material".to_string(),
        base_color: [1.0, 0.0, 0.0],
        metallic: 0.2,
        roughness: 0.8,
        emissive: [0.0, 0.0, 0.0],
    });
    
    // Create a blue cube
    let mut cube = create_cube(1.5, 1.5, 1.5);
    translate_mesh(&mut cube, Vec3::new(3.0, 0.0, 0.0));
    cube.material = Some(Material {
        name: "Blue Material".to_string(),
        base_color: [0.0, 0.0, 1.0],
        metallic: 0.5,
        roughness: 0.5,
        emissive: [0.0, 0.0, 0.0],
    });
    
    // Add meshes to scene
    scene.add_mesh(sphere);
    scene.add_mesh(cube);
    
    // Export the scene
    scene.export_scene_glb("example_scene.glb").expect("Failed to export scene");
}
```

## Procedural Mesh Generation

`mesh-tools` provides a robust set of primitive generators:

| Function | Description |
|----------|-------------|
| `create_cube` | Creates a cube with configurable width, height, and depth |
| `create_plane` | Creates a flat plane with configurable width, depth, and segments |
| `create_sphere` | Creates a UV sphere with configurable radius and resolution |
| `create_cylinder` | Creates a cylinder with configurable radius, height, and segments |
| `create_cone` | Creates a cone with configurable radius, height, and segments |
| `create_torus` | Creates a torus with configurable major and minor radii |
| `create_icosphere` | Creates an icosphere with configurable radius and subdivision level |

## Mesh Modification

The `modifiers` module contains functions for manipulating existing meshes:

| Function | Description |
|----------|-------------|
| `transform_mesh` | Apply an arbitrary 4x4 matrix transform |
| `scale_mesh` | Scale a mesh uniformly or non-uniformly |
| `rotate_mesh` | Rotate a mesh around an axis |
| `translate_mesh` | Move a mesh in 3D space |
| `flip_normals` | Invert the direction of all normals |
| `subdivide_mesh` | Increase mesh resolution |
| `generate_smooth_normals` | Generate smooth vertex normals |
| `bend_mesh` | Bend a mesh along an axis |
| `taper_mesh` | Taper a mesh along an axis |

## Export

`mesh-tools` supports exporting to industry-standard formats:

- **GLB/glTF**: Fully compliant with the glTF 2.0 specification
- **PBR Materials**: Support for base color, metallic/roughness workflow, and emissive properties
- **Scene Export**: Export multi-mesh scenes with different materials

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
