use crate::builder::GltfBuilder;
use crate::models::{Animation, AnimationChannel, AnimationChannelTarget, AnimationSampler};

/// Animation path type
pub enum AnimationPath {
    Translation,
    Rotation,
    Scale,
    Weights,
}

impl AnimationPath {
    /// Convert to string representation for glTF
    pub fn to_string(&self) -> String {
        match self {
            AnimationPath::Translation => "translation".to_string(),
            AnimationPath::Rotation => "rotation".to_string(),
            AnimationPath::Scale => "scale".to_string(),
            AnimationPath::Weights => "weights".to_string(),
        }
    }
}

/// Interpolation method for animation
pub enum InterpolationType {
    Linear,
    Step,
    CubicSpline,
}

impl InterpolationType {
    /// Convert to string representation for glTF
    pub fn to_string(&self) -> String {
        match self {
            InterpolationType::Linear => "LINEAR".to_string(),
            InterpolationType::Step => "STEP".to_string(),
            InterpolationType::CubicSpline => "CUBICSPLINE".to_string(),
        }
    }
}

impl GltfBuilder {
    /// Add an animation to the glTF
    /// 
    /// # Arguments
    /// 
    /// * `name` - Optional name for the animation
    /// 
    /// # Returns
    /// 
    /// The index of the created animation
    pub fn add_animation(&mut self, name: Option<String>) -> usize {
        let animation = Animation {
            name,
            channels: Some(Vec::new()),
            samplers: Some(Vec::new()),
        };
        
        if let Some(animations) = &mut self.gltf.animations {
            let index = animations.len();
            animations.push(animation);
            index
        } else {
            self.gltf.animations = Some(vec![animation]);
            0
        }
    }
    
    /// Add a sampler to an animation
    /// 
    /// # Arguments
    /// 
    /// * `animation_index` - The index of the animation to add the sampler to
    /// * `input_accessor` - The accessor containing keyframe timestamps (in seconds)
    /// * `output_accessor` - The accessor containing output values
    /// * `interpolation` - The interpolation method
    /// 
    /// # Returns
    /// 
    /// The index of the created sampler within the animation
    pub fn add_animation_sampler(
        &mut self, 
        animation_index: usize, 
        input_accessor: usize, 
        output_accessor: usize,
        interpolation: InterpolationType
    ) -> usize {
        let sampler = AnimationSampler {
            input: input_accessor,
            interpolation: Some(interpolation.to_string()),
            output: output_accessor,
        };
        
        let animations = self.gltf.animations.as_mut().expect("Animations array not initialized");
        if animation_index >= animations.len() {
            panic!("Animation index out of bounds");
        }
        
        let animation = &mut animations[animation_index];
        let samplers = animation.samplers.get_or_insert_with(|| Vec::new());
        let sampler_index = samplers.len();
        samplers.push(sampler);
        
        sampler_index
    }
    
    /// Add a channel to an animation
    /// 
    /// # Arguments
    /// 
    /// * `animation_index` - The index of the animation to add the channel to
    /// * `sampler_index` - The index of the sampler within the animation
    /// * `target_node` - The index of the node being animated
    /// * `target_path` - The property being animated (translation, rotation, scale, weights)
    /// 
    /// # Returns
    /// 
    /// The index of the created channel within the animation
    pub fn add_animation_channel(
        &mut self, 
        animation_index: usize, 
        sampler_index: usize, 
        target_node: usize, 
        target_path: AnimationPath
    ) -> usize {
        let channel = AnimationChannel {
            sampler: sampler_index,
            target: AnimationChannelTarget {
                node: target_node,
                path: target_path.to_string(),
            }
        };
        
        let animations = self.gltf.animations.as_mut().expect("Animations array not initialized");
        if animation_index >= animations.len() {
            panic!("Animation index out of bounds");
        }
        
        let animation = &mut animations[animation_index];
        let channels = animation.channels.get_or_insert_with(|| Vec::new());
        let channel_index = channels.len();
        channels.push(channel);
        
        channel_index
    }
    
    /// Create translation keyframes for an animation
    /// 
    /// # Arguments
    /// 
    /// * `animation_index` - The index of the animation
    /// * `node_index` - The index of the target node
    /// * `timestamps` - Vector of keyframe timestamps (in seconds)
    /// * `translations` - Vector of translation values ([x, y, z] for each keyframe)
    /// * `interpolation` - The interpolation method
    /// 
    /// # Returns
    /// 
    /// The indices of the created channel and sampler
    pub fn create_translation_animation(
        &mut self,
        animation_index: usize,
        node_index: usize,
        timestamps: Vec<f32>,
        translations: Vec<[f32; 3]>,
        interpolation: InterpolationType,
    ) -> (usize, usize) {
        if timestamps.len() != translations.len() {
            panic!("Timestamps and translations must have the same length");
        }
        
        // Create time input accessor
        let timestamps_data: Vec<u8> = timestamps.iter().flat_map(|&t| t.to_le_bytes()).collect();
        
        // Add buffer data and create buffer view
        let (time_offset, time_length) = self.add_buffer_data(&timestamps_data);
        let time_buffer_view = self.add_buffer_view(
            time_offset,
            time_length,
            None
        );
        
        // Create input accessor (timestamps)
        let input_accessor = self.add_accessor(
            time_buffer_view,
            5126, // FLOAT component type
            timestamps.len(),
            "SCALAR".to_string(),
            None,
            None,
            None
        );
        
        // Create translation output accessor
        let translations_data: Vec<u8> = translations.iter().flat_map(|t| t.iter().flat_map(|&v| v.to_le_bytes())).collect();
        
        // Add buffer data and create buffer view
        let (trans_offset, trans_length) = self.add_buffer_data(&translations_data);
        let trans_buffer_view = self.add_buffer_view(
            trans_offset,
            trans_length,
            None
        );
        
        // Create output accessor (translations)
        let output_accessor = self.add_accessor(
            trans_buffer_view,
            5126, // FLOAT component type
            translations.len(),
            "VEC3".to_string(),
            None,
            None,
            None
        );
        
        // Create sampler and channel
        let sampler_index = self.add_animation_sampler(
            animation_index,
            input_accessor,
            output_accessor,
            interpolation,
        );
        
        let channel_index = self.add_animation_channel(
            animation_index,
            sampler_index,
            node_index,
            AnimationPath::Translation,
        );
        
        (channel_index, sampler_index)
    }
    
    /// Create rotation keyframes for an animation
    /// 
    /// # Arguments
    /// 
    /// * `animation_index` - The index of the animation
    /// * `node_index` - The index of the target node
    /// * `timestamps` - Vector of keyframe timestamps (in seconds)
    /// * `rotations` - Vector of rotation quaternions ([x, y, z, w] for each keyframe)
    /// * `interpolation` - The interpolation method
    /// 
    /// # Returns
    /// 
    /// The indices of the created channel and sampler
    pub fn create_rotation_animation(
        &mut self,
        animation_index: usize,
        node_index: usize,
        timestamps: Vec<f32>,
        rotations: Vec<[f32; 4]>,
        interpolation: InterpolationType,
    ) -> (usize, usize) {
        if timestamps.len() != rotations.len() {
            panic!("Timestamps and rotations must have the same length");
        }
        
        // Create time input accessor
        let timestamps_data: Vec<u8> = timestamps.iter().flat_map(|&t| t.to_le_bytes()).collect();
        
        // Add buffer data and create buffer view
        let (time_offset, time_length) = self.add_buffer_data(&timestamps_data);
        let time_buffer_view = self.add_buffer_view(
            time_offset,
            time_length,
            None
        );
        
        // Create input accessor (timestamps)
        let input_accessor = self.add_accessor(
            time_buffer_view,
            5126, // FLOAT component type
            timestamps.len(),
            "SCALAR".to_string(),
            None,
            None,
            None
        );
        
        // Create rotation output accessor
        let rotations_data: Vec<u8> = rotations.iter().flat_map(|q| q.iter().flat_map(|&v| v.to_le_bytes())).collect();
        
        // Add buffer data and create buffer view
        let (rot_offset, rot_length) = self.add_buffer_data(&rotations_data);
        let rot_buffer_view = self.add_buffer_view(
            rot_offset,
            rot_length,
            None
        );
        
        // Create output accessor (rotations)
        let output_accessor = self.add_accessor(
            rot_buffer_view,
            5126, // FLOAT component type
            rotations.len(),
            "VEC4".to_string(),
            None,
            None,
            None
        );
        
        // Create sampler and channel
        let sampler_index = self.add_animation_sampler(
            animation_index,
            input_accessor,
            output_accessor,
            interpolation,
        );
        
        let channel_index = self.add_animation_channel(
            animation_index,
            sampler_index,
            node_index,
            AnimationPath::Rotation,
        );
        
        (channel_index, sampler_index)
    }
    
    /// Create scale keyframes for an animation
    /// 
    /// # Arguments
    /// 
    /// * `animation_index` - The index of the animation
    /// * `node_index` - The index of the target node
    /// * `timestamps` - Vector of keyframe timestamps (in seconds)
    /// * `scales` - Vector of scale values ([x, y, z] for each keyframe)
    /// * `interpolation` - The interpolation method
    /// 
    /// # Returns
    /// 
    /// The indices of the created channel and sampler
    pub fn create_scale_animation(
        &mut self,
        animation_index: usize,
        node_index: usize,
        timestamps: Vec<f32>,
        scales: Vec<[f32; 3]>,
        interpolation: InterpolationType,
    ) -> (usize, usize) {
        if timestamps.len() != scales.len() {
            panic!("Timestamps and scales must have the same length");
        }
        
        // Create time input accessor
        let timestamps_data: Vec<u8> = timestamps.iter().flat_map(|&t| t.to_le_bytes()).collect();
        
        // Add buffer data and create buffer view
        let (time_offset, time_length) = self.add_buffer_data(&timestamps_data);
        let time_buffer_view = self.add_buffer_view(
            time_offset,
            time_length,
            None
        );
        
        // Create input accessor (timestamps)
        let input_accessor = self.add_accessor(
            time_buffer_view,
            5126, // FLOAT component type
            timestamps.len(),
            "SCALAR".to_string(),
            None,
            None,
            None
        );
        
        // Create scale output accessor
        let scales_data: Vec<u8> = scales.iter().flat_map(|s| s.iter().flat_map(|&v| v.to_le_bytes())).collect();
        
        // Add buffer data and create buffer view
        let (scale_offset, scale_length) = self.add_buffer_data(&scales_data);
        let scale_buffer_view = self.add_buffer_view(
            scale_offset,
            scale_length,
            None
        );
        
        // Create output accessor (scales)
        let output_accessor = self.add_accessor(
            scale_buffer_view,
            5126, // FLOAT component type
            scales.len(),
            "VEC3".to_string(),
            None,
            None,
            None
        );
        
        // Create sampler and channel
        let sampler_index = self.add_animation_sampler(
            animation_index,
            input_accessor,
            output_accessor,
            interpolation,
        );
        
        let channel_index = self.add_animation_channel(
            animation_index,
            sampler_index,
            node_index,
            AnimationPath::Scale,
        );
        
        (channel_index, sampler_index)
    }
}
