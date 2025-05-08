//! # mesh-tools: A Rust library for exporting glTF/GLB files
//!
//! This library provides a high-level API for creating and exporting 3D models in the
//! [glTF](https://www.khronos.org/gltf/) (GL Transmission Format) standard, which is a 
//! JSON-based format for 3D scenes and models, widely used for efficient transmission and loading 
//! of 3D content.
//!
//! ## Key Features
//! 
//! - Create and manipulate 3D geometry (vertices, normals, UVs, indices)
//! - Generate primitive shapes (boxes, spheres, planes, cylinders, etc.)
//! - Define materials with physically-based rendering (PBR) properties
//! - Support for textures and image data
//! - Create complex hierarchical scenes with node parent-child relationships
//! - Export models in both glTF (JSON+binary) and GLB (single binary) formats
//!
//! ## Basic Usage
//!
//! ```rust
//! use mesh_tools::GltfBuilder;
//!
//! // Create a new glTF builder
//! let mut builder = GltfBuilder::new();
//!
//! // Create a simple box mesh
//! let box_mesh = builder.create_box(1.0);
//!
//! // Add a node referencing the mesh
//! let node = builder.add_node(
//!     Some("Box".to_string()),
//!     Some(box_mesh),
//!     None, // Default position
//!     None, // Default rotation
//!     None, // Default scale
//! );
//!
//! // Create a scene containing the node
//! builder.add_scene(Some("Main Scene".to_string()), Some(vec![node]));
//!
//! // Export to GLB format
//! builder.export_glb("output.glb").unwrap();
//! ```
//!
//! See the examples directory for more complex usage scenarios.

// Public modules
pub mod texture;     // Texture and image handling
pub mod primitives;  // Geometry generation primitives
pub mod error;       // Error types and results
pub mod models;      // glTF data model definitions
pub mod constants;   // glTF format constants
pub mod material;    // Material creation and management
pub mod mesh;        // Mesh creation and manipulation
pub mod builder;     // Main GltfBuilder implementation

// Internal implementation modules
mod builder_primitives;  // Implementations for primitive shape creation
mod builder_texture;     // Implementations for texture handling
mod builder_material;    // Implementations for material handling

// Re-exports
pub use error::{GltfError, Result};
pub use models::*;
pub use builder::GltfBuilder;

// Constants re-exports
pub use constants::accessor_type;
pub use constants::buffer_view_target;
pub use constants::component_type;
pub use constants::sampler_filter;
pub use constants::sampler_wrap;
pub use constants::alpha_mode;
pub use constants::primitive_mode;
