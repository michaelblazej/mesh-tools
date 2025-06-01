//! # Compatibility layer for math types
//!
//! This module re-exports types from the `mint` crate and provides constructor functions 
//! to maintain compatibility with the previous nalgebra-based API. Users can directly 
//! use these types without adding mint as a direct dependency.
//!
//! ## Usage
//!
//! ```rust
//! use mesh_tools::compat::{Point3, Vector2, Vector3};
//! 
//! // Creating points and vectors
//! let position = mesh_tools::compat::point3::new(1.0, 2.0, 3.0);
//! let uv = mesh_tools::compat::vector2::new(0.5, 0.5);
//! let normal = mesh_tools::compat::vector3::new(0.0, 1.0, 0.0);
//! 
//! // Accessing components directly
//! let x = position.x;
//! let y = position.y;
//! let z = position.z;
//! 
//! // Using the module-level functions
//! use mesh_tools::compat::{point3_new, vector2_new, vector3_new};
//! let position = point3_new(1.0, 2.0, 3.0);
//! 
//! // Vector operations
//! use mesh_tools::compat::{cross, normalize, dot};
//! let a = vector3_new(1.0, 0.0, 0.0);
//! let b = vector3_new(0.0, 1.0, 0.0);
//! let cross_product = cross(a, b);
//! let unit_vector = normalize(cross_product);
//! let dot_product = dot(a, b);
//! ```

// Re-export mint types directly
pub use mint::{Point3, Vector2, Vector3};

/// Point3 creation and manipulation functions
pub mod point3 {
    use super::Point3;
    
    /// Creates a new 3D point
    pub fn new<T>(x: T, y: T, z: T) -> Point3<T> {
        Point3 { x, y, z }
    }
}

/// Vector2 creation and manipulation functions
pub mod vector2 {
    use super::Vector2;
    
    /// Creates a new 2D vector
    pub fn new<T>(x: T, y: T) -> Vector2<T> {
        Vector2 { x, y }
    }
}

/// Vector3 creation and manipulation functions
pub mod vector3 {
    use super::Vector3;
    
    /// Creates a new 3D vector
    pub fn new<T>(x: T, y: T, z: T) -> Vector3<T> {
        Vector3 { x, y, z }
    }
    
    /// Computes the cross product of two 3D vectors
    pub fn cross(a: Vector3<f32>, b: Vector3<f32>) -> Vector3<f32> {
        Vector3 {
            x: a.y * b.z - a.z * b.y,
            y: a.z * b.x - a.x * b.z,
            z: a.x * b.y - a.y * b.x,
        }
    }
    
    /// Normalizes a 3D vector to unit length
    pub fn normalize(v: Vector3<f32>) -> Vector3<f32> {
        let length_squared = v.x * v.x + v.y * v.y + v.z * v.z;
        if length_squared > 0.0 {
            let length = length_squared.sqrt();
            Vector3 {
                x: v.x / length,
                y: v.y / length,
                z: v.z / length,
            }
        } else {
            v
        }
    }
    
    /// Computes the dot product of two 3D vectors
    pub fn dot(a: Vector3<f32>, b: Vector3<f32>) -> f32 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }
}

// Re-export common functions at the module level for easier access
pub use point3::new as point3_new;
pub use vector2::new as vector2_new;
pub use vector3::new as vector3_new;
pub use vector3::cross;
pub use vector3::normalize;
pub use vector3::dot;
