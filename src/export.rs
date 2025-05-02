//! Mesh export utilities
//!
//! This module provides functionality to export meshes to various file formats.
//! Currently supported formats:
//! - GLB (binary glTF)

use crate::{Mesh, MeshError, MeshResult};
use glam::{Vec2, Vec3, Vec4};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use byteorder::{LittleEndian, WriteBytesExt};

/// Error types that can occur during mesh export
#[derive(thiserror::Error, Debug)]
pub enum ExportError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Mesh error: {0}")]
    MeshError(#[from] MeshError),

    #[error("Invalid mesh data: {0}")]
    InvalidData(String),

    #[error("Buffer size exceeds GLB limit")]
    BufferSizeExceeded,
}

/// Specialized result type for export operations
pub type ExportResult<T> = Result<T, ExportError>;

/// Represents a material with basic properties
#[derive(Debug, Clone)]
pub struct Material {
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
            base_color: [0.8, 0.8, 0.8],
            metallic: 0.0,
            roughness: 0.5,
            emissive: [0.0, 0.0, 0.0],
        }
    }
}

/// GLB export options
#[derive(Debug, Clone)]
pub struct GlbExportOptions {
    /// Name of the model
    pub name: String,
    /// Whether to include normals
    pub export_normals: bool,
    /// Whether to include UVs
    pub export_uvs: bool,
    /// Whether to include tangents
    pub export_tangents: bool,
    /// Whether to include colors
    pub export_colors: bool,
    /// Material to use
    pub material: Material,
}

impl Default for GlbExportOptions {
    fn default() -> Self {
        Self {
            name: "MeshModel".to_string(),
            export_normals: true,
            export_uvs: true,
            export_tangents: false,
            export_colors: false,
            material: Material::default(),
        }
    }
}

// GLB constants
const GLB_MAGIC: u32 = 0x46546C67; // "glTF" in ASCII
const GLB_VERSION: u32 = 2;
const GLB_CHUNK_TYPE_JSON: u32 = 0x4E4F534A; // "JSON" in ASCII
const GLB_CHUNK_TYPE_BIN: u32 = 0x004E4942; // "BIN\0" in ASCII

/// Exports a mesh to GLB (binary glTF) format
pub fn export_to_glb(mesh: &Mesh, path: impl AsRef<Path>, options: GlbExportOptions) -> ExportResult<()> {
    // Check if mesh has required attributes based on options
    if options.export_normals && !mesh.has_normals() {
        return Err(ExportError::InvalidData("Mesh doesn't have normals but export_normals is true".into()));
    }
    
    if options.export_uvs && !mesh.has_uvs() {
        return Err(ExportError::InvalidData("Mesh doesn't have UVs but export_uvs is true".into()));
    }
    
    if options.export_tangents {
        let has_tangents = mesh.vertices.iter().any(|v| v.tangent.is_some());
        if !has_tangents {
            return Err(ExportError::InvalidData("Mesh doesn't have tangents but export_tangents is true".into()));
        }
    }
    
    if options.export_colors {
        let has_colors = mesh.vertices.iter().any(|v| v.color.is_some());
        if !has_colors {
            return Err(ExportError::InvalidData("Mesh doesn't have colors but export_colors is true".into()));
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
            let color = vertex.color.unwrap_or(Vec3::ONE);
            bin_buffer.write_f32::<LittleEndian>(color.x)?;
            bin_buffer.write_f32::<LittleEndian>(color.y)?;
            bin_buffer.write_f32::<LittleEndian>(color.z)?;
            bin_buffer.write_f32::<LittleEndian>(1.0)?; // Alpha = 1.0
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
    let mut writer = BufWriter::new(file);
    
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
    json.push_str(&format!("{}", options.name));
    json.push_str(r#""
    }
  ],
  "scene": 0,
"#);
    
    // Materials
    json.push_str(r#"  "materials": [
    {
      "name": "Material",
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
        json.push_str(&format!(",\n            \"NORMAL\": {}", accessor_index));
        accessor_index += 1;
    }
    
    if options.export_uvs {
        json.push_str(&format!(",\n            \"TEXCOORD_0\": {}", accessor_index));
        accessor_index += 1;
    }
    
    if options.export_tangents {
        json.push_str(&format!(",\n            \"TANGENT\": {}", accessor_index));
        accessor_index += 1;
    }
    
    if options.export_colors {
        json.push_str(&format!(",\n            \"COLOR_0\": {}", accessor_index));
        accessor_index += 1;
    }
    
    json.push_str(&format!("\n          }},\n          \"indices\": {},\n          \"material\": 0\n        }}\n      ]}}
  ]],\n", accessor_index));
    
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
        json.push_str(&format!("{}", options.export_normals ? 2 : 1));
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
      "type": "VEC4"
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
        .map_err(|_| ExportError::InvalidData("Invalid UTF-8 in glTF JSON".into()))?;
    
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
