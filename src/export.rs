
use crate::{Mesh, MeshError, Scene};
use std::fs::File;
use std::io::{Write, Seek, SeekFrom};
use std::path::Path;
use byteorder::{LittleEndian, WriteBytesExt};
use glam::{Vec2, Vec3, Vec4};
use std::fmt::Write as FmtWrite;
use serde_json;


const GLB_MAGIC: u32 = 0x46546C67; 
const GLB_VERSION: u32 = 2;
const GLB_CHUNK_TYPE_JSON: u32 = 0x4E4F534A; 
const GLB_CHUNK_TYPE_BIN: u32 = 0x004E4942; 

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

pub type ExportResult<T> = Result<T, ExportError>;

#[derive(Debug, Clone, PartialEq)]
pub struct Material {
    pub name: String,
    pub base_color: [f32; 3],
    pub metallic: f32,
    pub roughness: f32,
    pub emissive: [f32; 3],
}

impl Default for Material {
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

#[derive(Debug, Clone)]
pub struct GlbExportOptions {
    pub material: Material,
    pub export_normals: bool,
    pub export_uvs: bool,
    pub export_tangents: bool,
    pub export_colors: bool,
    pub name: String,
}

impl Default for GlbExportOptions {
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

pub fn export_to_glb(mesh: &Mesh, path: impl AsRef<Path>, options: GlbExportOptions) -> ExportResult<()> {
    
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

    
    let mut bin_buffer = Vec::new();
    
    
    let positions_byte_offset = 0;
    let positions_byte_length = mesh.vertices.len() * 3 * std::mem::size_of::<f32>();
    for vertex in &mesh.vertices {
        bin_buffer.write_f32::<LittleEndian>(vertex.position.x)?;
        bin_buffer.write_f32::<LittleEndian>(vertex.position.y)?;
        bin_buffer.write_f32::<LittleEndian>(vertex.position.z)?;
    }
    
    
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
    
    
    let colors_byte_offset = if options.export_colors {
        let offset = bin_buffer.len();
        for vertex in &mesh.vertices {
            if let Some(color) = vertex.color {
                bin_buffer.write_f32::<LittleEndian>(color.x)?;
                bin_buffer.write_f32::<LittleEndian>(color.y)?;
                bin_buffer.write_f32::<LittleEndian>(color.z)?;
                
                bin_buffer.write_f32::<LittleEndian>(1.0)?;
            } else {
                
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
        mesh.vertices.len() * 4 * std::mem::size_of::<f32>() 
    } else {
        0
    };
    
    
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
        5123 
    } else {
        indices_byte_length = mesh.triangles.len() * 3 * std::mem::size_of::<u32>();
        for triangle in &mesh.triangles {
            for &idx in &triangle.indices {
                bin_buffer.write_u32::<LittleEndian>(idx as u32)?;
            }
        }
        5125 
    };

    
    while bin_buffer.len() % 4 != 0 {
        bin_buffer.push(0);
    }
    
    
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
    
    
    let mut padded_json = json.into_bytes();
    while padded_json.len() % 4 != 0 {
        padded_json.push(b' ');
    }
    
    
    let file = File::create(path)?;
    let mut writer = std::io::BufWriter::new(file);
    
    
    writer.write_u32::<LittleEndian>(GLB_MAGIC)?;
    writer.write_u32::<LittleEndian>(GLB_VERSION)?;
    
    
    let total_length = 12 + 8 + padded_json.len() + 8 + bin_buffer.len();
    writer.write_u32::<LittleEndian>(total_length as u32)?;
    
    
    writer.write_u32::<LittleEndian>(padded_json.len() as u32)?;
    writer.write_u32::<LittleEndian>(GLB_CHUNK_TYPE_JSON)?;
    writer.write_all(&padded_json)?;
    
    
    writer.write_u32::<LittleEndian>(bin_buffer.len() as u32)?;
    writer.write_u32::<LittleEndian>(GLB_CHUNK_TYPE_BIN)?;
    writer.write_all(&bin_buffer)?;
    
    
    
    Ok(())
}

pub trait ExportMesh {
    fn export_glb(&self, path: impl AsRef<Path>) -> ExportResult<()>;
    
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

pub trait ExportScene {
    fn export_scene_glb(&self, path: impl AsRef<Path>) -> ExportResult<()>;
    
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

pub fn export_scene_to_glb(scene: &Scene, path: impl AsRef<Path>) -> ExportResult<()> {
    export_scene_to_glb_with_options(scene, path, GlbExportOptions::default())
}

pub fn export_scene_to_glb_with_options(
    scene: &Scene, 
    path: impl AsRef<Path>,
    default_options: GlbExportOptions
) -> ExportResult<()> {
    let file = File::create(path.as_ref())?;
    let mut writer = std::io::BufWriter::new(file);
    
    
    let mut bin_buffer = Vec::new();
    let mut mesh_export_info = Vec::new();
    let mut materials = Vec::new();
    
    
    for (i, mesh) in scene.meshes.iter().enumerate() {
        
        let material = if let Some(mesh_material) = mesh.get_material() {
            mesh_material.clone()
        } else {
            default_options.material.clone()
        };
        
        
        let positions_byte_offset = bin_buffer.len();
        let positions_byte_length = mesh.vertices.len() * 3 * 4; 
        
        
        let mut min_pos = Vec3::new(f32::MAX, f32::MAX, f32::MAX);
        let mut max_pos = Vec3::new(f32::MIN, f32::MIN, f32::MIN);
        
        for vertex in &mesh.vertices {
            min_pos = min_pos.min(vertex.position);
            max_pos = max_pos.max(vertex.position);
            
            
            bin_buffer.write_f32::<LittleEndian>(vertex.position.x)?;
            bin_buffer.write_f32::<LittleEndian>(vertex.position.y)?;
            bin_buffer.write_f32::<LittleEndian>(vertex.position.z)?;
        }
        
        
        let normals_byte_offset = bin_buffer.len();
        let normals_byte_length = if default_options.export_normals {
            let start_len = bin_buffer.len();
            for vertex in &mesh.vertices {
                let normal = vertex.normal.unwrap_or(Vec3::Y);
                bin_buffer.write_f32::<LittleEndian>(normal.x)?;
                bin_buffer.write_f32::<LittleEndian>(normal.y)?;
                bin_buffer.write_f32::<LittleEndian>(normal.z)?;
            }
            bin_buffer.len() - start_len
        } else {
            0
        };
        
        
        let uvs_byte_offset = bin_buffer.len();
        let uvs_byte_length = if default_options.export_uvs {
            let start_len = bin_buffer.len();
            for vertex in &mesh.vertices {
                let uv = vertex.uv.unwrap_or(Vec2::ZERO);
                bin_buffer.write_f32::<LittleEndian>(uv.x)?;
                bin_buffer.write_f32::<LittleEndian>(uv.y)?;
            }
            bin_buffer.len() - start_len
        } else {
            0
        };
        
        
        let tangents_byte_offset = bin_buffer.len();
        let tangents_byte_length = if default_options.export_tangents {
            let start_len = bin_buffer.len();
            for vertex in &mesh.vertices {
                let tangent = vertex.tangent.unwrap_or(Vec4::new(1.0, 0.0, 0.0, 1.0));
                bin_buffer.write_f32::<LittleEndian>(tangent.x)?;
                bin_buffer.write_f32::<LittleEndian>(tangent.y)?;
                bin_buffer.write_f32::<LittleEndian>(tangent.z)?;
                bin_buffer.write_f32::<LittleEndian>(tangent.w)?;
            }
            bin_buffer.len() - start_len
        } else {
            0
        };
        
        
        let colors_byte_offset = bin_buffer.len();
        let colors_byte_length = if default_options.export_colors {
            let start_len = bin_buffer.len();
            for vertex in &mesh.vertices {
                if let Some(color) = vertex.color {
                    bin_buffer.write_f32::<LittleEndian>(color.x)?;
                    bin_buffer.write_f32::<LittleEndian>(color.y)?;
                    bin_buffer.write_f32::<LittleEndian>(color.z)?;
                    
                    bin_buffer.write_f32::<LittleEndian>(1.0)?;
                } else {
                    
                    bin_buffer.write_f32::<LittleEndian>(1.0)?;
                    bin_buffer.write_f32::<LittleEndian>(1.0)?;
                    bin_buffer.write_f32::<LittleEndian>(1.0)?;
                    bin_buffer.write_f32::<LittleEndian>(1.0)?;
                }
            }
            bin_buffer.len() - start_len
        } else {
            0
        };
        
        
        let indices_byte_offset = bin_buffer.len();
        let indices_count = mesh.triangles.len() * 3;
        
        for triangle in &mesh.triangles {
            bin_buffer.write_u32::<LittleEndian>(triangle.indices[0] as u32)?;
            bin_buffer.write_u32::<LittleEndian>(triangle.indices[1] as u32)?;
            bin_buffer.write_u32::<LittleEndian>(triangle.indices[2] as u32)?;
        }
        
        let indices_byte_length = mesh.triangles.len() * 3 * 4; 
        
        
        mesh_export_info.push(MeshExportInfo {
            name: format!("Mesh_{}", i),
            vertex_count: mesh.vertices.len(),
            index_count: indices_count,
            material_index: i,
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
            min_pos,
            max_pos,
        });
        
        
        materials.push(material);
    }
    
    
    while bin_buffer.len() % 4 != 0 {
        bin_buffer.push(0);
    }
    
    
    let json = build_multi_mesh_gltf_json(&scene.name, &mesh_export_info, &materials, bin_buffer.len());
    
    
    let mut padded_json = json.into_bytes();
    while padded_json.len() % 4 != 0 {
        padded_json.push(b' ');
    }
    
    
    writer.write_u32::<LittleEndian>(GLB_MAGIC)?;
    writer.write_u32::<LittleEndian>(GLB_VERSION)?;
    
    
    
    let total_length = 12 + 8 + padded_json.len() + 8 + bin_buffer.len();
    writer.write_u32::<LittleEndian>(total_length as u32)?;
    
    
    writer.write_u32::<LittleEndian>(padded_json.len() as u32)?;
    writer.write_u32::<LittleEndian>(GLB_CHUNK_TYPE_JSON)?;
    writer.write_all(&padded_json)?;
    
    
    writer.write_u32::<LittleEndian>(bin_buffer.len() as u32)?;
    writer.write_u32::<LittleEndian>(GLB_CHUNK_TYPE_BIN)?;
    writer.write_all(&bin_buffer)?;
    
    
    writer.flush()?;
    Ok(())
}

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
    let mut json_obj = serde_json::Map::new();
    
    
    let mut asset = serde_json::Map::new();
    asset.insert("version".to_string(), serde_json::Value::String("2.0".to_string()));
    asset.insert("generator".to_string(), serde_json::Value::String("mesh-tools GLB exporter".to_string()));
    json_obj.insert("asset".to_string(), serde_json::Value::Object(asset));
    
    
    json_obj.insert("scene".to_string(), serde_json::Value::Number(serde_json::Number::from(0)));
    
    
    let mut scene_obj = serde_json::Map::new();
    scene_obj.insert("nodes".to_string(), serde_json::Value::Array(vec![serde_json::Value::Number(serde_json::Number::from(0))]));
    json_obj.insert("scenes".to_string(), serde_json::Value::Array(vec![serde_json::Value::Object(scene_obj)]));
    
    
    let mut node = serde_json::Map::new();
    node.insert("mesh".to_string(), serde_json::Value::Number(serde_json::Number::from(0)));
    node.insert("name".to_string(), serde_json::Value::String(options.name.clone()));
    json_obj.insert("nodes".to_string(), serde_json::Value::Array(vec![serde_json::Value::Object(node)]));
    
    
    let mut mesh_obj = serde_json::Map::new();
    mesh_obj.insert("name".to_string(), serde_json::Value::String(options.name.clone()));
    
    
    let mut primitive = serde_json::Map::new();
    
    
    let mut attributes = serde_json::Map::new();
    
    
    attributes.insert("POSITION".to_string(), serde_json::Value::Number(serde_json::Number::from(0)));
    
    
    let mut next_accessor_index = 1;
    
    if normals_byte_length > 0 {
        attributes.insert("NORMAL".to_string(), serde_json::Value::Number(serde_json::Number::from(next_accessor_index)));
        next_accessor_index += 1;
    }
    
    if uvs_byte_length > 0 {
        attributes.insert("TEXCOORD_0".to_string(), serde_json::Value::Number(serde_json::Number::from(next_accessor_index)));
        next_accessor_index += 1;
    }
    
    if tangents_byte_length > 0 {
        attributes.insert("TANGENT".to_string(), serde_json::Value::Number(serde_json::Number::from(next_accessor_index)));
        next_accessor_index += 1;
    }
    
    if colors_byte_length > 0 {
        attributes.insert("COLOR_0".to_string(), serde_json::Value::Number(serde_json::Number::from(next_accessor_index)));
        next_accessor_index += 1;
    }
    
    primitive.insert("attributes".to_string(), serde_json::Value::Object(attributes));
    
    
    primitive.insert("indices".to_string(), serde_json::Value::Number(serde_json::Number::from(next_accessor_index)));
    
    
    primitive.insert("material".to_string(), serde_json::Value::Number(serde_json::Number::from(0)));
    
    
    primitive.insert("mode".to_string(), serde_json::Value::Number(serde_json::Number::from(4)));
    
    mesh_obj.insert("primitives".to_string(), serde_json::Value::Array(vec![serde_json::Value::Object(primitive)]));
    json_obj.insert("meshes".to_string(), serde_json::Value::Array(vec![serde_json::Value::Object(mesh_obj)]));
    
    
    let mut material = serde_json::Map::new();
    material.insert("name".to_string(), serde_json::Value::String(options.material.name.clone()));
    
    
    let mut pbr = serde_json::Map::new();
    
    
    let base_color_factor = vec![
        serde_json::Value::Number(serde_json::Number::from_f64(options.material.base_color[0] as f64).unwrap()),
        serde_json::Value::Number(serde_json::Number::from_f64(options.material.base_color[1] as f64).unwrap()),
        serde_json::Value::Number(serde_json::Number::from_f64(options.material.base_color[2] as f64).unwrap()),
        serde_json::Value::Number(serde_json::Number::from_f64(1.0).unwrap()), 
    ];
    pbr.insert("baseColorFactor".to_string(), serde_json::Value::Array(base_color_factor));
    
    
    pbr.insert("metallicFactor".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(options.material.metallic as f64).unwrap()));
    
    
    pbr.insert("roughnessFactor".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(options.material.roughness as f64).unwrap()));
    
    material.insert("pbrMetallicRoughness".to_string(), serde_json::Value::Object(pbr));
    
    
    let emissive_factor = vec![
        serde_json::Value::Number(serde_json::Number::from_f64(options.material.emissive[0] as f64).unwrap()),
        serde_json::Value::Number(serde_json::Number::from_f64(options.material.emissive[1] as f64).unwrap()),
        serde_json::Value::Number(serde_json::Number::from_f64(options.material.emissive[2] as f64).unwrap()),
    ];
    material.insert("emissiveFactor".to_string(), serde_json::Value::Array(emissive_factor));
    
    
    material.insert("doubleSided".to_string(), serde_json::Value::Bool(true));
    
    json_obj.insert("materials".to_string(), serde_json::Value::Array(vec![serde_json::Value::Object(material)]));
    
    
    let mut accessors = Vec::new();
    
    
    let mut position_accessor = serde_json::Map::new();
    position_accessor.insert("bufferView".to_string(), serde_json::Value::Number(serde_json::Number::from(0)));
    position_accessor.insert("componentType".to_string(), serde_json::Value::Number(serde_json::Number::from(5126))); 
    position_accessor.insert("count".to_string(), serde_json::Value::Number(serde_json::Number::from(mesh.vertices.len())));
    position_accessor.insert("type".to_string(), serde_json::Value::String("VEC3".to_string()));
    
    
    let mut min_pos = [f32::MAX, f32::MAX, f32::MAX];
    let mut max_pos = [f32::MIN, f32::MIN, f32::MIN];
    
    for vertex in &mesh.vertices {
        min_pos[0] = min_pos[0].min(vertex.position.x);
        min_pos[1] = min_pos[1].min(vertex.position.y);
        min_pos[2] = min_pos[2].min(vertex.position.z);
        
        max_pos[0] = max_pos[0].max(vertex.position.x);
        max_pos[1] = max_pos[1].max(vertex.position.y);
        max_pos[2] = max_pos[2].max(vertex.position.z);
    }
    
    let min = vec![
        serde_json::Value::Number(serde_json::Number::from_f64(min_pos[0] as f64).unwrap()),
        serde_json::Value::Number(serde_json::Number::from_f64(min_pos[1] as f64).unwrap()),
        serde_json::Value::Number(serde_json::Number::from_f64(min_pos[2] as f64).unwrap()),
    ];
    
    let max = vec![
        serde_json::Value::Number(serde_json::Number::from_f64(max_pos[0] as f64).unwrap()),
        serde_json::Value::Number(serde_json::Number::from_f64(max_pos[1] as f64).unwrap()),
        serde_json::Value::Number(serde_json::Number::from_f64(max_pos[2] as f64).unwrap()),
    ];
    
    position_accessor.insert("min".to_string(), serde_json::Value::Array(min));
    position_accessor.insert("max".to_string(), serde_json::Value::Array(max));
    
    accessors.push(serde_json::Value::Object(position_accessor));
    
    
    if normals_byte_length > 0 {
        let mut normal_accessor = serde_json::Map::new();
        normal_accessor.insert("bufferView".to_string(), serde_json::Value::Number(serde_json::Number::from(1)));
        normal_accessor.insert("componentType".to_string(), serde_json::Value::Number(serde_json::Number::from(5126))); 
        normal_accessor.insert("count".to_string(), serde_json::Value::Number(serde_json::Number::from(mesh.vertices.len())));
        normal_accessor.insert("type".to_string(), serde_json::Value::String("VEC3".to_string()));
        accessors.push(serde_json::Value::Object(normal_accessor));
    }
    
    
    if uvs_byte_length > 0 {
        let mut uv_accessor = serde_json::Map::new();
        uv_accessor.insert("bufferView".to_string(), serde_json::Value::Number(serde_json::Number::from(if normals_byte_length > 0 { 2 } else { 1 })));
        uv_accessor.insert("componentType".to_string(), serde_json::Value::Number(serde_json::Number::from(5126))); 
        uv_accessor.insert("count".to_string(), serde_json::Value::Number(serde_json::Number::from(mesh.vertices.len())));
        uv_accessor.insert("type".to_string(), serde_json::Value::String("VEC2".to_string()));
        accessors.push(serde_json::Value::Object(uv_accessor));
    }
    
    
    if tangents_byte_length > 0 {
        let mut tangent_accessor = serde_json::Map::new();
        
        
        let buffer_view_index = 1 + 
            (if normals_byte_length > 0 { 1 } else { 0 }) + 
            (if uvs_byte_length > 0 { 1 } else { 0 });
        
        tangent_accessor.insert("bufferView".to_string(), serde_json::Value::Number(serde_json::Number::from(buffer_view_index)));
        tangent_accessor.insert("componentType".to_string(), serde_json::Value::Number(serde_json::Number::from(5126))); 
        tangent_accessor.insert("count".to_string(), serde_json::Value::Number(serde_json::Number::from(mesh.vertices.len())));
        tangent_accessor.insert("type".to_string(), serde_json::Value::String("VEC4".to_string()));
        accessors.push(serde_json::Value::Object(tangent_accessor));
    }
    
    
    if colors_byte_length > 0 {
        let mut color_accessor = serde_json::Map::new();
        
        
        let buffer_view_index = 1 + 
            (if normals_byte_length > 0 { 1 } else { 0 }) + 
            (if uvs_byte_length > 0 { 1 } else { 0 }) + 
            (if tangents_byte_length > 0 { 1 } else { 0 });
        
        color_accessor.insert("bufferView".to_string(), serde_json::Value::Number(serde_json::Number::from(buffer_view_index)));
        color_accessor.insert("componentType".to_string(), serde_json::Value::Number(serde_json::Number::from(5126))); 
        color_accessor.insert("count".to_string(), serde_json::Value::Number(serde_json::Number::from(mesh.vertices.len())));
        color_accessor.insert("type".to_string(), serde_json::Value::String("VEC4".to_string()));
        color_accessor.insert("normalized".to_string(), serde_json::Value::Bool(true));
        accessors.push(serde_json::Value::Object(color_accessor));
    }
    
    
    let mut indices_accessor = serde_json::Map::new();
    
    
    let buffer_view_index = 1 + 
        (if normals_byte_length > 0 { 1 } else { 0 }) + 
        (if uvs_byte_length > 0 { 1 } else { 0 }) + 
        (if tangents_byte_length > 0 { 1 } else { 0 }) + 
        (if colors_byte_length > 0 { 1 } else { 0 });
    
    indices_accessor.insert("bufferView".to_string(), serde_json::Value::Number(serde_json::Number::from(buffer_view_index)));
    indices_accessor.insert("componentType".to_string(), serde_json::Value::Number(serde_json::Number::from(indices_component_type)));
    indices_accessor.insert("count".to_string(), serde_json::Value::Number(serde_json::Number::from(mesh.triangles.len() * 3)));
    indices_accessor.insert("type".to_string(), serde_json::Value::String("SCALAR".to_string()));
    
    accessors.push(serde_json::Value::Object(indices_accessor));
    
    json_obj.insert("accessors".to_string(), serde_json::Value::Array(accessors));
    
    
    let mut buffer_views = Vec::new();
    
    
    let mut position_view = serde_json::Map::new();
    position_view.insert("buffer".to_string(), serde_json::Value::Number(serde_json::Number::from(0)));
    position_view.insert("byteOffset".to_string(), serde_json::Value::Number(serde_json::Number::from(positions_byte_offset)));
    position_view.insert("byteLength".to_string(), serde_json::Value::Number(serde_json::Number::from(positions_byte_length)));
    position_view.insert("target".to_string(), serde_json::Value::Number(serde_json::Number::from(34962))); 
    buffer_views.push(serde_json::Value::Object(position_view));
    
    
    if normals_byte_length > 0 {
        let mut normal_view = serde_json::Map::new();
        normal_view.insert("buffer".to_string(), serde_json::Value::Number(serde_json::Number::from(0)));
        normal_view.insert("byteOffset".to_string(), serde_json::Value::Number(serde_json::Number::from(normals_byte_offset)));
        normal_view.insert("byteLength".to_string(), serde_json::Value::Number(serde_json::Number::from(normals_byte_length)));
        normal_view.insert("target".to_string(), serde_json::Value::Number(serde_json::Number::from(34962))); 
        buffer_views.push(serde_json::Value::Object(normal_view));
    }
    
    
    if uvs_byte_length > 0 {
        let mut uv_view = serde_json::Map::new();
        uv_view.insert("buffer".to_string(), serde_json::Value::Number(serde_json::Number::from(0)));
        uv_view.insert("byteOffset".to_string(), serde_json::Value::Number(serde_json::Number::from(uvs_byte_offset)));
        uv_view.insert("byteLength".to_string(), serde_json::Value::Number(serde_json::Number::from(uvs_byte_length)));
        uv_view.insert("target".to_string(), serde_json::Value::Number(serde_json::Number::from(34962))); 
        buffer_views.push(serde_json::Value::Object(uv_view));
    }
    
    
    if tangents_byte_length > 0 {
        let mut tangent_view = serde_json::Map::new();
        tangent_view.insert("buffer".to_string(), serde_json::Value::Number(serde_json::Number::from(0)));
        tangent_view.insert("byteOffset".to_string(), serde_json::Value::Number(serde_json::Number::from(tangents_byte_offset)));
        tangent_view.insert("byteLength".to_string(), serde_json::Value::Number(serde_json::Number::from(tangents_byte_length)));
        tangent_view.insert("target".to_string(), serde_json::Value::Number(serde_json::Number::from(34962))); 
        buffer_views.push(serde_json::Value::Object(tangent_view));
    }
    
    
    if colors_byte_length > 0 {
        let mut color_view = serde_json::Map::new();
        color_view.insert("buffer".to_string(), serde_json::Value::Number(serde_json::Number::from(0)));
        color_view.insert("byteOffset".to_string(), serde_json::Value::Number(serde_json::Number::from(colors_byte_offset)));
        color_view.insert("byteLength".to_string(), serde_json::Value::Number(serde_json::Number::from(colors_byte_length)));
        color_view.insert("target".to_string(), serde_json::Value::Number(serde_json::Number::from(34962))); 
        buffer_views.push(serde_json::Value::Object(color_view));
    }
    
    
    let mut indices_view = serde_json::Map::new();
    indices_view.insert("buffer".to_string(), serde_json::Value::Number(serde_json::Number::from(0)));
    indices_view.insert("byteOffset".to_string(), serde_json::Value::Number(serde_json::Number::from(indices_byte_offset)));
    indices_view.insert("byteLength".to_string(), serde_json::Value::Number(serde_json::Number::from(indices_byte_length)));
    indices_view.insert("target".to_string(), serde_json::Value::Number(serde_json::Number::from(34963))); 
    buffer_views.push(serde_json::Value::Object(indices_view));
    
    json_obj.insert("bufferViews".to_string(), serde_json::Value::Array(buffer_views));
    
    
    let mut buffer = serde_json::Map::new();
    buffer.insert("byteLength".to_string(), serde_json::Value::Number(serde_json::Number::from(buffer_length)));
    json_obj.insert("buffers".to_string(), serde_json::Value::Array(vec![serde_json::Value::Object(buffer)]));
    
    
    serde_json::to_string(&serde_json::Value::Object(json_obj)).unwrap_or_else(|_| "{}".to_string())
}

fn build_multi_mesh_gltf_json(
    scene_name: &str,
    mesh_export_info: &[MeshExportInfo],
    materials: &[Material],
    buffer_length: usize,
) -> String {
    let mut json_obj = serde_json::Map::new();
    
    
    let mut asset = serde_json::Map::new();
    asset.insert("version".to_string(), serde_json::Value::String("2.0".to_string()));
    asset.insert("generator".to_string(), serde_json::Value::String("mesh-tools GLB exporter".to_string()));
    json_obj.insert("asset".to_string(), serde_json::Value::Object(asset));
    
    
    json_obj.insert("scene".to_string(), serde_json::Value::Number(serde_json::Number::from(0)));
    
    
    let mut scene_obj = serde_json::Map::new();
    scene_obj.insert("name".to_string(), serde_json::Value::String(scene_name.to_string()));
    
    
    let node_indices: Vec<serde_json::Value> = (0..mesh_export_info.len())
        .map(|i| serde_json::Value::Number(serde_json::Number::from(i)))
        .collect();
    
    scene_obj.insert("nodes".to_string(), serde_json::Value::Array(node_indices));
    json_obj.insert("scenes".to_string(), serde_json::Value::Array(vec![serde_json::Value::Object(scene_obj)]));
    
    
    let mut nodes = Vec::new();
    for (i, info) in mesh_export_info.iter().enumerate() {
        let mut node = serde_json::Map::new();
        node.insert("mesh".to_string(), serde_json::Value::Number(serde_json::Number::from(i)));
        node.insert("name".to_string(), serde_json::Value::String(info.name.clone()));
        nodes.push(serde_json::Value::Object(node));
    }
    json_obj.insert("nodes".to_string(), serde_json::Value::Array(nodes));
    
    
    let mut meshes = Vec::new();
    for (i, info) in mesh_export_info.iter().enumerate() {
        let mut mesh = serde_json::Map::new();
        mesh.insert("name".to_string(), serde_json::Value::String(info.name.clone()));
        
        
        let mut primitive = serde_json::Map::new();
        
        
        let mut attributes = serde_json::Map::new();
        
        
        attributes.insert("POSITION".to_string(), serde_json::Value::Number(serde_json::Number::from(0)));
        
        
        let mut next_accessor_index = 1;
        
        if info.normals_byte_length > 0 {
            attributes.insert("NORMAL".to_string(), serde_json::Value::Number(serde_json::Number::from(next_accessor_index)));
            next_accessor_index += 1;
        }
        
        if info.uvs_byte_length > 0 {
            attributes.insert("TEXCOORD_0".to_string(), serde_json::Value::Number(serde_json::Number::from(next_accessor_index)));
            next_accessor_index += 1;
        }
        
        if info.tangents_byte_length > 0 {
            attributes.insert("TANGENT".to_string(), serde_json::Value::Number(serde_json::Number::from(next_accessor_index)));
            next_accessor_index += 1;
        }
        
        if info.colors_byte_length > 0 {
            attributes.insert("COLOR_0".to_string(), serde_json::Value::Number(serde_json::Number::from(next_accessor_index)));
            next_accessor_index += 1;
        }
        
        primitive.insert("attributes".to_string(), serde_json::Value::Object(attributes));
        
        
        primitive.insert("indices".to_string(), serde_json::Value::Number(serde_json::Number::from(next_accessor_index)));
        
        
        primitive.insert("material".to_string(), serde_json::Value::Number(serde_json::Number::from(info.material_index)));
        
        
        primitive.insert("mode".to_string(), serde_json::Value::Number(serde_json::Number::from(4)));
        
        mesh.insert("primitives".to_string(), serde_json::Value::Array(vec![serde_json::Value::Object(primitive)]));
        meshes.push(serde_json::Value::Object(mesh));
    }
    json_obj.insert("meshes".to_string(), serde_json::Value::Array(meshes));
    
    
    let mut materials_json = Vec::new();
    for material in materials {
        let mut material_json = serde_json::Map::new();
        material_json.insert("name".to_string(), serde_json::Value::String(material.name.clone()));
        
        
        let mut pbr = serde_json::Map::new();
        
        
        let base_color_factor = vec![
            serde_json::Value::Number(serde_json::Number::from_f64(material.base_color[0] as f64).unwrap()),
            serde_json::Value::Number(serde_json::Number::from_f64(material.base_color[1] as f64).unwrap()),
            serde_json::Value::Number(serde_json::Number::from_f64(material.base_color[2] as f64).unwrap()),
            serde_json::Value::Number(serde_json::Number::from_f64(1.0).unwrap()), 
        ];
        pbr.insert("baseColorFactor".to_string(), serde_json::Value::Array(base_color_factor));
        
        
        pbr.insert("metallicFactor".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(material.metallic as f64).unwrap()));
        
        
        pbr.insert("roughnessFactor".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(material.roughness as f64).unwrap()));
        
        material_json.insert("pbrMetallicRoughness".to_string(), serde_json::Value::Object(pbr));
        
        
        let emissive_factor = vec![
            serde_json::Value::Number(serde_json::Number::from_f64(material.emissive[0] as f64).unwrap()),
            serde_json::Value::Number(serde_json::Number::from_f64(material.emissive[1] as f64).unwrap()),
            serde_json::Value::Number(serde_json::Number::from_f64(material.emissive[2] as f64).unwrap()),
        ];
        material_json.insert("emissiveFactor".to_string(), serde_json::Value::Array(emissive_factor));
        
        
        material_json.insert("doubleSided".to_string(), serde_json::Value::Bool(true));
        
        materials_json.push(serde_json::Value::Object(material_json));
    }
    json_obj.insert("materials".to_string(), serde_json::Value::Array(materials_json));
    
    
    let mut accessors = Vec::new();
    let mut next_accessor_index = 0;
    
    for info in mesh_export_info {
        
        let mut position_accessor = serde_json::Map::new();
        position_accessor.insert("bufferView".to_string(), serde_json::Value::Number(serde_json::Number::from(0)));
        position_accessor.insert("componentType".to_string(), serde_json::Value::Number(serde_json::Number::from(5126))); 
        position_accessor.insert("count".to_string(), serde_json::Value::Number(serde_json::Number::from(info.vertex_count)));
        position_accessor.insert("type".to_string(), serde_json::Value::String("VEC3".to_string()));
        
        
        let min = vec![
            serde_json::Value::Number(serde_json::Number::from_f64(info.min_pos.x as f64).unwrap()),
            serde_json::Value::Number(serde_json::Number::from_f64(info.min_pos.y as f64).unwrap()),
            serde_json::Value::Number(serde_json::Number::from_f64(info.min_pos.z as f64).unwrap()),
        ];
        
        let max = vec![
            serde_json::Value::Number(serde_json::Number::from_f64(info.max_pos.x as f64).unwrap()),
            serde_json::Value::Number(serde_json::Number::from_f64(info.max_pos.y as f64).unwrap()),
            serde_json::Value::Number(serde_json::Number::from_f64(info.max_pos.z as f64).unwrap()),
        ];
        
        position_accessor.insert("min".to_string(), serde_json::Value::Array(min));
        position_accessor.insert("max".to_string(), serde_json::Value::Array(max));
        
        accessors.push(serde_json::Value::Object(position_accessor));
        
        
        if info.normals_byte_length > 0 {
            let mut normal_accessor = serde_json::Map::new();
            normal_accessor.insert("bufferView".to_string(), serde_json::Value::Number(serde_json::Number::from(1)));
            normal_accessor.insert("componentType".to_string(), serde_json::Value::Number(serde_json::Number::from(5126))); 
            normal_accessor.insert("count".to_string(), serde_json::Value::Number(serde_json::Number::from(info.vertex_count)));
            normal_accessor.insert("type".to_string(), serde_json::Value::String("VEC3".to_string()));
            accessors.push(serde_json::Value::Object(normal_accessor));
        }
        
        
        if info.uvs_byte_length > 0 {
            let mut uv_accessor = serde_json::Map::new();
            uv_accessor.insert("bufferView".to_string(), serde_json::Value::Number(serde_json::Number::from(if info.normals_byte_length > 0 { 2 } else { 1 })));
            uv_accessor.insert("componentType".to_string(), serde_json::Value::Number(serde_json::Number::from(5126))); 
            uv_accessor.insert("count".to_string(), serde_json::Value::Number(serde_json::Number::from(info.vertex_count)));
            uv_accessor.insert("type".to_string(), serde_json::Value::String("VEC2".to_string()));
            accessors.push(serde_json::Value::Object(uv_accessor));
        }
        
        
        if info.tangents_byte_length > 0 {
            let mut tangent_accessor = serde_json::Map::new();
            
            
            let buffer_view_index = 1 + 
                (if info.normals_byte_length > 0 { 1 } else { 0 }) + 
                (if info.uvs_byte_length > 0 { 1 } else { 0 });
            
            tangent_accessor.insert("bufferView".to_string(), serde_json::Value::Number(serde_json::Number::from(buffer_view_index)));
            tangent_accessor.insert("componentType".to_string(), serde_json::Value::Number(serde_json::Number::from(5126))); 
            tangent_accessor.insert("count".to_string(), serde_json::Value::Number(serde_json::Number::from(info.vertex_count)));
            tangent_accessor.insert("type".to_string(), serde_json::Value::String("VEC4".to_string()));
            accessors.push(serde_json::Value::Object(tangent_accessor));
        }
        
        
        if info.colors_byte_length > 0 {
            let mut color_accessor = serde_json::Map::new();
            
            
            let buffer_view_index = 1 + 
                (if info.normals_byte_length > 0 { 1 } else { 0 }) + 
                (if info.uvs_byte_length > 0 { 1 } else { 0 }) + 
                (if info.tangents_byte_length > 0 { 1 } else { 0 });
            
            color_accessor.insert("bufferView".to_string(), serde_json::Value::Number(serde_json::Number::from(buffer_view_index)));
            color_accessor.insert("componentType".to_string(), serde_json::Value::Number(serde_json::Number::from(5126))); 
            color_accessor.insert("count".to_string(), serde_json::Value::Number(serde_json::Number::from(info.vertex_count)));
            color_accessor.insert("type".to_string(), serde_json::Value::String("VEC4".to_string()));
            color_accessor.insert("normalized".to_string(), serde_json::Value::Bool(true));
            accessors.push(serde_json::Value::Object(color_accessor));
        }
        
        
        let mut indices_accessor = serde_json::Map::new();
        
        
        let buffer_view_index = 1 + 
            (if info.normals_byte_length > 0 { 1 } else { 0 }) + 
            (if info.uvs_byte_length > 0 { 1 } else { 0 }) + 
            (if info.tangents_byte_length > 0 { 1 } else { 0 }) + 
            (if info.colors_byte_length > 0 { 1 } else { 0 });
        
        indices_accessor.insert("bufferView".to_string(), serde_json::Value::Number(serde_json::Number::from(buffer_view_index)));
        indices_accessor.insert("componentType".to_string(), serde_json::Value::Number(serde_json::Number::from(5125))); 
        indices_accessor.insert("count".to_string(), serde_json::Value::Number(serde_json::Number::from(info.index_count)));
        indices_accessor.insert("type".to_string(), serde_json::Value::String("SCALAR".to_string()));
        
        accessors.push(serde_json::Value::Object(indices_accessor));
        
        next_accessor_index += 1 + 
            (if info.normals_byte_length > 0 { 1 } else { 0 }) + 
            (if info.uvs_byte_length > 0 { 1 } else { 0 }) + 
            (if info.tangents_byte_length > 0 { 1 } else { 0 }) + 
            (if info.colors_byte_length > 0 { 1 } else { 0 }) + 1;
    }
    json_obj.insert("accessors".to_string(), serde_json::Value::Array(accessors));
    
    
    let mut buffer_views = Vec::new();
    let mut next_buffer_view_index = 0;
    
    for info in mesh_export_info {
        
        let mut position_view = serde_json::Map::new();
        position_view.insert("buffer".to_string(), serde_json::Value::Number(serde_json::Number::from(0)));
        position_view.insert("byteOffset".to_string(), serde_json::Value::Number(serde_json::Number::from(info.positions_byte_offset)));
        position_view.insert("byteLength".to_string(), serde_json::Value::Number(serde_json::Number::from(info.positions_byte_length)));
        position_view.insert("target".to_string(), serde_json::Value::Number(serde_json::Number::from(34962))); 
        buffer_views.push(serde_json::Value::Object(position_view));
        
        
        if info.normals_byte_length > 0 {
            let mut normal_view = serde_json::Map::new();
            normal_view.insert("buffer".to_string(), serde_json::Value::Number(serde_json::Number::from(0)));
            normal_view.insert("byteOffset".to_string(), serde_json::Value::Number(serde_json::Number::from(info.normals_byte_offset)));
            normal_view.insert("byteLength".to_string(), serde_json::Value::Number(serde_json::Number::from(info.normals_byte_length)));
            normal_view.insert("target".to_string(), serde_json::Value::Number(serde_json::Number::from(34962))); 
            buffer_views.push(serde_json::Value::Object(normal_view));
        }
        
        
        if info.uvs_byte_length > 0 {
            let mut uv_view = serde_json::Map::new();
            uv_view.insert("buffer".to_string(), serde_json::Value::Number(serde_json::Number::from(0)));
            uv_view.insert("byteOffset".to_string(), serde_json::Value::Number(serde_json::Number::from(info.uvs_byte_offset)));
            uv_view.insert("byteLength".to_string(), serde_json::Value::Number(serde_json::Number::from(info.uvs_byte_length)));
            uv_view.insert("target".to_string(), serde_json::Value::Number(serde_json::Number::from(34962))); 
            buffer_views.push(serde_json::Value::Object(uv_view));
        }
        
        
        if info.tangents_byte_length > 0 {
            let mut tangent_view = serde_json::Map::new();
            tangent_view.insert("buffer".to_string(), serde_json::Value::Number(serde_json::Number::from(0)));
            tangent_view.insert("byteOffset".to_string(), serde_json::Value::Number(serde_json::Number::from(info.tangents_byte_offset)));
            tangent_view.insert("byteLength".to_string(), serde_json::Value::Number(serde_json::Number::from(info.tangents_byte_length)));
            tangent_view.insert("target".to_string(), serde_json::Value::Number(serde_json::Number::from(34962))); 
            buffer_views.push(serde_json::Value::Object(tangent_view));
        }
        
        
        if info.colors_byte_length > 0 {
            let mut color_view = serde_json::Map::new();
            color_view.insert("buffer".to_string(), serde_json::Value::Number(serde_json::Number::from(0)));
            color_view.insert("byteOffset".to_string(), serde_json::Value::Number(serde_json::Number::from(info.colors_byte_offset)));
            color_view.insert("byteLength".to_string(), serde_json::Value::Number(serde_json::Number::from(info.colors_byte_length)));
            color_view.insert("target".to_string(), serde_json::Value::Number(serde_json::Number::from(34962))); 
            buffer_views.push(serde_json::Value::Object(color_view));
        }
        
        
        let mut indices_view = serde_json::Map::new();
        indices_view.insert("buffer".to_string(), serde_json::Value::Number(serde_json::Number::from(0)));
        indices_view.insert("byteOffset".to_string(), serde_json::Value::Number(serde_json::Number::from(info.indices_byte_offset)));
        indices_view.insert("byteLength".to_string(), serde_json::Value::Number(serde_json::Number::from(info.indices_byte_length)));
        indices_view.insert("target".to_string(), serde_json::Value::Number(serde_json::Number::from(34963))); 
        buffer_views.push(serde_json::Value::Object(indices_view));
        
        next_buffer_view_index += 1 + 
            (if info.normals_byte_length > 0 { 1 } else { 0 }) + 
            (if info.uvs_byte_length > 0 { 1 } else { 0 }) + 
            (if info.tangents_byte_length > 0 { 1 } else { 0 }) + 
            (if info.colors_byte_length > 0 { 1 } else { 0 }) + 1;
    }
    json_obj.insert("bufferViews".to_string(), serde_json::Value::Array(buffer_views));
    
    
    let mut buffer = serde_json::Map::new();
    buffer.insert("byteLength".to_string(), serde_json::Value::Number(serde_json::Number::from(buffer_length)));
    json_obj.insert("buffers".to_string(), serde_json::Value::Array(vec![serde_json::Value::Object(buffer)]));
    
    
    serde_json::to_string(&serde_json::Value::Object(json_obj)).unwrap_or_else(|_| "{}".to_string())
}

#[derive(Debug, Clone)]
struct MeshExportInfo {
    name: String,
    vertex_count: usize,
    index_count: usize,
    material_index: usize,
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
    min_pos: Vec3,
    max_pos: Vec3,
}
