// Re-export the gltf-export library components
// Refactored for improved modularity and organization

// Public modules
pub mod texture;
pub mod primitives;
pub mod error;
pub mod models;
pub mod constants;
pub mod material;
pub mod mesh;
pub mod builder;

// Internal implementation modules
mod builder_primitives;
mod builder_texture;
mod builder_material;

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
