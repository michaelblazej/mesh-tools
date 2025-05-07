//! # Material Creation and Management
//!
//! This module provides utilities for creating and configuring materials in the glTF format.
//! Materials in glTF use the Physically Based Rendering (PBR) model which simulates how light
//! interacts with surfaces in a physically accurate way.
//!
//! The core of this module is the `MaterialBuilder` struct which provides a builder pattern
//! for creating materials with various properties:
//!
//! - Base color (diffuse color)
//! - Metallic and roughness factors
//! - Normal, occlusion, and emissive textures
//! - Transparency settings
//! - Double-sided rendering
//!
//! ## Example
//!
//! ```rust
//! use gltf_export::material;
//!
//! // Create a red metallic material
//! let red_metal = material::create_metallic_material(
//!     Some("RedMetal".to_string()),
//!     [1.0, 0.0, 0.0, 1.0], // Red color
//!     0.9, // High metallic
//!     0.2  // Low roughness (shiny)
//! );
//! ```

use crate::models::{Material, NormalTextureInfo, OcclusionTextureInfo, PbrMetallicRoughness, TextureInfo};

/// Builder for creating and configuring glTF materials with PBR properties
pub struct MaterialBuilder {
    pub material: Material,
}

impl MaterialBuilder {
    /// Create a new material builder
    pub fn new(name: Option<String>) -> Self {
        let mut material = Material::default();
        material.name = name;
        
        Self { material }
    }
    
    /// Set base color factor
    pub fn with_base_color(mut self, color: [f32; 4]) -> Self {
        if self.material.pbrMetallicRoughness.is_none() {
            self.material.pbrMetallicRoughness = Some(PbrMetallicRoughness::default());
        }
        
        if let Some(pbr) = &mut self.material.pbrMetallicRoughness {
            pbr.baseColorFactor = Some(color);
        }
        
        self
    }
    
    /// Set metallic factor
    pub fn with_metallic_factor(mut self, factor: f32) -> Self {
        if self.material.pbrMetallicRoughness.is_none() {
            self.material.pbrMetallicRoughness = Some(PbrMetallicRoughness::default());
        }
        
        if let Some(pbr) = &mut self.material.pbrMetallicRoughness {
            pbr.metallicFactor = Some(factor);
        }
        
        self
    }
    
    /// Set roughness factor
    pub fn with_roughness_factor(mut self, factor: f32) -> Self {
        if self.material.pbrMetallicRoughness.is_none() {
            self.material.pbrMetallicRoughness = Some(PbrMetallicRoughness::default());
        }
        
        if let Some(pbr) = &mut self.material.pbrMetallicRoughness {
            pbr.roughnessFactor = Some(factor);
        }
        
        self
    }
    
    /// Set base color texture
    pub fn with_base_color_texture(mut self, texture_index: usize, tex_coord: Option<usize>) -> Self {
        if self.material.pbrMetallicRoughness.is_none() {
            self.material.pbrMetallicRoughness = Some(PbrMetallicRoughness::default());
        }
        
        if let Some(pbr) = &mut self.material.pbrMetallicRoughness {
            let mut texture_info = TextureInfo::default();
            texture_info.index = texture_index;
            texture_info.texCoord = tex_coord;
            
            pbr.baseColorTexture = Some(texture_info);
        }
        
        self
    }
    
    /// Set metallic roughness texture
    pub fn with_metallic_roughness_texture(mut self, texture_index: usize, tex_coord: Option<usize>) -> Self {
        if self.material.pbrMetallicRoughness.is_none() {
            self.material.pbrMetallicRoughness = Some(PbrMetallicRoughness::default());
        }
        
        if let Some(pbr) = &mut self.material.pbrMetallicRoughness {
            let mut texture_info = TextureInfo::default();
            texture_info.index = texture_index;
            texture_info.texCoord = tex_coord;
            
            pbr.metallicRoughnessTexture = Some(texture_info);
        }
        
        self
    }
    
    /// Set normal texture
    pub fn with_normal_texture(mut self, texture_index: usize, tex_coord: Option<usize>, scale: Option<f32>) -> Self {
        let mut normal_info = NormalTextureInfo::default();
        normal_info.index = texture_index;
        normal_info.texCoord = tex_coord;
        normal_info.scale = scale;
        
        self.material.normalTexture = Some(normal_info);
        
        self
    }
    
    /// Set occlusion texture
    pub fn with_occlusion_texture(mut self, texture_index: usize, tex_coord: Option<usize>, strength: Option<f32>) -> Self {
        let mut occlusion_info = OcclusionTextureInfo::default();
        occlusion_info.index = texture_index;
        occlusion_info.texCoord = tex_coord;
        occlusion_info.strength = strength;
        
        self.material.occlusionTexture = Some(occlusion_info);
        
        self
    }
    
    /// Set emissive texture
    pub fn with_emissive_texture(mut self, texture_index: usize, tex_coord: Option<usize>) -> Self {
        let mut texture_info = TextureInfo::default();
        texture_info.index = texture_index;
        texture_info.texCoord = tex_coord;
        
        self.material.emissiveTexture = Some(texture_info);
        
        self
    }
    
    /// Set emissive factor
    pub fn with_emissive_factor(mut self, factor: [f32; 3]) -> Self {
        self.material.emissiveFactor = Some(factor);
        self
    }
    
    /// Set alpha mode and cutoff
    pub fn with_alpha_mode(mut self, mode: String, cutoff: Option<f32>) -> Self {
        self.material.alphaMode = Some(mode);
        self.material.alphaCutoff = cutoff;
        self
    }
    
    /// Set double sided flag
    pub fn with_double_sided(mut self, double_sided: bool) -> Self {
        self.material.doubleSided = Some(double_sided);
        self
    }
    
    /// Build the material
    pub fn build(self) -> Material {
        self.material
    }
}

/// Create a basic material with the specified color
pub fn create_basic_material(name: Option<String>, color: [f32; 4]) -> Material {
    MaterialBuilder::new(name)
        .with_base_color(color)
        .build()
}

/// Create a metallic material
pub fn create_metallic_material(
    name: Option<String>, 
    color: [f32; 4], 
    metallic: f32,
    roughness: f32
) -> Material {
    MaterialBuilder::new(name)
        .with_base_color(color)
        .with_metallic_factor(metallic)
        .with_roughness_factor(roughness)
        .build()
}

/// Create a textured material with additional options
pub fn create_textured_material(
    name: Option<String>,
    base_color_texture: Option<usize>,
    metallic_roughness_texture: Option<usize>,
    normal_texture: Option<usize>,
    occlusion_texture: Option<usize>,
    emissive_texture: Option<usize>,
    emissive_factor: Option<[f32; 3]>,
    metallic_factor: Option<f32>,
    roughness_factor: Option<f32>,
    alpha_mode: Option<String>,
    alpha_cutoff: Option<f32>,
    double_sided: Option<bool>
) -> Material {
    let mut builder = MaterialBuilder::new(name);
    
    if let Some(texture) = base_color_texture {
        builder = builder.with_base_color_texture(texture, None);
    }
    
    if let Some(texture) = metallic_roughness_texture {
        builder = builder.with_metallic_roughness_texture(texture, None);
    }
    
    if let Some(texture) = normal_texture {
        builder = builder.with_normal_texture(texture, None, None);
    }
    
    if let Some(texture) = occlusion_texture {
        builder = builder.with_occlusion_texture(texture, None, None);
    }
    
    if let Some(texture) = emissive_texture {
        builder = builder.with_emissive_texture(texture, None);
    }
    
    if let Some(factor) = emissive_factor {
        builder = builder.with_emissive_factor(factor);
    }
    
    if let Some(factor) = metallic_factor {
        builder = builder.with_metallic_factor(factor);
    }
    
    if let Some(factor) = roughness_factor {
        builder = builder.with_roughness_factor(factor);
    }
    
    if let Some(mode) = alpha_mode {
        builder = builder.with_alpha_mode(mode, alpha_cutoff);
    }
    
    if let Some(ds) = double_sided {
        builder = builder.with_double_sided(ds);
    }
    
    builder.build()
}
