use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a glTF 2.0 document
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Gltf {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset: Option<Asset>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenes: Option<Vec<Scene>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene: Option<usize>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<Node>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meshes: Option<Vec<Mesh>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accessors: Option<Vec<Accessor>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bufferViews: Option<Vec<BufferView>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffers: Option<Vec<Buffer>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub materials: Option<Vec<Material>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub textures: Option<Vec<Texture>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<Image>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub samplers: Option<Vec<Sampler>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensionsUsed: Option<Vec<String>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensionsRequired: Option<Vec<String>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<HashMap<String, serde_json::Value>>,
}

/// Represents the glTF asset information
#[derive(Serialize, Deserialize, Debug)]
pub struct Asset {
    pub version: String,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generator: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copyright: Option<String>,
}

/// Represents a glTF scene
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Scene {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<usize>>,
}

/// Represents a glTF node
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Node {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh: Option<usize>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translation: Option<[f32; 3]>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation: Option<[f32; 4]>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<[f32; 3]>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matrix: Option<[f32; 16]>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<usize>>,
}

/// Represents a glTF mesh
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Mesh {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    
    pub primitives: Vec<Primitive>,
}

/// Represents a glTF mesh primitive
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Primitive {
    pub attributes: HashMap<String, usize>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indices: Option<usize>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub material: Option<usize>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<usize>,
}

/// Represents a glTF accessor
#[derive(Serialize, Deserialize, Debug)]
pub struct Accessor {
    pub bufferView: usize,
    pub componentType: usize,
    pub count: usize,
    #[serde(rename = "type")]
    pub type_: String,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub byteOffset: Option<usize>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<Vec<f32>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<Vec<f32>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normalized: Option<bool>,
}

/// Represents a glTF buffer view
#[derive(Serialize, Deserialize, Debug)]
pub struct BufferView {
    pub buffer: usize,
    pub byteOffset: usize,
    pub byteLength: usize,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub byteStride: Option<usize>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<usize>,
}

/// Represents a glTF buffer
#[derive(Serialize, Deserialize, Debug)]
pub struct Buffer {
    pub byteLength: usize,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

/// Represents a glTF material
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Material {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pbrMetallicRoughness: Option<PbrMetallicRoughness>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normalTexture: Option<NormalTextureInfo>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub occlusionTexture: Option<OcclusionTextureInfo>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emissiveTexture: Option<TextureInfo>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emissiveFactor: Option<[f32; 3]>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alphaMode: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alphaCutoff: Option<f32>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doubleSided: Option<bool>,
}

/// Represents a glTF PBR material
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PbrMetallicRoughness {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseColorFactor: Option<[f32; 4]>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseColorTexture: Option<TextureInfo>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metallicFactor: Option<f32>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roughnessFactor: Option<f32>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metallicRoughnessTexture: Option<TextureInfo>,
}

/// Represents basic texture reference information
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TextureInfo {
    pub index: usize,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub texCoord: Option<usize>,
}

/// Represents normal texture reference information
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct NormalTextureInfo {
    pub index: usize,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub texCoord: Option<usize>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<f32>,
}

/// Represents occlusion texture reference information
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct OcclusionTextureInfo {
    pub index: usize,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub texCoord: Option<usize>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength: Option<f32>,
}

/// Represents a glTF texture
#[derive(Serialize, Deserialize, Debug)]
pub struct Texture {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    
    pub source: usize,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampler: Option<usize>,
}

/// Represents a glTF image
#[derive(Serialize, Deserialize, Debug)]
pub struct Image {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mimeType: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bufferView: Option<usize>,
}

/// Represents a glTF sampler
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Sampler {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub magFilter: Option<usize>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minFilter: Option<usize>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wrapS: Option<usize>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wrapT: Option<usize>,
}
