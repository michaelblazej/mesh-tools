use crate::builder::GltfBuilder;
use crate::material;

impl GltfBuilder {
    /// Add a material to the glTF document
    pub fn add_material(&mut self, name: Option<String>, 
                        base_color: Option<[f32; 4]>,
                        metallic_factor: Option<f32>,
                        roughness_factor: Option<f32>,
                        double_sided: Option<bool>) -> usize {
        let mut builder = material::MaterialBuilder::new(name);
        
        if let Some(color) = base_color {
            builder = builder.with_base_color(color);
        }
        
        if let Some(metallic) = metallic_factor {
            builder = builder.with_metallic_factor(metallic);
        }
        
        if let Some(roughness) = roughness_factor {
            builder = builder.with_roughness_factor(roughness);
        }
        
        if let Some(double_sided) = double_sided {
            builder = builder.with_double_sided(double_sided);
        }
        
        let material = builder.build();
        
        if let Some(materials) = &mut self.gltf.materials {
            let index = materials.len();
            materials.push(material);
            index
        } else {
            self.gltf.materials = Some(vec![material]);
            0
        }
    }

    /// Create a basic material with the specified color
    pub fn create_basic_material(&mut self, name: Option<String>, color: [f32; 4]) -> usize {
        let material = material::create_basic_material(name, color);
        
        if let Some(materials) = &mut self.gltf.materials {
            let index = materials.len();
            materials.push(material);
            index
        } else {
            self.gltf.materials = Some(vec![material]);
            0
        }
    }

    /// Create a metallic material
    pub fn create_metallic_material(&mut self, name: Option<String>, 
                                   color: [f32; 4], 
                                   metallic: f32,
                                   roughness: f32) -> usize {
        let material = material::create_metallic_material(name, color, metallic, roughness);
        
        if let Some(materials) = &mut self.gltf.materials {
            let index = materials.len();
            materials.push(material);
            index
        } else {
            self.gltf.materials = Some(vec![material]);
            0
        }
    }
}
