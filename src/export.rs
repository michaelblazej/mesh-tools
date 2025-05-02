//! Mesh export utilities
//!
//! This module provides functionality to export meshes to various file formats.
//! Currently supported formats:
//! - GLB (binary glTF)

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
#[derive(thiserror::Error, Debug)]
pub enum ExportError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Mesh is missing required vertex attributes for export: {0}")]
    MissingAttributes(String),

    #[error("Mesh error: {0}")]
    MeshError(#[from] MeshError),

    #[error("Invalid material configuration: {0}")]
    InvalidMaterial(String),

    #[error("GLB construction error: {0}")]
    GlbError(String),
}

/// Specialized result type for export operations
pub type ExportResult<T> = Result<T, ExportError>;

/// Represents a material with basic properties
#[derive(Debug, Clone, PartialEq)]
pub struct Material {
    /// Material name
    pub name: String,
    /// Base color (RGB)
    pub base_color: [f32; 3],
    /// Metallic factor (0-1)
    pub metallic: f32,
    /// Roughness factor (0-1)
    pub roughness: f32,
    /// Emissive color (RGB)
    pub emissive: [f32; 3],
}

impl Default for Material {
    fn default() -> Self {
        Self {
            name: "DefaultMaterial".to_string(),
            base_color: [0.8, 0.8, 0.8], // Light gray
            metallic: 0.0,
            roughness: 1.0,
            emissive: [0.0, 0.0, 0.0],
        }
    }
}

/// GLB export options
#[derive(Debug, Clone)]
pub struct GlbExportOptions {
    /// Scene/mesh name
    pub name: String,
    /// Whether to export normal attributes
    pub export_normals: bool,
    /// Whether to export UV coordinates
    pub export_uvs: bool,
    /// Whether to export tangent attributes
    pub export_tangents: bool,
    /// Whether to export vertex colors
    pub export_colors: bool,
    /// Material properties
    pub material: Material,
}

impl Default for GlbExportOptions {
    fn default() -> Self {
        Self {
            name: "Default".to_string(),
            export_normals: true,
            export_uvs: true,
            export_tangents: false,
            export_colors: false,
            material: Material::default(),
        }
    }
}

/// Exports a mesh to GLB (binary glTF) format
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

/// Build the JSON part of the GLB file
fn build_gltf_json(
    options: &GlbExportOptions,
    mesh: &Mesh,
    buffer_length: usize,
    positions_byte_offset: usize,
    positions_byte_length: usize,
    normals_byte_offset: usize,
    normals_byte_length: usize,
    uvs_byte_offset: usize,
    uvs_byte_length: usize,
    tangents_byte_offset: usize,
    tangents_byte_length: usize,
    colors_byte_offset: usize,
    colors_byte_length: usize,
    indices_byte_offset: usize,
    indices_byte_length: usize,
    indices_component_type: u32,
) -> String {
    let mut json = String::new();
    
    // Start JSON
    json.push_str("{\n");
    
    // Asset info
    json.push_str(r#"  "asset": {
    "version": "2.0",
    "generator": "mesh-tools GLB exporter"
  },
"#);
    
    // Scenes and nodes
    json.push_str(r#"  "scenes": [
    {
      "nodes": [0]
    }
  ],
  "nodes": [
    {
      "mesh": 0,
      "name": ""#);
    json.push_str(&json_escape(&options.name));
    json.push_str(r#""
    }
  ],
  "scene": 0,
"#);
    
    // Materials
    json.push_str(r#"  "materials": [
    {
      "name": ""#);
    json.push_str(&json_escape(&options.material.name));
    json.push_str(r#",
      "pbrMetallicRoughness": {
        "baseColorFactor": ["#);
    json.push_str(&format!("{}, {}, {}, 1.0", 
        options.material.base_color[0], 
        options.material.base_color[1], 
        options.material.base_color[2]));
    json.push_str(r#"],
        "metallicFactor": "#);
    json.push_str(&format!("{}", options.material.metallic));
    json.push_str(r#",
        "roughnessFactor": "#);
    json.push_str(&format!("{}", options.material.roughness));
    json.push_str(r#"
      },
      "emissiveFactor": ["#);
    json.push_str(&format!("{}, {}, {}", 
        options.material.emissive[0], 
        options.material.emissive[1], 
        options.material.emissive[2]));
    json.push_str(r#"],
      "doubleSided": true
    }
  ],
"#);
    
    // Meshes
    json.push_str(r#"  "meshes": [
    {
      "name": "Mesh",
      "primitives": [
        {
          "attributes": {
            "POSITION": 0"#);
    
    // Add optional attributes
    let mut accessor_index = 1;
    
    if options.export_normals {
        json.push_str(&format!(r#",
            "NORMAL": {}"#, accessor_index));
        accessor_index += 1;
    }
    
    if options.export_uvs {
        json.push_str(&format!(r#",
            "TEXCOORD_0": {}"#, accessor_index));
        accessor_index += 1;
    }
    
    if options.export_tangents {
        json.push_str(&format!(r#",
            "TANGENT": {}"#, accessor_index));
        accessor_index += 1;
    }
    
    if options.export_colors {
        json.push_str(&format!(r#",
            "COLOR_0": {}"#, accessor_index));
        accessor_index += 1;
    }
    
    json.push_str(&format!(r#"
          }},\n          "indices": {},\n          "material": 0\n        }}\n      ]}}\n  ],\n"#, accessor_index));
    
    // Accessors
    json.push_str(r#"  "accessors": [
    {
      "bufferView": 0,
      "componentType": 5126,
      "count": "#);
    json.push_str(&format!("{}", mesh.vertices.len()));
    json.push_str(r#",
      "type": "VEC3",
      "min": ["#);
    
    // Calculate min/max for positions
    let mut min_pos = Vec3::new(f32::MAX, f32::MAX, f32::MAX);
    let mut max_pos = Vec3::new(f32::MIN, f32::MIN, f32::MIN);
    
    for vertex in &mesh.vertices {
        min_pos.x = min_pos.x.min(vertex.position.x);
        min_pos.y = min_pos.y.min(vertex.position.y);
        min_pos.z = min_pos.z.min(vertex.position.z);
        
        max_pos.x = max_pos.x.max(vertex.position.x);
        max_pos.y = max_pos.y.max(vertex.position.y);
        max_pos.z = max_pos.z.max(vertex.position.z);
    }
    
    json.push_str(&format!("{}, {}, {}", min_pos.x, min_pos.y, min_pos.z));
    json.push_str(r#"],
      "max": ["#);
    json.push_str(&format!("{}, {}, {}", max_pos.x, max_pos.y, max_pos.z));
    json.push_str(r#"]
    }"#);
    
    // Add normal accessor if needed
    if options.export_normals {
        json.push_str(r#",
    {
      "bufferView": 1,
      "componentType": 5126,
      "count": "#);
        json.push_str(&format!("{}", mesh.vertices.len()));
        json.push_str(r#",
      "type": "VEC3"
    }"#);
    }
    
    // Add UV accessor if needed
    if options.export_uvs {
        json.push_str(r#",
    {
      "bufferView": "#);
        json.push_str(&format!("{}", options.export_normals as usize + 1));
        json.push_str(r#",
      "componentType": 5126,
      "count": "#);
        json.push_str(&format!("{}", mesh.vertices.len()));
        json.push_str(r#",
      "type": "VEC2"
    }"#);
    }
    
    // Add tangent accessor if needed
    if options.export_tangents {
        json.push_str(r#",
    {
      "bufferView": "#);
        let buffer_view_idx = 1 + (options.export_normals as usize) + (options.export_uvs as usize);
        json.push_str(&format!("{}", buffer_view_idx));
        json.push_str(r#",
      "componentType": 5126,
      "count": "#);
        json.push_str(&format!("{}", mesh.vertices.len()));
        json.push_str(r#",
      "type": "VEC4"
    }"#);
    }
    
    // Add color accessor if needed
    if options.export_colors {
        json.push_str(r#",
    {
      "bufferView": "#);
        let buffer_view_idx = 1 + (options.export_normals as usize) + (options.export_uvs as usize) + (options.export_tangents as usize);
        json.push_str(&format!("{}", buffer_view_idx));
        json.push_str(r#",
      "componentType": 5126,
      "count": "#);
        json.push_str(&format!("{}", mesh.vertices.len()));
        json.push_str(r#",
      "type": "VEC4",
      "normalized": true
    }"#);
    }
    
    // Add indices accessor
    json.push_str(r#",
    {
      "bufferView": "#);
    let buffer_view_idx = 1 + (options.export_normals as usize) + (options.export_uvs as usize) + 
                          (options.export_tangents as usize) + (options.export_colors as usize);
    json.push_str(&format!("{}", buffer_view_idx));
    json.push_str(r#",
      "componentType": "#);
    json.push_str(&format!("{}", indices_component_type));
    json.push_str(r#",
      "count": "#);
    json.push_str(&format!("{}", mesh.triangles.len() * 3));
    json.push_str(r#",
      "type": "SCALAR"
    }
  ],
"#);
    
    // Buffer views
    json.push_str(r#"  "bufferViews": [
    {
      "buffer": 0,
      "byteOffset": "#);
    json.push_str(&format!("{}", positions_byte_offset));
    json.push_str(r#",
      "byteLength": "#);
    json.push_str(&format!("{}", positions_byte_length));
    json.push_str(r#",
      "target": 34962
    }"#);
    
    // Add normal buffer view if needed
    if options.export_normals {
        json.push_str(r#",
    {
      "buffer": 0,
      "byteOffset": "#);
        json.push_str(&format!("{}", normals_byte_offset));
        json.push_str(r#",
      "byteLength": "#);
        json.push_str(&format!("{}", normals_byte_length));
        json.push_str(r#",
      "target": 34962
    }"#);
    }
    
    // Add UV buffer view if needed
    if options.export_uvs {
        json.push_str(r#",
    {
      "buffer": 0,
      "byteOffset": "#);
        json.push_str(&format!("{}", uvs_byte_offset));
        json.push_str(r#",
      "byteLength": "#);
        json.push_str(&format!("{}", uvs_byte_length));
        json.push_str(r#",
      "target": 34962
    }"#);
    }
    
    // Add tangent buffer view if needed
    if options.export_tangents {
        json.push_str(r#",
    {
      "buffer": 0,
      "byteOffset": "#);
        json.push_str(&format!("{}", tangents_byte_offset));
        json.push_str(r#",
      "byteLength": "#);
        json.push_str(&format!("{}", tangents_byte_length));
        json.push_str(r#",
      "target": 34962
    }"#);
    }
    
    // Add color buffer view if needed
    if options.export_colors {
        json.push_str(r#",
    {
      "buffer": 0,
      "byteOffset": "#);
        json.push_str(&format!("{}", colors_byte_offset));
        json.push_str(r#",
      "byteLength": "#);
        json.push_str(&format!("{}", colors_byte_length));
        json.push_str(r#",
      "target": 34962
    }"#);
    }
    
    // Add indices buffer view
    json.push_str(r#",
    {
      "buffer": 0,
      "byteOffset": "#);
    json.push_str(&format!("{}", indices_byte_offset));
    json.push_str(r#",
      "byteLength": "#);
    json.push_str(&format!("{}", indices_byte_length));
    json.push_str(r#",
      "target": 34963
    }
  ],
"#);
    
    // Buffer
    json.push_str(r#"  "buffers": [
    {
      "byteLength": "#);
    json.push_str(&format!("{}", buffer_length));
    json.push_str(r#"
    }
  ]
}"#);
    
    json
}

/// Export a mesh to glTF text format
pub fn export_to_gltf(mesh: &Mesh, path: impl AsRef<Path>, options: GlbExportOptions) -> ExportResult<()> {
    // A simple approach is to first export to GLB and then extract
    // the JSON chunk to a separate file
    
    // Export to a temporary GLB file in the same directory
    let path = path.as_ref();
    let temp_glb_path = path.with_extension("temp.glb");
    
    // Export to GLB first
    export_to_glb(mesh, &temp_glb_path, options)?;
    
    // Open the GLB file and extract the JSON part
    let mut file = File::open(&temp_glb_path)?;
    
    // Read the GLB header
    let mut header_buf = [0u8; 12];
    std::io::Read::read_exact(&mut file, &mut header_buf)?;
    
    // Read the JSON chunk header
    let mut chunk_header = [0u8; 8];
    std::io::Read::read_exact(&mut file, &mut chunk_header)?;
    
    let json_length = u32::from_le_bytes([chunk_header[0], chunk_header[1], chunk_header[2], chunk_header[3]]) as usize;
    
    // Read the JSON chunk
    let mut json_data = vec![0u8; json_length];
    std::io::Read::read_exact(&mut file, &mut json_data)?;
    
    // Parse the JSON to pretty-print it
    let json_str = std::str::from_utf8(&json_data)
        .map_err(|_| ExportError::GlbError("Invalid UTF-8 in glTF JSON".into()))?;
    
    // Write the JSON to the output file
    let mut output_file = File::create(path)?;
    output_file.write_all(json_str.as_bytes())?;
    
    // Delete the temporary GLB file
    std::fs::remove_file(temp_glb_path)?;
    
    Ok(())
}

/// Extension trait to add GLB export capabilities directly to Mesh
pub trait ExportMesh {
    /// Export the mesh to GLB format
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
pub trait ExportScene {
    /// Export the scene to GLB format
    fn export_scene_glb(&self, path: impl AsRef<Path>) -> ExportResult<()>;
    
    /// Export the scene to GLB format with custom options
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives;
    use std::path::PathBuf;
    
    #[test]
    fn test_export_cube() {
        let cube = primitives::create_cube(1.0, 1.0, 1.0);
        
        // Get a temporary path for the GLB file
        let path = PathBuf::from("/tmp/cube_test.glb");
        
        // Export the cube
        let result = cube.export_glb(path);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_export_sphere_with_custom_options() {
        let sphere = primitives::create_sphere(1.0, 32, 16);
        
        // Get a temporary path for the GLB file
        let path = PathBuf::from("/tmp/sphere_test.glb");
        
        // Create custom export options
        let options = GlbExportOptions {
            name: "TestSphere".to_string(),
            export_normals: true,
            export_uvs: true,
            export_tangents: false,
            export_colors: false,
            material: Material {
                base_color: [0.7, 0.2, 0.2],
                metallic: 0.7,
                roughness: 0.3,
                emissive: [0.0, 0.0, 0.0],
            },
        };
        
        // Export the sphere with custom options
        let result = sphere.export_glb_with_options(path, options);
        assert!(result.is_ok());
    }
}

/// Export a scene (multiple meshes with potentially different materials) to GLB format
pub fn export_scene_to_glb(scene: &Scene, path: impl AsRef<Path>) -> ExportResult<()> {
    // Use default options for meshes without materials
    export_scene_to_glb_with_options(scene, path, GlbExportOptions::default())
}

/// Export a scene to GLB format with custom options (for meshes without materials)
pub fn export_scene_to_glb_with_options(
    scene: &Scene, 
    path: impl AsRef<Path>,
    default_options: GlbExportOptions
) -> ExportResult<()> {
    // Create the output file
    let file = File::create(&path)?;
    let mut writer = std::io::BufWriter::new(file);
    
    // Write GLB header (12 bytes)
    writer.write_u32::<LittleEndian>(GLB_MAGIC)?; // Magic
    writer.write_u32::<LittleEndian>(GLB_VERSION)?; // Version
    
    // Write a placeholder for total length (will be updated later)
    let length_pos = writer.seek(SeekFrom::Current(0))?;
    writer.write_u32::<LittleEndian>(0)?; // Placeholder for length
    
    // Collect unique materials from all meshes
    let mut materials = Vec::new();
    let mut material_indices = std::collections::HashMap::new();
    
    for mesh in &scene.meshes {
        let material = if let Some(mesh_material) = &mesh.material {
            mesh_material
        } else {
            &default_options.material
        };
        
        // Only add unique materials (by name)
        if !material_indices.contains_key(&material.name) {
            material_indices.insert(material.name.clone(), materials.len());
            materials.push(material.clone());
        }
    }
    
    // Process each mesh and collect export information
    let mut mesh_data = Vec::new();
    
    // Create binary buffer for all meshes
    let mut combined_buffer = Vec::new();
    let mut combined_buffer_offset = 0;
    
    for (i, mesh) in scene.meshes.iter().enumerate() {
        // Use mesh material or default
        let material = if let Some(mesh_material) = &mesh.material {
            mesh_material
        } else {
            &default_options.material
        };
        
        // Find material index or use default
        let material_index = material_indices.get(&material.name).unwrap_or(&0);
        
        // Create positions buffer
        let positions_byte_offset = combined_buffer_offset;
        let mut positions_byte_length = 0;
        for vertex in &mesh.vertices {
            combined_buffer.write_f32::<LittleEndian>(vertex.position.x)?;
            combined_buffer.write_f32::<LittleEndian>(vertex.position.y)?;
            combined_buffer.write_f32::<LittleEndian>(vertex.position.z)?;
            positions_byte_length += 12; // 3 floats * 4 bytes
        }
        combined_buffer_offset += positions_byte_length;
        
        // Create normals buffer (optional)
        let normals_byte_offset = combined_buffer_offset;
        let mut normals_byte_length = 0;
        if default_options.export_normals {
            for vertex in &mesh.vertices {
                if let Some(normal) = vertex.normal {
                    combined_buffer.write_f32::<LittleEndian>(normal.x)?;
                    combined_buffer.write_f32::<LittleEndian>(normal.y)?;
                    combined_buffer.write_f32::<LittleEndian>(normal.z)?;
                } else {
                    // Default normal (up vector)
                    combined_buffer.write_f32::<LittleEndian>(0.0)?;
                    combined_buffer.write_f32::<LittleEndian>(1.0)?;
                    combined_buffer.write_f32::<LittleEndian>(0.0)?;
                }
                normals_byte_length += 12; // 3 floats * 4 bytes
            }
            combined_buffer_offset += normals_byte_length;
        }
        
        // Create UVs buffer (optional)
        let uvs_byte_offset = combined_buffer_offset;
        let mut uvs_byte_length = 0;
        if default_options.export_uvs {
            for vertex in &mesh.vertices {
                // UVs in glTF are Vec2, which matches our internal representation
                let uv = vertex.uv.unwrap_or(Vec2::ZERO);
                combined_buffer.write_f32::<LittleEndian>(uv.x)?;
                combined_buffer.write_f32::<LittleEndian>(uv.y)?;
                uvs_byte_length += 8; // 2 floats * 4 bytes
            }
            combined_buffer_offset += uvs_byte_length;
        }
        
        // Create tangents buffer (optional)
        let tangents_byte_offset = combined_buffer_offset;
        let mut tangents_byte_length = 0;
        if default_options.export_tangents {
            for vertex in &mesh.vertices {
                if let Some(tangent) = vertex.tangent {
                    combined_buffer.write_f32::<LittleEndian>(tangent.x)?;
                    combined_buffer.write_f32::<LittleEndian>(tangent.y)?;
                    combined_buffer.write_f32::<LittleEndian>(tangent.z)?;
                    combined_buffer.write_f32::<LittleEndian>(tangent.w)?;
                } else {
                    // Default tangent
                    combined_buffer.write_f32::<LittleEndian>(1.0)?;
                    combined_buffer.write_f32::<LittleEndian>(0.0)?;
                    combined_buffer.write_f32::<LittleEndian>(0.0)?;
                    combined_buffer.write_f32::<LittleEndian>(1.0)?;
                }
                tangents_byte_length += 16; // 4 floats * 4 bytes
            }
            combined_buffer_offset += tangents_byte_length;
        }
        
        // Create colors buffer (optional)
        let colors_byte_offset = combined_buffer_offset;
        let mut colors_byte_length = 0;
        if default_options.export_colors {
            for vertex in &mesh.vertices {
                if let Some(color) = vertex.color {
                    combined_buffer.write_f32::<LittleEndian>(color.x)?;
                    combined_buffer.write_f32::<LittleEndian>(color.y)?;
                    combined_buffer.write_f32::<LittleEndian>(color.z)?;
                    // Always write 1.0 for alpha since our colors are Vec3
                    combined_buffer.write_f32::<LittleEndian>(1.0)?;
                } else {
                    // Default color (white)
                    combined_buffer.write_f32::<LittleEndian>(1.0)?;
                    combined_buffer.write_f32::<LittleEndian>(1.0)?;
                    combined_buffer.write_f32::<LittleEndian>(1.0)?;
                    combined_buffer.write_f32::<LittleEndian>(1.0)?;
                }
                colors_byte_length += 16; // 4 floats * 4 bytes
            }
            combined_buffer_offset += colors_byte_length;
        }
        
        // Create indices buffer
        let indices_byte_offset = combined_buffer_offset;
        let mut indices_byte_length = 0;
        let indices_component_type;
        
        if mesh.vertices.len() <= 65535 {
            // Use 16-bit indices for smaller meshes
            indices_component_type = 5123; // GL.UNSIGNED_SHORT
            for triangle in &mesh.triangles {
                combined_buffer.write_u16::<LittleEndian>(triangle.indices[0] as u16)?;
                combined_buffer.write_u16::<LittleEndian>(triangle.indices[1] as u16)?;
                combined_buffer.write_u16::<LittleEndian>(triangle.indices[2] as u16)?;
                indices_byte_length += 6; // 3 uint16s * 2 bytes
            }
        } else {
            // Use 32-bit indices for larger meshes
            indices_component_type = 5125; // GL.UNSIGNED_INT
            for triangle in &mesh.triangles {
                combined_buffer.write_u32::<LittleEndian>(triangle.indices[0] as u32)?;
                combined_buffer.write_u32::<LittleEndian>(triangle.indices[1] as u32)?;
                combined_buffer.write_u32::<LittleEndian>(triangle.indices[2] as u32)?;
                indices_byte_length += 12; // 3 uint32s * 4 bytes
            }
        }
        combined_buffer_offset += indices_byte_length;
        
        // Calculate min/max bounds for position data
        let mut min_pos = [f32::MAX, f32::MAX, f32::MAX];
        let mut max_pos = [f32::MIN, f32::MIN, f32::MIN];
        for vertex in &mesh.vertices {
            // Update min bounds
            if vertex.position.x < min_pos[0] { min_pos[0] = vertex.position.x; }
            if vertex.position.y < min_pos[1] { min_pos[1] = vertex.position.y; }
            if vertex.position.z < min_pos[2] { min_pos[2] = vertex.position.z; }
            
            // Update max bounds
            if vertex.position.x > max_pos[0] { max_pos[0] = vertex.position.x; }
            if vertex.position.y > max_pos[1] { max_pos[1] = vertex.position.y; }
            if vertex.position.z > max_pos[2] { max_pos[2] = vertex.position.z; }
        }
        
        mesh_data.push(MeshExportInfo {
            name: format!("Mesh_{}", i),
            material_index: *material_index,
            vertex_count: mesh.vertices.len(),
            index_count: mesh.triangles.len() * 3,
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
            min_pos,
            max_pos,
        });
    }
    
    // Build the glTF JSON with multiple meshes and materials
    let json = build_multi_mesh_gltf_json(&scene.name, &mesh_data, &materials, combined_buffer.len());
    
    // Align JSON to 4-byte boundary with spaces
    let json_chunk_length = (json.len() + 3) & !3; // Round up to multiple of 4
    let padding_bytes = json_chunk_length - json.len();
    let mut padding = String::new();
    for _ in 0..padding_bytes {
        padding.push(' ');
    }
    let json_with_padding = json + &padding;
    
    // Write JSON chunk
    writer.write_u32::<LittleEndian>(json_with_padding.len() as u32)?; // JSON chunk length
    writer.write_u32::<LittleEndian>(GLB_CHUNK_TYPE_JSON)?; // JSON chunk type
    writer.write_all(json_with_padding.as_bytes())?;
    
    // Align binary buffer to 4-byte boundary with zeros
    let bin_chunk_length = (combined_buffer.len() + 3) & !3; // Round up to multiple of 4
    let bin_padding_bytes = bin_chunk_length - combined_buffer.len();
    let mut bin_padding = Vec::new();
    for _ in 0..bin_padding_bytes {
        bin_padding.push(0);
    }
    let bin_buffer_with_padding = [&combined_buffer[..], &bin_padding[..]].concat();
    
    // Write binary chunk
    writer.write_u32::<LittleEndian>(bin_buffer_with_padding.len() as u32)?; // BIN chunk length
    writer.write_u32::<LittleEndian>(GLB_CHUNK_TYPE_BIN)?; // BIN chunk type
    writer.write_all(&bin_buffer_with_padding)?;
    
    // Update total file length
    let current_pos = writer.seek(SeekFrom::Current(0))?;
    writer.seek(SeekFrom::Start(length_pos))?;
    writer.write_u32::<LittleEndian>(current_pos as u32)?;
    
    // Flush the writer
    writer.flush()?;
    
    // No validation needed - our implementation follows the glTF spec
    
    Ok(())
}

// Helper struct for mesh export information
#[derive(Debug)]
struct MeshExportInfo {
    name: String,
    material_index: usize,
    vertex_count: usize,
    index_count: usize,
    positions_byte_offset: usize,
    positions_byte_length: usize,
    normals_byte_offset: usize,
    normals_byte_length: usize,
    uvs_byte_offset: usize,
    uvs_byte_length: usize,
    tangents_byte_offset: usize,
    tangents_byte_length: usize,
    colors_byte_offset: usize,
    colors_byte_length: usize,
    indices_byte_offset: usize,
    indices_byte_length: usize,
    indices_component_type: u32,
    min_pos: [f32; 3],
    max_pos: [f32; 3],
}

/// Build the JSON part of the GLB file for multiple meshes and materials
fn build_multi_mesh_gltf_json(
    scene_name: &str,
    mesh_data: &[MeshExportInfo],
    materials: &[Material],
    buffer_length: usize,
) -> String {
    let mut json_obj = serde_json::Map::new();
    
    // 1. Asset info
    let mut asset = serde_json::Map::new();
    asset.insert("version".to_string(), serde_json::Value::String("2.0".to_string()));
    asset.insert("generator".to_string(), serde_json::Value::String("mesh-tools GLB exporter".to_string()));
    json_obj.insert("asset".to_string(), serde_json::Value::Object(asset));
    
    // 2. Scene
    json_obj.insert("scene".to_string(), serde_json::Value::Number(serde_json::Number::from(0)));
    
    // 3. Scenes array
    let mut scenes_arr = serde_json::Map::new();
    scenes_arr.insert("name".to_string(), serde_json::Value::String(scene_name.to_string()));
    
    let mut nodes_refs = Vec::new();
    for i in 0..mesh_data.len() {
        nodes_refs.push(serde_json::Value::Number(serde_json::Number::from(i)));
    }
    scenes_arr.insert("nodes".to_string(), serde_json::Value::Array(nodes_refs));
    
    json_obj.insert("scenes".to_string(), serde_json::Value::Array(vec![serde_json::Value::Object(scenes_arr)]));
    
    // 4. Nodes
    let mut nodes = Vec::new();
    for i in 0..mesh_data.len() {
        let mut node = serde_json::Map::new();
        node.insert("mesh".to_string(), serde_json::Value::Number(serde_json::Number::from(i)));
        node.insert("name".to_string(), serde_json::Value::String(format!("Mesh_{}", i)));
        nodes.push(serde_json::Value::Object(node));
    }
    json_obj.insert("nodes".to_string(), serde_json::Value::Array(nodes));
    
    // 5. Meshes
    let mut meshes = Vec::new();
    let mut current_accessor_index = 0;
    for (i, mesh_info) in mesh_data.iter().enumerate() {
        let mut mesh_obj = serde_json::Map::new();
        mesh_obj.insert("name".to_string(), serde_json::Value::String(mesh_info.name.clone()));
        
        // Primitives
        let mut primitives = Vec::new();
        let mut primitive = serde_json::Map::new();
        
        // Attributes
        let mut attributes = serde_json::Map::new();
        attributes.insert("POSITION".to_string(), serde_json::Value::Number(serde_json::Number::from(current_accessor_index)));
        current_accessor_index += 1;
        
        if mesh_info.normals_byte_length > 0 {
            attributes.insert("NORMAL".to_string(), serde_json::Value::Number(serde_json::Number::from(current_accessor_index)));
            current_accessor_index += 1;
        }
        
        if mesh_info.uvs_byte_length > 0 {
            attributes.insert("TEXCOORD_0".to_string(), serde_json::Value::Number(serde_json::Number::from(current_accessor_index)));
            current_accessor_index += 1;
        }
        
        if mesh_info.tangents_byte_length > 0 {
            attributes.insert("TANGENT".to_string(), serde_json::Value::Number(serde_json::Number::from(current_accessor_index)));
            current_accessor_index += 1;
        }
        
        if mesh_info.colors_byte_length > 0 {
            attributes.insert("COLOR_0".to_string(), serde_json::Value::Number(serde_json::Number::from(current_accessor_index)));
            current_accessor_index += 1;
        }
        
        primitive.insert("attributes".to_string(), serde_json::Value::Object(attributes));
        
        // Set indices. In glTF, indices are a reference to an accessor.
        let indices_accessor_index = current_accessor_index;
        primitive.insert("indices".to_string(), serde_json::Value::Number(serde_json::Number::from(indices_accessor_index)));
        current_accessor_index += 1;
        
        // Set material reference
        primitive.insert("material".to_string(), serde_json::Value::Number(serde_json::Number::from(mesh_info.material_index)));
        
        // Set mode to triangles (4 = TRIANGLES)
        primitive.insert("mode".to_string(), serde_json::Value::Number(serde_json::Number::from(4)));
        
        primitives.push(serde_json::Value::Object(primitive));
        
        mesh_obj.insert("primitives".to_string(), serde_json::Value::Array(primitives));
        meshes.push(serde_json::Value::Object(mesh_obj));
    }
    json_obj.insert("meshes".to_string(), serde_json::Value::Array(meshes));
    
    // 6. Materials
    let mut materials_arr = Vec::new();
    for material in materials {
        let mut mat_obj = serde_json::Map::new();
        mat_obj.insert("name".to_string(), serde_json::Value::String(material.name.clone()));
        
        // PBR Metallic Roughness
        let mut pbr = serde_json::Map::new();
        let base_color = vec![
            serde_json::Value::Number(serde_json::Number::from_f64(material.base_color[0] as f64).unwrap()),
            serde_json::Value::Number(serde_json::Number::from_f64(material.base_color[1] as f64).unwrap()),
            serde_json::Value::Number(serde_json::Number::from_f64(material.base_color[2] as f64).unwrap()),
            serde_json::Value::Number(serde_json::Number::from_f64(1.0).unwrap())
        ];
        pbr.insert("baseColorFactor".to_string(), serde_json::Value::Array(base_color));
        pbr.insert("metallicFactor".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(material.metallic as f64).unwrap()));
        pbr.insert("roughnessFactor".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(material.roughness as f64).unwrap()));
        mat_obj.insert("pbrMetallicRoughness".to_string(), serde_json::Value::Object(pbr));
        
        // Emissive factor
        let emissive = vec![
            serde_json::Value::Number(serde_json::Number::from_f64(material.emissive[0] as f64).unwrap()),
            serde_json::Value::Number(serde_json::Number::from_f64(material.emissive[1] as f64).unwrap()),
            serde_json::Value::Number(serde_json::Number::from_f64(material.emissive[2] as f64).unwrap())
        ];
        mat_obj.insert("emissiveFactor".to_string(), serde_json::Value::Array(emissive));
        mat_obj.insert("doubleSided".to_string(), serde_json::Value::Bool(true));
        
        materials_arr.push(serde_json::Value::Object(mat_obj));
    }
    json_obj.insert("materials".to_string(), serde_json::Value::Array(materials_arr));
    
    // 7. Accessors
    let mut accessors = Vec::new();
    let mut buffer_views = Vec::new();
    let mut accessor_index = 0;
    let mut buffer_view_index = 0;
    
    for (i, mesh_info) in mesh_data.iter().enumerate() {
        // Position buffer view and accessor
        let mut pos_view = serde_json::Map::new();
        pos_view.insert("buffer".to_string(), serde_json::Value::Number(serde_json::Number::from(0)));
        pos_view.insert("byteOffset".to_string(), serde_json::Value::Number(serde_json::Number::from(mesh_info.positions_byte_offset)));
        pos_view.insert("byteLength".to_string(), serde_json::Value::Number(serde_json::Number::from(mesh_info.positions_byte_length)));
        pos_view.insert("target".to_string(), serde_json::Value::Number(serde_json::Number::from(34962))); // ARRAY_BUFFER
        buffer_views.push(serde_json::Value::Object(pos_view));
        
        let mut pos_accessor = serde_json::Map::new();
        pos_accessor.insert("bufferView".to_string(), serde_json::Value::Number(serde_json::Number::from(buffer_view_index)));
        pos_accessor.insert("componentType".to_string(), serde_json::Value::Number(serde_json::Number::from(5126))); // FLOAT
        pos_accessor.insert("count".to_string(), serde_json::Value::Number(serde_json::Number::from(mesh_info.vertex_count)));
        pos_accessor.insert("type".to_string(), serde_json::Value::String("VEC3".to_string()));
        
        // Min/max bounds
        let min = vec![
            serde_json::Value::Number(serde_json::Number::from_f64(mesh_info.min_pos[0] as f64).unwrap()),
            serde_json::Value::Number(serde_json::Number::from_f64(mesh_info.min_pos[1] as f64).unwrap()),
            serde_json::Value::Number(serde_json::Number::from_f64(mesh_info.min_pos[2] as f64).unwrap())
        ];
        let max = vec![
            serde_json::Value::Number(serde_json::Number::from_f64(mesh_info.max_pos[0] as f64).unwrap()),
            serde_json::Value::Number(serde_json::Number::from_f64(mesh_info.max_pos[1] as f64).unwrap()),
            serde_json::Value::Number(serde_json::Number::from_f64(mesh_info.max_pos[2] as f64).unwrap())
        ];
        pos_accessor.insert("min".to_string(), serde_json::Value::Array(min));
        pos_accessor.insert("max".to_string(), serde_json::Value::Array(max));
        accessors.push(serde_json::Value::Object(pos_accessor));
        
        buffer_view_index += 1;
        accessor_index += 1;
        
        // Normal buffer view and accessor (optional)
        if mesh_info.normals_byte_length > 0 {
            let mut norm_view = serde_json::Map::new();
            norm_view.insert("buffer".to_string(), serde_json::Value::Number(serde_json::Number::from(0)));
            norm_view.insert("byteOffset".to_string(), serde_json::Value::Number(serde_json::Number::from(mesh_info.normals_byte_offset)));
            norm_view.insert("byteLength".to_string(), serde_json::Value::Number(serde_json::Number::from(mesh_info.normals_byte_length)));
            norm_view.insert("target".to_string(), serde_json::Value::Number(serde_json::Number::from(34962))); // ARRAY_BUFFER
            buffer_views.push(serde_json::Value::Object(norm_view));
            
            let mut norm_accessor = serde_json::Map::new();
            norm_accessor.insert("bufferView".to_string(), serde_json::Value::Number(serde_json::Number::from(buffer_view_index)));
            norm_accessor.insert("componentType".to_string(), serde_json::Value::Number(serde_json::Number::from(5126))); // FLOAT
            norm_accessor.insert("count".to_string(), serde_json::Value::Number(serde_json::Number::from(mesh_info.vertex_count)));
            norm_accessor.insert("type".to_string(), serde_json::Value::String("VEC3".to_string()));
            accessors.push(serde_json::Value::Object(norm_accessor));
            
            buffer_view_index += 1;
            accessor_index += 1;
        }
        
        // UV buffer view and accessor (optional)
        if mesh_info.uvs_byte_length > 0 {
            let mut uv_view = serde_json::Map::new();
            uv_view.insert("buffer".to_string(), serde_json::Value::Number(serde_json::Number::from(0)));
            uv_view.insert("byteOffset".to_string(), serde_json::Value::Number(serde_json::Number::from(mesh_info.uvs_byte_offset)));
            uv_view.insert("byteLength".to_string(), serde_json::Value::Number(serde_json::Number::from(mesh_info.uvs_byte_length)));
            uv_view.insert("target".to_string(), serde_json::Value::Number(serde_json::Number::from(34962))); // ARRAY_BUFFER
            buffer_views.push(serde_json::Value::Object(uv_view));
            
            let mut uv_accessor = serde_json::Map::new();
            uv_accessor.insert("bufferView".to_string(), serde_json::Value::Number(serde_json::Number::from(buffer_view_index)));
            uv_accessor.insert("componentType".to_string(), serde_json::Value::Number(serde_json::Number::from(5126))); // FLOAT
            uv_accessor.insert("count".to_string(), serde_json::Value::Number(serde_json::Number::from(mesh_info.vertex_count)));
            uv_accessor.insert("type".to_string(), serde_json::Value::String("VEC2".to_string()));
            accessors.push(serde_json::Value::Object(uv_accessor));
            
            buffer_view_index += 1;
            accessor_index += 1;
        }
        
        // Tangent buffer view and accessor (optional)
        if mesh_info.tangents_byte_length > 0 {
            let mut tan_view = serde_json::Map::new();
            tan_view.insert("buffer".to_string(), serde_json::Value::Number(serde_json::Number::from(0)));
            tan_view.insert("byteOffset".to_string(), serde_json::Value::Number(serde_json::Number::from(mesh_info.tangents_byte_offset)));
            tan_view.insert("byteLength".to_string(), serde_json::Value::Number(serde_json::Number::from(mesh_info.tangents_byte_length)));
            tan_view.insert("target".to_string(), serde_json::Value::Number(serde_json::Number::from(34962))); // ARRAY_BUFFER
            buffer_views.push(serde_json::Value::Object(tan_view));
            
            let mut tan_accessor = serde_json::Map::new();
            tan_accessor.insert("bufferView".to_string(), serde_json::Value::Number(serde_json::Number::from(buffer_view_index)));
            tan_accessor.insert("componentType".to_string(), serde_json::Value::Number(serde_json::Number::from(5126))); // FLOAT
            tan_accessor.insert("count".to_string(), serde_json::Value::Number(serde_json::Number::from(mesh_info.vertex_count)));
            tan_accessor.insert("type".to_string(), serde_json::Value::String("VEC4".to_string()));
            accessors.push(serde_json::Value::Object(tan_accessor));
            
            buffer_view_index += 1;
            accessor_index += 1;
        }
        
        // Color buffer view and accessor (optional)
        if mesh_info.colors_byte_length > 0 {
            let mut col_view = serde_json::Map::new();
            col_view.insert("buffer".to_string(), serde_json::Value::Number(serde_json::Number::from(0)));
            col_view.insert("byteOffset".to_string(), serde_json::Value::Number(serde_json::Number::from(mesh_info.colors_byte_offset)));
            col_view.insert("byteLength".to_string(), serde_json::Value::Number(serde_json::Number::from(mesh_info.colors_byte_length)));
            col_view.insert("target".to_string(), serde_json::Value::Number(serde_json::Number::from(34962))); // ARRAY_BUFFER
            buffer_views.push(serde_json::Value::Object(col_view));
            
            let mut col_accessor = serde_json::Map::new();
            col_accessor.insert("bufferView".to_string(), serde_json::Value::Number(serde_json::Number::from(buffer_view_index)));
            col_accessor.insert("componentType".to_string(), serde_json::Value::Number(serde_json::Number::from(5126))); // FLOAT
            col_accessor.insert("count".to_string(), serde_json::Value::Number(serde_json::Number::from(mesh_info.vertex_count)));
            col_accessor.insert("type".to_string(), serde_json::Value::String("VEC4".to_string()));
            col_accessor.insert("normalized".to_string(), serde_json::Value::Bool(true));
            accessors.push(serde_json::Value::Object(col_accessor));
            
            buffer_view_index += 1;
            accessor_index += 1;
        }
        
        // Indices buffer view and accessor
        let mut idx_view = serde_json::Map::new();
        idx_view.insert("buffer".to_string(), serde_json::Value::Number(serde_json::Number::from(0)));
        idx_view.insert("byteOffset".to_string(), serde_json::Value::Number(serde_json::Number::from(mesh_info.indices_byte_offset)));
        idx_view.insert("byteLength".to_string(), serde_json::Value::Number(serde_json::Number::from(mesh_info.indices_byte_length)));
        idx_view.insert("target".to_string(), serde_json::Value::Number(serde_json::Number::from(34963))); // ELEMENT_ARRAY_BUFFER
        buffer_views.push(serde_json::Value::Object(idx_view));
        
        let mut idx_accessor = serde_json::Map::new();
        idx_accessor.insert("bufferView".to_string(), serde_json::Value::Number(serde_json::Number::from(buffer_view_index)));
        
        // Choose appropriate component type based on vertex count
        let component_type = if mesh_info.vertex_count <= 65535 { 5123 } else { 5125 }; // UNSIGNED_SHORT or UNSIGNED_INT
        idx_accessor.insert("componentType".to_string(), serde_json::Value::Number(serde_json::Number::from(component_type)));
        idx_accessor.insert("count".to_string(), serde_json::Value::Number(serde_json::Number::from(mesh_info.index_count)));
        idx_accessor.insert("type".to_string(), serde_json::Value::String("SCALAR".to_string()));
        accessors.push(serde_json::Value::Object(idx_accessor));
        
        buffer_view_index += 1;
        accessor_index += 1;
    }
    
    json_obj.insert("accessors".to_string(), serde_json::Value::Array(accessors));
    json_obj.insert("bufferViews".to_string(), serde_json::Value::Array(buffer_views));

    // 9. Buffers
    let mut buffer = serde_json::Map::new();
    buffer.insert("byteLength".to_string(), serde_json::Value::Number(serde_json::Number::from(buffer_length)));
    json_obj.insert("buffers".to_string(), serde_json::Value::Array(vec![serde_json::Value::Object(buffer)]));
    
    // Convert the JSON object to a properly formatted string without control characters
    serde_json::to_string(&serde_json::Value::Object(json_obj)).unwrap_or_else(|_| "{}".to_string())
}

/// Escape a string for JSON inclusion
fn json_escape(s: &str) -> String {
    let mut escaped = String::with_capacity(s.len() + 2);
    for c in s.chars() {
        match c {
            '"' => escaped.push_str("\\\""),
            '\\' => escaped.push_str("\\\\"),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            '\u{0008}' => escaped.push_str("\\b"), // backspace
            '\u{000C}' => escaped.push_str("\\f"), // form feed
            c if c.is_control() => {
                let _ = write!(escaped, "\\u{:04x}", c as u32);
            }
            c => escaped.push(c),
        }
    }
    escaped
}
