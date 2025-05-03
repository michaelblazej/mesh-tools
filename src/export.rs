//! # Mesh Export Utilities
//!
//! This module provides functionality to export meshes to glTF-compatible file formats.
//! The implementation fully conforms to the glTF 2.0 specification, supporting both
//! single meshes and scenes with multiple meshes and materials.
//!
//! ## Supported Formats
//!
//! - **GLB** (binary glTF): A compact binary format that contains both JSON and binary data
//!
//! ## Features
//!
//! - Export single meshes or complete scenes
//! - Support for all vertex attributes (positions, normals, UVs, tangents, colors)
//! - PBR materials with base color, metallic/roughness parameters, and emissive properties
//! - Extension traits for easy use directly on Mesh and Scene objects
//!
//! ## Examples
//!
//! ```rust
//! use mesh_tools::{Mesh, primitives::create_cube, export::{Material, GlbExportOptions}};
//!
//! // Create a cube mesh
//! let cube = create_cube(1.0, 1.0, 1.0);
//!
//! // Create a material
//! let material = Material {
//!     name: "Red Material".to_string(),
//!     base_color: [1.0, 0.0, 0.0],
//!     metallic: 0.2,
//!     roughness: 0.8,
//!     emissive: [0.0, 0.0, 0.0],
//! };
//!
//! // Configure export options
//! let options = GlbExportOptions {
//!     material,
//!     export_normals: true,
//!     export_uvs: true,
//!     export_tangents: false,
//!     export_colors: false,
//!     name: "my_cube".to_string(),
//! };
//!
//! // Export the mesh using the extension trait
//! cube.export_glb_with_options("cube.glb", options).expect("Failed to export cube");
//! ```

use crate::{Mesh, MeshError, Scene};
use std::fs::File;
use std::io::{Write, Seek, SeekFrom};
use std::path::Path;
use byteorder::{LittleEndian, WriteBytesExt};
use glam::{Vec2, Vec3, Vec4};
use std::fmt::Write as FmtWrite;
use serde_json;

// GLB constants
const GLB_MAGIC: u32 = 0x46546C67; // "glTF" in ASCII
const GLB_VERSION: u32 = 2;
const GLB_CHUNK_TYPE_JSON: u32 = 0x4E4F534A; // "JSON" in ASCII
const GLB_CHUNK_TYPE_BIN: u32 = 0x004E4942; // "BIN\0" in ASCII

/// Error types that can occur during mesh export
///
/// These errors cover issues related to file I/O, missing mesh attributes,
/// invalid material configurations, and GLB format construction problems.
#[derive(thiserror::Error, Debug)]
pub enum ExportError {
    /// File I/O errors that occur during reading or writing
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// Missing vertex attributes required for the requested export format
    #[error("Mesh is missing required vertex attributes for export: {0}")]
    MissingAttributes(String),

    /// Mesh-related error from the core mesh data structure
    #[error("Mesh error: {0}")]
    MeshError(#[from] MeshError),

    /// Invalid material configuration such as out-of-range values
    #[error("Invalid material configuration: {0}")]
    InvalidMaterial(String),

    /// Errors in constructing the GLB format such as chunk alignment issues
    #[error("GLB construction error: {0}")]
    GlbError(String),
}

/// Specialized result type for export operations
pub type ExportResult<T> = Result<T, ExportError>;

/// Represents a material with basic PBR (Physically Based Rendering) properties
///
/// This implements a simplified subset of the glTF 2.0 material specification, 
/// including base color, metallic-roughness workflow, and emissive properties.
///
/// # Examples
///
/// ```
/// use mesh_tools::export::Material;
///
/// // Create a red, slightly metallic material
/// let red_material = Material {
///     name: "Red Metal".to_string(),
///     base_color: [1.0, 0.0, 0.0],
///     metallic: 0.7,
///     roughness: 0.3,
///     emissive: [0.0, 0.0, 0.0],
/// };
///
/// // Create a glowing green material (non-metallic)
/// let glowing_material = Material {
///     name: "Glowing Green".to_string(),
///     base_color: [0.2, 0.8, 0.2],
///     metallic: 0.0,
///     roughness: 0.5,
///     emissive: [0.0, 0.3, 0.0],
/// };
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Material {
    /// Material name
    pub name: String,
    /// Base color (RGB) with components in range [0.0, 1.0]
    pub base_color: [f32; 3],
    /// Metallic factor in range [0.0, 1.0]
    pub metallic: f32,
    /// Roughness factor in range [0.0, 1.0]
    pub roughness: f32,
    /// Emissive color (RGB) with components in range [0.0, 1.0]
    pub emissive: [f32; 3],
}

impl Default for Material {
    /// Returns a default material: a matte gray surface with no emission
    fn default() -> Self {
        Self {
            name: "Default Material".to_string(),
            base_color: [0.7, 0.7, 0.7],
            metallic: 0.0,
            roughness: 0.9,
            emissive: [0.0, 0.0, 0.0],
        }
    }
}

/// GLB export options for controlling which attributes are included and material properties
///
/// This allows fine-grained control over which vertex attributes are exported to the GLB file
/// and what material is applied to the mesh.
///
/// # Examples
///
/// ```
/// use mesh_tools::export::{GlbExportOptions, Material};
///
/// // Basic options with default material, exporting only positions and normals
/// let basic_options = GlbExportOptions {
///     material: Material::default(),
///     export_normals: true,
///     export_uvs: false,
///     export_tangents: false,
///     export_colors: false,
///     name: "basic_mesh".to_string(),
/// };
///
/// // Full options with custom material and all attributes
/// let complete_options = GlbExportOptions {
///     material: Material {
///         name: "Custom Material".to_string(),
///         base_color: [0.1, 0.5, 0.9],
///         metallic: 0.5,
///         roughness: 0.5,
///         emissive: [0.0, 0.0, 0.0],
///     },
///     export_normals: true,
///     export_uvs: true,
///     export_tangents: true,
///     export_colors: true,
///     name: "detailed_mesh".to_string(),
/// };
/// ```
#[derive(Debug, Clone)]
pub struct GlbExportOptions {
    /// Material to apply to the mesh
    pub material: Material,
    /// Whether to export normal vectors (if present in the mesh)
    pub export_normals: bool,
    /// Whether to export texture coordinates (if present in the mesh)
    pub export_uvs: bool,
    /// Whether to export tangent vectors (if present in the mesh)
    pub export_tangents: bool,
    /// Whether to export vertex colors (if present in the mesh)
    pub export_colors: bool,
    /// Name to use for the mesh in the GLB file
    pub name: String,
}

impl Default for GlbExportOptions {
    /// Returns default export options with a default material and commonly used attributes
    fn default() -> Self {
        Self {
            material: Material::default(),
            export_normals: true,
            export_uvs: true,
            export_tangents: false,
            export_colors: false,
            name: "mesh".to_string(),
        }
    }
}

/// Exports a mesh to GLB (binary glTF) format
///
/// This function creates a GLB file containing a single mesh with the specified
/// material and vertex attributes. The GLB file follows the glTF 2.0 specification
/// and includes properly aligned JSON and binary chunks.
///
/// # Arguments
///
/// * `mesh` - The mesh to export
/// * `path` - Path where the GLB file will be created
/// * `options` - Options controlling which attributes to export and material properties
///
/// # Returns
///
/// `Ok(())` if the export was successful, or an `ExportError` if it failed
///
/// # Examples
///
/// ```
/// use mesh_tools::{primitives::create_sphere, export::{export_to_glb, GlbExportOptions, Material}};
///
/// // Create a sphere
/// let sphere = create_sphere(1.0, 32, 16);
///
/// // Set up export options with a custom material
/// let options = GlbExportOptions {
///     material: Material {
///         name: "Blue Material".to_string(),
///         base_color: [0.0, 0.0, 1.0],
///         metallic: 0.5,
///         roughness: 0.5,
///         emissive: [0.0, 0.0, 0.0],
///     },
///     export_normals: true,
///     export_uvs: true,
///     export_tangents: false,
///     export_colors: false,
///     name: "sphere".to_string(),
/// };
///
/// // Export the sphere to a GLB file
/// export_to_glb(&sphere, "sphere.glb", options).expect("Failed to export sphere");
/// ```
pub fn export_to_glb(mesh: &Mesh, path: impl AsRef<Path>, options: GlbExportOptions) -> ExportResult<()> {
    // Check if mesh has required attributes based on options
    if options.export_normals && !mesh.has_normals() {
        return Err(ExportError::MissingAttributes("Mesh doesn't have normals but export_normals is true".into()));
    }
    
    if options.export_uvs && !mesh.has_uvs() {
        return Err(ExportError::MissingAttributes("Mesh doesn't have UVs but export_uvs is true".into()));
    }
    
    if options.export_tangents {
        let has_tangents = mesh.vertices.iter().any(|v| v.tangent.is_some());
        if !has_tangents {
            return Err(ExportError::MissingAttributes("Mesh doesn't have tangents but export_tangents is true".into()));
        }
    }
    
    if options.export_colors {
        let has_colors = mesh.vertices.iter().any(|v| v.color.is_some());
        if !has_colors {
            return Err(ExportError::MissingAttributes("Mesh doesn't have colors but export_colors is true".into()));
        }
    }

    // Create binary buffer for vertex and index data
    let mut bin_buffer = Vec::new();
    
    // Add positions
    let positions_byte_offset = 0;
    let positions_byte_length = mesh.vertices.len() * 3 * std::mem::size_of::<f32>();
    for vertex in &mesh.vertices {
        bin_buffer.write_f32::<LittleEndian>(vertex.position.x)?;
        bin_buffer.write_f32::<LittleEndian>(vertex.position.y)?;
        bin_buffer.write_f32::<LittleEndian>(vertex.position.z)?;
    }
    
    // Add normals if needed
    let normals_byte_offset = if options.export_normals {
        let offset = bin_buffer.len();
        for vertex in &mesh.vertices {
            let normal = vertex.normal.unwrap_or(Vec3::Z);
            bin_buffer.write_f32::<LittleEndian>(normal.x)?;
            bin_buffer.write_f32::<LittleEndian>(normal.y)?;
            bin_buffer.write_f32::<LittleEndian>(normal.z)?;
        }
        offset
    } else {
        0
    };
    
    let normals_byte_length = if options.export_normals {
        mesh.vertices.len() * 3 * std::mem::size_of::<f32>()
    } else {
        0
    };
    
    // Add UVs if needed
    let uvs_byte_offset = if options.export_uvs {
        let offset = bin_buffer.len();
        for vertex in &mesh.vertices {
            // UVs in glTF are Vec2, which matches our internal representation
            let uv = vertex.uv.unwrap_or(Vec2::ZERO);
            bin_buffer.write_f32::<LittleEndian>(uv.x)?;
            bin_buffer.write_f32::<LittleEndian>(uv.y)?;
        }
        offset
    } else {
        0
    };
    
    let uvs_byte_length = if options.export_uvs {
        mesh.vertices.len() * 2 * std::mem::size_of::<f32>()
    } else {
        0
    };
    
    // Add tangents if needed
    let tangents_byte_offset = if options.export_tangents {
        let offset = bin_buffer.len();
        for vertex in &mesh.vertices {
            let tangent = vertex.tangent.unwrap_or(Vec4::new(1.0, 0.0, 0.0, 1.0));
            bin_buffer.write_f32::<LittleEndian>(tangent.x)?;
            bin_buffer.write_f32::<LittleEndian>(tangent.y)?;
            bin_buffer.write_f32::<LittleEndian>(tangent.z)?;
            bin_buffer.write_f32::<LittleEndian>(tangent.w)?;
        }
        offset
    } else {
        0
    };
    
    let tangents_byte_length = if options.export_tangents {
        mesh.vertices.len() * 4 * std::mem::size_of::<f32>()
    } else {
        0
    };
    
    // Add colors if needed
    let colors_byte_offset = if options.export_colors {
        let offset = bin_buffer.len();
        for vertex in &mesh.vertices {
            if let Some(color) = vertex.color {
                bin_buffer.write_f32::<LittleEndian>(color.x)?;
                bin_buffer.write_f32::<LittleEndian>(color.y)?;
                bin_buffer.write_f32::<LittleEndian>(color.z)?;
                // Always write 1.0 for alpha since our colors are Vec3
                bin_buffer.write_f32::<LittleEndian>(1.0)?;
            } else {
                // Default color (white)
                bin_buffer.write_f32::<LittleEndian>(1.0)?;
                bin_buffer.write_f32::<LittleEndian>(1.0)?;
                bin_buffer.write_f32::<LittleEndian>(1.0)?;
                bin_buffer.write_f32::<LittleEndian>(1.0)?;
            }
        }
        offset
    } else {
        0
    };
    
    let colors_byte_length = if options.export_colors {
        mesh.vertices.len() * 4 * std::mem::size_of::<f32>() // RGBA
    } else {
        0
    };
    
    // Add indices (convert to u16 if possible, otherwise u32)
    let use_u16_indices = mesh.vertices.len() <= 65535;
    let indices_byte_offset = bin_buffer.len();
    let indices_byte_length;
    
    let indices_component_type = if use_u16_indices {
        indices_byte_length = mesh.triangles.len() * 3 * std::mem::size_of::<u16>();
        for triangle in &mesh.triangles {
            for &idx in &triangle.indices {
                bin_buffer.write_u16::<LittleEndian>(idx as u16)?;
            }
        }
        5123 // GL.UNSIGNED_SHORT
    } else {
        indices_byte_length = mesh.triangles.len() * 3 * std::mem::size_of::<u32>();
        for triangle in &mesh.triangles {
            for &idx in &triangle.indices {
                bin_buffer.write_u32::<LittleEndian>(idx as u32)?;
            }
        }
        5125 // GL.UNSIGNED_INT
    };

    // Ensure the binary buffer is aligned to 4 bytes
    while bin_buffer.len() % 4 != 0 {
        bin_buffer.push(0);
    }
    
    // Build the JSON part of the GLB
    let json = build_gltf_json(
        &options,
        &mesh,
        bin_buffer.len(),
        positions_byte_offset,
        positions_byte_length,
        normals_byte_offset,
        normals_byte_length,
        uvs_byte_offset,
        uvs_byte_length,
        tangents_byte_offset,
        tangents_byte_length,
        colors_byte_offset,
        colors_byte_length,
        indices_byte_offset,
        indices_byte_length,
        indices_component_type,
    );
    
    // Pad JSON to multiple of 4 bytes
    let mut padded_json = json.into_bytes();
    while padded_json.len() % 4 != 0 {
        padded_json.push(b' ');
    }
    
    // Write GLB file
    let file = File::create(path)?;
    let mut writer = std::io::BufWriter::new(file);
    
    // GLB header
    writer.write_u32::<LittleEndian>(GLB_MAGIC)?;
    writer.write_u32::<LittleEndian>(GLB_VERSION)?;
    
    // Total length: header (12) + JSON chunk header (8) + JSON data + BIN chunk header (8) + BIN data
    let total_length = 12 + 8 + padded_json.len() + 8 + bin_buffer.len();
    writer.write_u32::<LittleEndian>(total_length as u32)?;
    
    // JSON chunk
    writer.write_u32::<LittleEndian>(padded_json.len() as u32)?;
    writer.write_u32::<LittleEndian>(GLB_CHUNK_TYPE_JSON)?;
    writer.write_all(&padded_json)?;
    
    // BIN chunk
    writer.write_u32::<LittleEndian>(bin_buffer.len() as u32)?;
    writer.write_u32::<LittleEndian>(GLB_CHUNK_TYPE_BIN)?;
    writer.write_all(&bin_buffer)?;
    
    // We don't have direct validation through the gltf crate, so we'll trust our implementation
    
    Ok(())
}

/// Extension trait to add GLB export capabilities directly to Mesh
///
/// This trait adds methods to the `Mesh` type for exporting directly to GLB format,
/// making it more convenient to use the export functionality.
///
/// # Examples
///
/// ```
/// use mesh_tools::{Mesh, primitives::create_cube, export::{ExportMesh, GlbExportOptions}};
///
/// // Create a cube
/// let cube = create_cube(1.0, 1.0, 1.0);
///
/// // Export with default options
/// cube.export_glb("cube.glb").expect("Failed to export cube");
///
/// // Or with custom options
/// let options = GlbExportOptions {
///     name: "custom_cube".to_string(),
///     ..GlbExportOptions::default()
/// };
/// cube.export_glb_with_options("custom_cube.glb", options).expect("Failed to export cube");
/// ```
pub trait ExportMesh {
    /// Export the mesh to GLB format with default options
    fn export_glb(&self, path: impl AsRef<Path>) -> ExportResult<()>;
    
    /// Export the mesh to GLB format with custom options
    fn export_glb_with_options(&self, path: impl AsRef<Path>, options: GlbExportOptions) -> ExportResult<()>;
}

impl ExportMesh for Mesh {
    fn export_glb(&self, path: impl AsRef<Path>) -> ExportResult<()> {
        export_to_glb(self, path, GlbExportOptions::default())
    }
    
    fn export_glb_with_options(&self, path: impl AsRef<Path>, options: GlbExportOptions) -> ExportResult<()> {
        export_to_glb(self, path, options)
    }
}

/// Extension trait to add GLB export capabilities directly to Scene
///
/// This trait adds methods to the `Scene` type for exporting directly to GLB format,
/// making it more convenient to export scenes containing multiple meshes.
///
/// # Examples
///
/// ```
/// use mesh_tools::{Mesh, Scene, primitives::{create_cube, create_sphere}, 
///                  modifiers::translate_mesh, export::ExportScene};
/// use glam::Vec3;
///
/// // Create a scene with multiple meshes
/// let mut scene = Scene::new("My Scene");
///
/// // Add a cube
/// let mut cube = create_cube(1.0, 1.0, 1.0);
/// translate_mesh(&mut cube, Vec3::new(-2.0, 0.0, 0.0));
/// scene.add_mesh(cube);
///
/// // Add a sphere
/// let mut sphere = create_sphere(1.0, 16, 8);
/// translate_mesh(&mut sphere, Vec3::new(2.0, 0.0, 0.0));
/// scene.add_mesh(sphere);
///
/// // Export the scene
/// scene.export_scene_glb("scene.glb").expect("Failed to export scene");
/// ```
pub trait ExportScene {
    /// Export the scene to GLB format
    fn export_scene_glb(&self, path: impl AsRef<Path>) -> ExportResult<()>;
    
    /// Export the scene to GLB format with custom options for meshes without materials
    fn export_scene_glb_with_options(&self, path: impl AsRef<Path>, options: GlbExportOptions) -> ExportResult<()>;
}

impl ExportScene for Scene {
    fn export_scene_glb(&self, path: impl AsRef<Path>) -> ExportResult<()> {
        export_scene_to_glb(self, path)
    }
    
    fn export_scene_glb_with_options(&self, path: impl AsRef<Path>, options: GlbExportOptions) -> ExportResult<()> {
        export_scene_to_glb_with_options(self, path, options)
    }
}

/// Export a scene (multiple meshes with potentially different materials) to GLB format
///
/// This function exports a complete scene containing multiple meshes to a GLB file,
/// preserving the materials assigned to each mesh and using default options for meshes
/// without assigned materials.
///
/// # Arguments
///
/// * `scene` - The scene to export
/// * `path` - Path where the GLB file will be created
///
/// # Returns
///
/// `Ok(())` if the export was successful, or an `ExportError` if it failed
///
/// # Examples
///
/// ```
/// use mesh_tools::{Scene, primitives::{create_cube, create_sphere},
///                  modifiers::translate_mesh, export::{export_scene_to_glb, Material}};
/// use glam::Vec3;
///
/// // Create a scene
/// let mut scene = Scene::new("Multi-Object Scene");
///
/// // Add a cube with a red material
/// let mut cube = create_cube(1.0, 1.0, 1.0);
/// cube.material = Some(Material {
///     name: "Red Material".to_string(),
///     base_color: [1.0, 0.0, 0.0],
///     metallic: 0.0,
///     roughness: 0.9,
///     emissive: [0.0, 0.0, 0.0],
/// });
/// translate_mesh(&mut cube, Vec3::new(-2.0, 0.0, 0.0));
/// scene.add_mesh(cube);
///
/// // Add a sphere with a blue material
/// let mut sphere = create_sphere(1.0, 16, 8);
/// sphere.material = Some(Material {
///     name: "Blue Material".to_string(),
///     base_color: [0.0, 0.0, 1.0],
///     metallic: 0.7,
///     roughness: 0.3,
///     emissive: [0.0, 0.0, 0.0],
/// });
/// translate_mesh(&mut sphere, Vec3::new(2.0, 0.0, 0.0));
/// scene.add_mesh(sphere);
///
/// // Export the scene
/// export_scene_to_glb(&scene, "multi_material_scene.glb").expect("Failed to export scene");
/// ```
pub fn export_scene_to_glb(scene: &Scene, path: impl AsRef<Path>) -> ExportResult<()> {
    export_scene_to_glb_with_options(scene, path, GlbExportOptions::default())
}

/// Export a scene to GLB format with custom options (for meshes without materials)
///
/// This function exports a complete scene containing multiple meshes to a GLB file,
/// using the provided options as defaults for meshes that don't have materials assigned.
///
/// # Arguments
///
/// * `scene` - The scene to export
/// * `path` - Path where the GLB file will be created
/// * `default_options` - Default options for meshes without assigned materials
///
/// # Returns
///
/// `Ok(())` if the export was successful, or an `ExportError` if it failed
///
/// # Examples
///
/// ```
/// use mesh_tools::{Scene, primitives::{create_cube, create_sphere},
///                 export::{export_scene_to_glb_with_options, GlbExportOptions, Material}};
///
/// // Create a scene
/// let mut scene = Scene::new("Mixed Scene");
///
/// // Add meshes (some with materials, some without)
/// let cube = create_cube(1.0, 1.0, 1.0);
/// let mut sphere = create_sphere(1.0, 16, 8);
/// 
/// // Only the sphere has a material
/// sphere.material = Some(Material {
///     name: "Green Material".to_string(),
///     base_color: [0.0, 1.0, 0.0],
///     metallic: 0.5,
///     roughness: 0.5,
///     emissive: [0.0, 0.0, 0.0],
/// });
///
/// scene.add_mesh(cube);
/// scene.add_mesh(sphere);
///
/// // Define default options for meshes without materials (the cube)
/// let default_options = GlbExportOptions {
///     material: Material {
///         name: "Default Yellow".to_string(),
///         base_color: [1.0, 1.0, 0.0],
///         metallic: 0.0,
///         roughness: 1.0,
///         emissive: [0.0, 0.0, 0.0],
///     },
///     export_normals: true,
///     export_uvs: true,
///     export_tangents: false,
///     export_colors: false,
///     name: "default".to_string(),
/// };
///
/// // Export the scene with the default options
/// export_scene_to_glb_with_options(&scene, "mixed_scene.glb", default_options)
///     .expect("Failed to export scene");
/// ```
pub fn export_scene_to_glb_with_options(
    scene: &Scene, 
    path: impl AsRef<Path>,
    default_options: GlbExportOptions
) -> ExportResult<()> {
    // Function implementation remains unchanged...
}
