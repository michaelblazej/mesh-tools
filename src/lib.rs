//! Mesh tools library for generating and manipulating 3D meshes
//!
//! This library provides:
//!  - Core mesh data structures for 3D vertices, triangles, and meshes
//!  - Primitive shape generators (cubes, spheres, etc.)
//!  - glTF import/export functionality
//!  - Utility functions for mesh manipulation

// Re-export core data structures
pub use mesh::{Edge, Mesh, MeshError, Triangle, Vertex};

// Re-export glTF functionality
pub use gltf::{
    export_to_glb, export_to_glb_with_options, export_to_gltf, export_to_gltf_with_options,
    ExportMesh, GlbExportOptions,
};

// Include modules
pub mod gltf;
pub mod mesh;

// Add primitives module stub for future implementation
pub mod primitives {
    //! Primitive shape generators for creating basic 3D shapes
    //!
    //! This module provides functions to create primitive shapes like
    //! cubes, spheres, cones, cylinders, tori, planes, and ico-spheres.
    
    // Placeholder for future implementation
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::{Vec2, Vec3, Vec4};
    
    #[test]
    fn test_mesh_construction() {
        let mut mesh = Mesh::new();
        
        // Add vertices
        let v0 = mesh.add_vertex(Vertex::new(Vec3::new(0.0, 0.0, 0.0)));
        let v1 = mesh.add_vertex(Vertex::new(Vec3::new(1.0, 0.0, 0.0)));
        let v2 = mesh.add_vertex(Vertex::new(Vec3::new(0.0, 1.0, 0.0)));
        
        // Add triangle
        mesh.add_triangle(Triangle::new(v0, v1, v2)).unwrap();
        
        // Verify
        assert_eq!(mesh.vertex_count(), 3);
        assert_eq!(mesh.triangle_count(), 1);
    }
    
    #[test]
    fn test_mesh_normals() {
        let mut mesh = Mesh::new();
        
        // Add vertices for a triangle in xy plane
        let v0 = mesh.add_vertex(Vertex::new(Vec3::new(0.0, 0.0, 0.0)));
        let v1 = mesh.add_vertex(Vertex::new(Vec3::new(1.0, 0.0, 0.0)));
        let v2 = mesh.add_vertex(Vertex::new(Vec3::new(0.0, 1.0, 0.0)));
        
        // Add triangle
        mesh.add_triangle(Triangle::new(v0, v1, v2)).unwrap();
        
        // Calculate normals
        mesh.calculate_normals();
        
        // Verify normal points in +z direction
        let expected_normal = Vec3::new(0.0, 0.0, 1.0);
        assert!((mesh.vertices[0].normal.unwrap() - expected_normal).length() < 0.001);
    }
}
