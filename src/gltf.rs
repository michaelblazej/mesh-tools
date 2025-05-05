//! glTF 2.0 format implementation for mesh-tools
//! 
//! This module provides functionality to export meshes to the glTF 2.0 format,
//! both as JSON (.gltf) and binary (.glb) files.
//! 
//! References:
//! - glTF 2.0 Specification: https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html

use std::collections::HashMap;
use std::io::{self, Write};
use std::path::Path;
use std::fs::File;
use byteorder::{LittleEndian, WriteBytesExt};
use serde::{Serialize, Deserialize};
use glam::{Vec2, Vec3, Vec4};
use crate::mesh::Mesh;

// Magic header for GLB format
const GLB_MAGIC: u32 = 0x46546C67; // "glTF" in ASCII
const GLB_VERSION: u32 = 2;
const GLB_CHUNK_TYPE_JSON: u32 = 0x4E4F534A; // "JSON" in ASCII
const GLB_CHUNK_TYPE_BIN: u32 = 0x004E4942; // "BIN\0" in ASCII
const GLB_HEADER_LENGTH: usize = 12;
const GLB_CHUNK_HEADER_LENGTH: usize = 8;

// Component type constants 
pub mod component_type {
    /// Signed byte (8-bit)
    pub const BYTE: u32 = 5120;
    /// Unsigned byte (8-bit)
    pub const UNSIGNED_BYTE: u32 = 5121;
    /// Signed short (16-bit)
    pub const SHORT: u32 = 5122;
    /// Unsigned short (16-bit)
    pub const UNSIGNED_SHORT: u32 = 5123;
    /// Unsigned int (32-bit)
    pub const UNSIGNED_INT: u32 = 5125;
    /// Floating point (32-bit)
    pub const FLOAT: u32 = 5126;
}

// Buffer target constants
pub mod buffer_target {
    /// Array buffer
    pub const ARRAY_BUFFER: u32 = 34962;
    /// Element array buffer
    pub const ELEMENT_ARRAY_BUFFER: u32 = 34963;
}

// Primitive mode constants
pub mod primitive_mode {
    /// Points
    pub const POINTS: u32 = 0;
    /// Lines
    pub const LINES: u32 = 1;
    /// Line loop
    pub const LINE_LOOP: u32 = 2;
    /// Line strip
    pub const LINE_STRIP: u32 = 3;
    /// Triangles (default)
    pub const TRIANGLES: u32 = 4;
    /// Triangle strip
    pub const TRIANGLE_STRIP: u32 = 5;
    /// Triangle fan
    pub const TRIANGLE_FAN: u32 = 6;
}

// Alpha mode constants
pub mod alpha_mode {
    /// The alpha value is ignored and the rendered output is fully opaque
    pub const OPAQUE: &str = "OPAQUE";
    /// The rendered output is either fully opaque or fully transparent depending on the alpha value
    pub const MASK: &str = "MASK";
    /// The alpha value is used to composite the source and destination areas
    pub const BLEND: &str = "BLEND";
}

/// GLB export options
#[derive(Debug, Clone)]
pub struct GlbExportOptions {
    /// Whether to include normals in the export
    pub include_normals: bool,
    /// Whether to include texture coordinates in the export
    pub include_uvs: bool,
    /// Whether to include tangents in the export
    pub include_tangents: bool,
    /// Whether to include vertex colors in the export
    pub include_colors: bool,
    /// Optional material to apply to the mesh
    pub material: Option<Material>,
}

impl Default for GlbExportOptions {
    fn default() -> Self {
        Self {
            include_normals: true,
            include_uvs: true,
            include_tangents: false,
            include_colors: false,
            material: None,
        }
    }
}

/// Binary GLB file format utilities
struct GlbWriter {
    json_chunk: Vec<u8>,
    bin_chunk: Vec<u8>,
}

impl GlbWriter {
    /// Create a new GLB writer
    fn new() -> Self {
        Self {
            json_chunk: Vec::new(),
            bin_chunk: Vec::new(),
        }
    }

    /// Set the JSON chunk content
    fn set_json(&mut self, json: String) {
        self.json_chunk = json.into_bytes();
    }

    /// Add data to the binary buffer and return the byte offset
    fn add_binary_data(&mut self, data: &[u8]) -> usize {
        let offset = self.bin_chunk.len();
        self.bin_chunk.extend_from_slice(data);
        offset
    }

    /// Write the GLB file to the given writer
    fn write<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        // Calculate padded lengths
        let json_padded_length = Self::padded_length(&self.json_chunk);
        let bin_padded_length = Self::padded_length(&self.bin_chunk);
        
        // Calculate total length (header + chunk headers + padded chunks)
        let total_length = GLB_HEADER_LENGTH + 
                          2 * GLB_CHUNK_HEADER_LENGTH + 
                          json_padded_length + 
                          bin_padded_length;
        
        // Write GLB header
        writer.write_u32::<LittleEndian>(GLB_MAGIC)?;
        writer.write_u32::<LittleEndian>(GLB_VERSION)?;
        writer.write_u32::<LittleEndian>(total_length as u32)?;
        
        // Write JSON chunk header
        writer.write_u32::<LittleEndian>(json_padded_length as u32)?;
        writer.write_u32::<LittleEndian>(GLB_CHUNK_TYPE_JSON)?;
        
        // Write JSON chunk data
        writer.write_all(&self.json_chunk)?;
        
        // Pad JSON chunk to 4-byte boundary
        Self::write_padding(writer, &self.json_chunk)?;
        
        // Write BIN chunk header
        writer.write_u32::<LittleEndian>(bin_padded_length as u32)?;
        writer.write_u32::<LittleEndian>(GLB_CHUNK_TYPE_BIN)?;
        
        // Write BIN chunk data
        writer.write_all(&self.bin_chunk)?;
        
        // Pad BIN chunk to 4-byte boundary
        Self::write_padding(writer, &self.bin_chunk)?;
        
        Ok(())
    }

    /// Calculate the length of a buffer padded to a 4-byte boundary
    fn padded_length(buffer: &[u8]) -> usize {
        let remainder = buffer.len() % 4;
        if remainder == 0 {
            buffer.len()
        } else {
            buffer.len() + (4 - remainder)
        }
    }

    /// Write padding bytes to align to a 4-byte boundary
    fn write_padding<W: Write>(writer: &mut W, buffer: &[u8]) -> io::Result<()> {
        let remainder = buffer.len() % 4;
        if remainder != 0 {
            for _ in 0..(4 - remainder) {
                writer.write_u8(0)?;
            }
        }
        Ok(())
    }
}

// glTF 2.0 Core Data Structures
// These structs match the glTF 2.0 specification

/// The root glTF object
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct GltfRoot {
    /// Metadata about the glTF asset
    pub asset: Asset,
    /// An array of scenes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenes: Option<Vec<Scene>>,
    /// The index of the default scene
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene: Option<usize>,
    /// An array of nodes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<Node>>,
    /// An array of meshes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meshes: Option<Vec<GltfMesh>>,
    /// An array of materials
    #[serde(skip_serializing_if = "Option::is_none")]
    pub materials: Option<Vec<Material>>,
    /// An array of textures
    #[serde(skip_serializing_if = "Option::is_none")]
    pub textures: Option<Vec<Texture>>,
    /// An array of images
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<Image>>,
    /// An array of samplers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub samplers: Option<Vec<Sampler>>,
    /// An array of accessors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accessors: Option<Vec<Accessor>>,
    /// An array of buffer views
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bufferViews: Option<Vec<BufferView>>,
    /// An array of buffers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffers: Option<Vec<Buffer>>,
    /// glTF extensions used in this asset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensionsUsed: Option<Vec<String>>,
    /// glTF extensions required to properly load this asset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensionsRequired: Option<Vec<String>>,
    /// Application-specific data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,
}

/// Metadata about the glTF asset
#[derive(Serialize, Deserialize, Debug)]
pub struct Asset {
    /// A copyright message suitable for display to credit the content creator
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copyright: Option<String>,
    /// Tool that generated this glTF model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generator: Option<String>,
    /// The glTF version
    pub version: String,
    /// The minimum glTF version that this asset targets
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minVersion: Option<String>,
    /// Application-specific data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,
}

impl Default for Asset {
    fn default() -> Self {
        Self {
            copyright: None,
            generator: Some("mesh-tools".to_string()),
            version: "2.0".to_string(),
            minVersion: None,
            extras: None,
        }
    }
}

/// A scene in the glTF asset
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Scene {
    /// The indices of each root node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<usize>>,
    /// The user-defined name of this object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Application-specific data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,
}

/// A node in the node hierarchy
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Node {
    /// The index of the camera referenced by this node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub camera: Option<usize>,
    /// The indices of this node's children
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<usize>>,
    /// The index of the skin referenced by this node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skin: Option<usize>,
    /// A floating-point 4x4 transformation matrix stored in column-major order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matrix: Option<[f32; 16]>,
    /// The index of the mesh in this node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh: Option<usize>,
    /// The node's unit quaternion rotation in the order (x, y, z, w)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation: Option<[f32; 4]>,
    /// The node's non-uniform scale, given as the scaling factors along the x, y, and z axes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<[f32; 3]>,
    /// The node's translation along the x, y, and z axes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translation: Option<[f32; 3]>,
    /// The weights of the instantiated morph target
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weights: Option<Vec<f32>>,
    /// The user-defined name of this object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Application-specific data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,
}

/// A mesh primitive
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Primitive {
    /// A dictionary object, where each key corresponds to a mesh attribute semantic
    /// and each value is the index of the accessor containing attribute's data
    pub attributes: HashMap<String, usize>,
    /// The index of the accessor that contains the indices
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indices: Option<usize>,
    /// The index of the material to apply to this primitive when rendering
    #[serde(skip_serializing_if = "Option::is_none")]
    pub material: Option<usize>,
    /// The type of primitives to render
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<u32>,
    /// An array of morph targets
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<HashMap<String, usize>>>,
    /// Application-specific data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,
}

/// A set of primitives to be rendered
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct GltfMesh {
    /// An array of primitives, each defining geometry to be rendered
    pub primitives: Vec<Primitive>,
    /// Array of weights to be applied to the morph targets
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weights: Option<Vec<f32>>,
    /// The user-defined name of this object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Application-specific data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,
}

/// The material appearance of a primitive
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Material {
    /// The name of the material
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A set of parameter values that define the metallic-roughness material model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pbrMetallicRoughness: Option<PbrMetallicRoughness>,
    /// The normal map texture
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normalTexture: Option<NormalTextureInfo>,
    /// The occlusion texture
    #[serde(skip_serializing_if = "Option::is_none")]
    pub occlusionTexture: Option<OcclusionTextureInfo>,
    /// The emissive texture
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emissiveTexture: Option<TextureInfo>,
    /// The emissive color of the material
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emissiveFactor: Option<[f32; 3]>,
    /// The alpha rendering mode of the material
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alphaMode: Option<String>,
    /// The alpha cutoff value of the material
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alphaCutoff: Option<f32>,
    /// Specifies whether the material is double sided
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doubleSided: Option<bool>,
    /// Application-specific data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,
}

/// A set of parameter values that define the metallic-roughness material model
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PbrMetallicRoughness {
    /// The base color factor
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseColorFactor: Option<[f32; 4]>,
    /// The base color texture
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseColorTexture: Option<TextureInfo>,
    /// The metalness factor
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metallicFactor: Option<f32>,
    /// The roughness factor
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roughnessFactor: Option<f32>,
    /// The metallic-roughness texture
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metallicRoughnessTexture: Option<TextureInfo>,
    /// Application-specific data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,
}

/// Reference to a texture
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TextureInfo {
    /// The index of the texture
    pub index: usize,
    /// The set index of texture's TEXCOORD attribute used for texture coordinate mapping
    #[serde(skip_serializing_if = "Option::is_none")]
    pub texCoord: Option<usize>,
    /// Application-specific data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,
}

/// Normal texture information
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct NormalTextureInfo {
    /// The index of the texture
    pub index: usize,
    /// The set index of texture's TEXCOORD attribute used for texture coordinate mapping
    #[serde(skip_serializing_if = "Option::is_none")]
    pub texCoord: Option<usize>,
    /// The scalar multiplier applied to each normal vector of the normal texture
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<f32>,
    /// Application-specific data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,
}

/// Occlusion texture information
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct OcclusionTextureInfo {
    /// The index of the texture
    pub index: usize,
    /// The set index of texture's TEXCOORD attribute used for texture coordinate mapping
    #[serde(skip_serializing_if = "Option::is_none")]
    pub texCoord: Option<usize>,
    /// A scalar multiplier controlling the amount of occlusion applied
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength: Option<f32>,
    /// Application-specific data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,
}

/// A texture and its sampler
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Texture {
    /// The index of the sampler used by this texture
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampler: Option<usize>,
    /// The index of the image used by this texture
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<usize>,
    /// The user-defined name of this object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Application-specific data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,
}

/// Image data used to create a texture
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Image {
    /// The URI (or IRI) of the image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    /// The image's MIME type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mimeType: Option<String>,
    /// The index of the bufferView that contains the image. Use this instead of the image's uri property
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bufferView: Option<usize>,
    /// The user-defined name of this object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Application-specific data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,
}

/// Texture sampler properties for filtering and wrapping modes
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Sampler {
    /// Magnification filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub magFilter: Option<u32>,
    /// Minification filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minFilter: Option<u32>,
    /// s wrapping mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wrapS: Option<u32>,
    /// t wrapping mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wrapT: Option<u32>,
    /// The user-defined name of this object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Application-specific data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,
}

/// A typed view into a buffer view that contains raw binary data
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Accessor {
    /// The index of the bufferView
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bufferView: Option<usize>,
    /// The offset relative to the start of the bufferView in bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub byteOffset: Option<usize>,
    /// The datatype of components in the attribute
    pub componentType: u32,
    /// Specifies whether integer data values should be normalized
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normalized: Option<bool>,
    /// The number of attributes referenced by this accessor
    pub count: usize,
    /// Specifies if the attribute is a scalar, vector, or matrix
    pub type_: String,
    /// Maximum value of each component in this attribute
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<Vec<f32>>,
    /// Minimum value of each component in this attribute
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<Vec<f32>>,
    /// Specifies if the accessor's elements are sparse
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sparse: Option<AccessorSparse>,
    /// The user-defined name of this object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Application-specific data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,
}

/// Sparse storage of attributes that deviate from their initialization value
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AccessorSparse {
    /// Number of entries stored in the sparse array
    pub count: usize,
    /// Index array of size `count` that points to those accessor attributes that deviate from their initialization value
    pub indices: AccessorSparseIndices,
    /// Array of size `count` times number of components, storing the displaced accessor attributes
    pub values: AccessorSparseValues,
    /// Application-specific data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,
}

/// Indices of those attributes that deviate from their initialization value
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AccessorSparseIndices {
    /// The index of the bufferView with sparse indices
    pub bufferView: usize,
    /// The offset relative to the start of the bufferView in bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub byteOffset: Option<usize>,
    /// The indices data type
    pub componentType: u32,
    /// Application-specific data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,
}

/// Array of size `count` times number of components storing the displaced accessor attributes
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AccessorSparseValues {
    /// The index of the bufferView with sparse values
    pub bufferView: usize,
    /// The offset relative to the start of the bufferView in bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub byteOffset: Option<usize>,
    /// Application-specific data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,
}

/// A view into a buffer generally representing a subset of the buffer
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct BufferView {
    /// The index of the buffer
    pub buffer: usize,
    /// The offset into the buffer in bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub byteOffset: Option<usize>,
    /// The length of the bufferView in bytes
    pub byteLength: usize,
    /// The stride, in bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub byteStride: Option<usize>,
    /// The target that the GPU buffer should be bound to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<u32>,
    /// The user-defined name of this object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Application-specific data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,
}

/// A buffer points to binary geometry, animation, or skins
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Buffer {
    /// The URI (or IRI) of the buffer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    /// The length of the buffer in bytes
    pub byteLength: usize,
    /// The user-defined name of this object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Application-specific data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,
}

/// Mesh to glTF/GLB conversion functions
impl GltfRoot {
    /// Create a new default glTF root
    pub fn new() -> Self {
        Self::default()
    }

    /// Build a glTF scene from a mesh
    pub fn from_mesh(mesh: &Mesh, options: &GlbExportOptions) -> Self {
        let mut gltf = Self::new();
        
        // Setup binary buffer
        let mut buffer_views = Vec::new();
        let mut accessors = Vec::new();
        let mut binary_data = Vec::new();
        
        // Create primitive
        let mut primitive = Primitive::default();
        primitive.mode = Some(primitive_mode::TRIANGLES);
        
        // Add material if specified
        if let Some(material) = &options.material {
            let material_index = match &gltf.materials {
                Some(materials) => materials.len(),
                None => 0,
            };
            
            if gltf.materials.is_none() {
                gltf.materials = Some(Vec::new());
            }
            
            if let Some(materials) = &mut gltf.materials {
                materials.push(material.clone());
                primitive.material = Some(material_index);
            }
        }
        
        // Process vertex positions
        let positions_data = mesh.vertices.iter()
            .flat_map(|v| [v.position.x, v.position.y, v.position.z])
            .collect::<Vec<f32>>();
        
        let position_view_index = buffer_views.len();
        let position_accessor_index = accessors.len();
        
        // Find min/max bounds for positions
        let mut min_pos = [f32::MAX, f32::MAX, f32::MAX];
        let mut max_pos = [f32::MIN, f32::MIN, f32::MIN];
        
        for i in 0..mesh.vertices.len() {
            let v = &mesh.vertices[i];
            min_pos[0] = min_pos[0].min(v.position.x);
            min_pos[1] = min_pos[1].min(v.position.y);
            min_pos[2] = min_pos[2].min(v.position.z);
            
            max_pos[0] = max_pos[0].max(v.position.x);
            max_pos[1] = max_pos[1].max(v.position.y);
            max_pos[2] = max_pos[2].max(v.position.z);
        }
        
        // Add positions to binary data
        let positions_offset = binary_data.len();
        binary_data.extend_from_slice(bytemuck::cast_slice(&positions_data));
        
        // Add buffer view for positions
        buffer_views.push(BufferView {
            buffer: 0,
            byteOffset: Some(positions_offset),
            byteLength: positions_data.len() * std::mem::size_of::<f32>(),
            byteStride: None,
            target: Some(buffer_target::ARRAY_BUFFER),
            name: Some("positions".to_string()),
            extras: None,
        });
        
        // Add accessor for positions
        accessors.push(Accessor {
            bufferView: Some(position_view_index),
            byteOffset: None,
            componentType: component_type::FLOAT,
            normalized: None,
            count: mesh.vertices.len(),
            type_: "VEC3".to_string(),
            max: Some(vec![max_pos[0], max_pos[1], max_pos[2]]),
            min: Some(vec![min_pos[0], min_pos[1], min_pos[2]]),
            sparse: None,
            name: Some("positions".to_string()),
            extras: None,
        });
        
        primitive.attributes.insert("POSITION".to_string(), position_accessor_index);
        
        // Process normals if requested
        if options.include_normals && mesh.vertices.iter().any(|v| v.normal.is_some()) {
            let normals_data = mesh.vertices.iter()
                .flat_map(|v| {
                    let n = v.normal.unwrap_or(Vec3::new(0.0, 1.0, 0.0)); // Default to up if missing
                    [n.x, n.y, n.z]
                })
                .collect::<Vec<f32>>();
            
            let normal_view_index = buffer_views.len();
            let normal_accessor_index = accessors.len();
            
            // Add normals to binary data
            let normals_offset = binary_data.len();
            binary_data.extend_from_slice(bytemuck::cast_slice(&normals_data));
            
            // Add buffer view for normals
            buffer_views.push(BufferView {
                buffer: 0,
                byteOffset: Some(normals_offset),
                byteLength: normals_data.len() * std::mem::size_of::<f32>(),
                byteStride: None,
                target: Some(buffer_target::ARRAY_BUFFER),
                name: Some("normals".to_string()),
                extras: None,
            });
            
            // Add accessor for normals
            accessors.push(Accessor {
                bufferView: Some(normal_view_index),
                byteOffset: None,
                componentType: component_type::FLOAT,
                normalized: None,
                count: mesh.vertices.len(),
                type_: "VEC3".to_string(),
                max: None,
                min: None,
                sparse: None,
                name: Some("normals".to_string()),
                extras: None,
            });
            
            primitive.attributes.insert("NORMAL".to_string(), normal_accessor_index);
        }
        
        // Process UVs if requested
        if options.include_uvs && mesh.vertices.iter().any(|v| v.uv.is_some()) {
            let uvs_data = mesh.vertices.iter()
                .flat_map(|v| {
                    let uv = v.uv.unwrap_or(Vec2::new(0.0, 0.0)); // Default to (0,0) if missing
                    [uv.x, uv.y]
                })
                .collect::<Vec<f32>>();
            
            let uv_view_index = buffer_views.len();
            let uv_accessor_index = accessors.len();
            
            // Add UVs to binary data
            let uvs_offset = binary_data.len();
            binary_data.extend_from_slice(bytemuck::cast_slice(&uvs_data));
            
            // Add buffer view for UVs
            buffer_views.push(BufferView {
                buffer: 0,
                byteOffset: Some(uvs_offset),
                byteLength: uvs_data.len() * std::mem::size_of::<f32>(),
                byteStride: None,
                target: Some(buffer_target::ARRAY_BUFFER),
                name: Some("texcoords".to_string()),
                extras: None,
            });
            
            // Add accessor for UVs
            accessors.push(Accessor {
                bufferView: Some(uv_view_index),
                byteOffset: None,
                componentType: component_type::FLOAT,
                normalized: None,
                count: mesh.vertices.len(),
                type_: "VEC2".to_string(),
                max: None,
                min: None,
                sparse: None,
                name: Some("texcoords".to_string()),
                extras: None,
            });
            
            primitive.attributes.insert("TEXCOORD_0".to_string(), uv_accessor_index);
        }
        
        // Process tangents if requested
        if options.include_tangents && mesh.vertices.iter().any(|v| v.tangent.is_some()) {
            let tangents_data = mesh.vertices.iter()
                .flat_map(|v| {
                    let t = v.tangent.unwrap_or(Vec4::new(1.0, 0.0, 0.0, 1.0)); // Default tangent if missing
                    [t.x, t.y, t.z, t.w]
                })
                .collect::<Vec<f32>>();
            
            let tangent_view_index = buffer_views.len();
            let tangent_accessor_index = accessors.len();
            
            // Add tangents to binary data
            let tangents_offset = binary_data.len();
            binary_data.extend_from_slice(bytemuck::cast_slice(&tangents_data));
            
            // Add buffer view for tangents
            buffer_views.push(BufferView {
                buffer: 0,
                byteOffset: Some(tangents_offset),
                byteLength: tangents_data.len() * std::mem::size_of::<f32>(),
                byteStride: None,
                target: Some(buffer_target::ARRAY_BUFFER),
                name: Some("tangents".to_string()),
                extras: None,
            });
            
            // Add accessor for tangents
            accessors.push(Accessor {
                bufferView: Some(tangent_view_index),
                byteOffset: None,
                componentType: component_type::FLOAT,
                normalized: None,
                count: mesh.vertices.len(),
                type_: "VEC4".to_string(),
                max: None,
                min: None,
                sparse: None,
                name: Some("tangents".to_string()),
                extras: None,
            });
            
            primitive.attributes.insert("TANGENT".to_string(), tangent_accessor_index);
        }
        
        // Process colors if requested
        if options.include_colors && mesh.vertices.iter().any(|v| v.color.is_some()) {
            let colors_data = mesh.vertices.iter()
                .flat_map(|v| {
                    let c = v.color.unwrap_or(Vec4::new(1.0, 1.0, 1.0, 1.0)); // Default white if missing
                    [c.x, c.y, c.z, c.w]
                })
                .collect::<Vec<f32>>();
            
            let color_view_index = buffer_views.len();
            let color_accessor_index = accessors.len();
            
            // Add colors to binary data
            let colors_offset = binary_data.len();
            binary_data.extend_from_slice(bytemuck::cast_slice(&colors_data));
            
            // Add buffer view for colors
            buffer_views.push(BufferView {
                buffer: 0,
                byteOffset: Some(colors_offset),
                byteLength: colors_data.len() * std::mem::size_of::<f32>(),
                byteStride: None,
                target: Some(buffer_target::ARRAY_BUFFER),
                name: Some("colors".to_string()),
                extras: None,
            });
            
            // Add accessor for colors
            accessors.push(Accessor {
                bufferView: Some(color_view_index),
                byteOffset: None,
                componentType: component_type::FLOAT,
                normalized: None,
                count: mesh.vertices.len(),
                type_: "VEC4".to_string(),
                max: None,
                min: None,
                sparse: None,
                name: Some("colors".to_string()),
                extras: None,
            });
            
            primitive.attributes.insert("COLOR_0".to_string(), color_accessor_index);
        }
        
        // Process indices
        let indices_view_index = buffer_views.len();
        let indices_accessor_index = accessors.len();
        
        // Determine if we need u16 or u32 indices
        let use_u16 = mesh.vertices.len() <= 65535;
        let indices_offset = binary_data.len();
        
        if use_u16 {
            // Convert to u16 indices
            let indices_u16 = mesh.triangles.iter()
                .flat_map(|t| [t.0, t.1, t.2].map(|i| i as u16))
                .collect::<Vec<u16>>();
            
            binary_data.extend_from_slice(bytemuck::cast_slice(&indices_u16));
            
            // Add buffer view for indices
            buffer_views.push(BufferView {
                buffer: 0,
                byteOffset: Some(indices_offset),
                byteLength: indices_u16.len() * std::mem::size_of::<u16>(),
                byteStride: None,
                target: Some(buffer_target::ELEMENT_ARRAY_BUFFER),
                name: Some("indices".to_string()),
                extras: None,
            });
            
            // Add accessor for indices
            accessors.push(Accessor {
                bufferView: Some(indices_view_index),
                byteOffset: None,
                componentType: component_type::UNSIGNED_SHORT,
                normalized: None,
                count: mesh.triangles.len() * 3,
                type_: "SCALAR".to_string(),
                max: None,
                min: None,
                sparse: None,
                name: Some("indices".to_string()),
                extras: None,
            });
        } else {
            // Use u32 indices
            let indices_u32 = mesh.triangles.iter()
                .flat_map(|t| [t.0, t.1, t.2])
                .collect::<Vec<u32>>();
            
            binary_data.extend_from_slice(bytemuck::cast_slice(&indices_u32));
            
            // Add buffer view for indices
            buffer_views.push(BufferView {
                buffer: 0,
                byteOffset: Some(indices_offset),
                byteLength: indices_u32.len() * std::mem::size_of::<u32>(),
                byteStride: None,
                target: Some(buffer_target::ELEMENT_ARRAY_BUFFER),
                name: Some("indices".to_string()),
                extras: None,
            });
            
            // Add accessor for indices
            accessors.push(Accessor {
                bufferView: Some(indices_view_index),
                byteOffset: None,
                componentType: component_type::UNSIGNED_INT,
                normalized: None,
                count: mesh.triangles.len() * 3,
                type_: "SCALAR".to_string(),
                max: None,
                min: None,
                sparse: None,
                name: Some("indices".to_string()),
                extras: None,
            });
        }
        
        primitive.indices = Some(indices_accessor_index);
        
        // Create mesh
        let gltf_mesh = GltfMesh {
            primitives: vec![primitive],
            weights: None,
            name: Some("mesh".to_string()),
            extras: None,
        };
        
        // Create node
        let node = Node {
            mesh: Some(0),
            name: Some("node".to_string()),
            ..Default::default()
        };
        
        // Create scene
        let scene = Scene {
            nodes: Some(vec![0]),
            name: Some("scene".to_string()),
            extras: None,
        };
        
        // Assemble glTF
        gltf.scenes = Some(vec![scene]);
        gltf.scene = Some(0);
        gltf.nodes = Some(vec![node]);
        gltf.meshes = Some(vec![gltf_mesh]);
        gltf.accessors = Some(accessors);
        gltf.bufferViews = Some(buffer_views);
        gltf.buffers = Some(vec![Buffer {
            uri: None, // No URI for GLB embedded buffer
            byteLength: binary_data.len(),
            name: Some("buffer".to_string()),
            extras: None,
        }]);
        
        gltf
    }
}

/// GLB export functions
pub fn export_to_glb<P: AsRef<Path>>(mesh: &Mesh, path: P) -> io::Result<()> {
    export_to_glb_with_options(mesh, path, &GlbExportOptions::default())
}

/// Export mesh to GLB file with custom options
pub fn export_to_glb_with_options<P: AsRef<Path>>(mesh: &Mesh, path: P, options: &GlbExportOptions) -> io::Result<()> {
    let gltf = GltfRoot::from_mesh(mesh, options);
    
    // Create writer
    let mut writer = GlbWriter::new();
    
    // Convert JSON
    let json = serde_json::to_string(&gltf)?;
    writer.set_json(json);
    
    // Add binary data
    // NOTE: In a real implementation, we'd need to extract the actual binary data here
    // For now, I'm simulating this by creating a vector of the right size
    let binary_data_size = gltf.buffers.as_ref()
        .and_then(|buffers| buffers.first())
        .map(|buffer| buffer.byteLength)
        .unwrap_or(0);
    
    let binary_data = mesh.vertices.iter()
        .flat_map(|v| [v.position.x, v.position.y, v.position.z])
        .collect::<Vec<f32>>();
    
    // Add indices data
    let indices_data = mesh.triangles.iter()
        .flat_map(|t| [t.0, t.1, t.2])
        .collect::<Vec<u32>>();
    
    // Serialize all the vertex and index data
    let mut buffer_data: Vec<u8> = Vec::with_capacity(binary_data_size);
    buffer_data.extend_from_slice(bytemuck::cast_slice(&binary_data));
    
    // Add indices
    if mesh.vertices.len() <= 65535 {
        // Convert to u16 indices
        let indices_u16 = indices_data.iter().map(|&i| i as u16).collect::<Vec<u16>>();
        buffer_data.extend_from_slice(bytemuck::cast_slice(&indices_u16));
    } else {
        buffer_data.extend_from_slice(bytemuck::cast_slice(&indices_data));
    }
    
    // Add additional attribute data as needed
    // (We'd need to generate binary data for normals, UVs, etc. based on the options)
    
    // Write to file
    let mut file = File::create(path)?;
    writer.write(&mut file)?;
    
    Ok(())
}

/// Export mesh to glTF text file
pub fn export_to_gltf<P: AsRef<Path>>(mesh: &Mesh, path: P) -> io::Result<()> {
    export_to_gltf_with_options(mesh, path, &GlbExportOptions::default())
}

/// Export mesh to glTF text file with custom options
pub fn export_to_gltf_with_options<P: AsRef<Path>>(mesh: &Mesh, path: P, options: &GlbExportOptions) -> io::Result<()> {
    let gltf = GltfRoot::from_mesh(mesh, options);
    
    // Convert JSON with pretty formatting
    let json = serde_json::to_string_pretty(&gltf)?;
    
    // Write to file
    let mut file = File::create(path)?;
    file.write_all(json.as_bytes())?;
    
    Ok(())
}

/// Trait for exporting meshes to glTF/GLB
pub trait ExportMesh {
    /// Export mesh to GLB file
    fn export_glb<P: AsRef<Path>>(&self, path: P) -> io::Result<()>;
    
    /// Export mesh to GLB file with custom options
    fn export_glb_with_options<P: AsRef<Path>>(&self, path: P, options: &GlbExportOptions) -> io::Result<()>;
    
    /// Export mesh to glTF text file
    fn export_gltf<P: AsRef<Path>>(&self, path: P) -> io::Result<()>;
    
    /// Export mesh to glTF text file with custom options
    fn export_gltf_with_options<P: AsRef<Path>>(&self, path: P, options: &GlbExportOptions) -> io::Result<()>;
}

impl ExportMesh for Mesh {
    fn export_glb<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        export_to_glb(self, path)
    }
    
    fn export_glb_with_options<P: AsRef<Path>>(&self, path: P, options: &GlbExportOptions) -> io::Result<()> {
        export_to_glb_with_options(self, path, options)
    }
    
    fn export_gltf<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        export_to_gltf(self, path)
    }
    
    fn export_gltf_with_options<P: AsRef<Path>>(&self, path: P, options: &GlbExportOptions) -> io::Result<()> {
        export_to_gltf_with_options(self, path, options)
    }
}