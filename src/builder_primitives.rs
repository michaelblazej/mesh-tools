use crate::builder::GltfBuilder;
use crate::constants::{accessor_type, buffer_view_target, component_type};
use crate::models::Primitive;
use crate::primitives;
use std::collections::HashMap;

impl GltfBuilder {
    /// Create a simple box mesh
    pub fn create_box(&mut self, size: f32) -> usize {
        // Box centered at origin with given size
        let half_size = size / 2.0;
        
        // 8 vertices for a cube (8 corners)
        let positions = vec![
            // Front face (z+)
            -half_size, -half_size,  half_size,  // 0: bottom-left-front
             half_size, -half_size,  half_size,  // 1: bottom-right-front
             half_size,  half_size,  half_size,  // 2: top-right-front
            -half_size,  half_size,  half_size,  // 3: top-left-front
            
            // Back face (z-)
            -half_size, -half_size, -half_size,  // 4: bottom-left-back
             half_size, -half_size, -half_size,  // 5: bottom-right-back
             half_size,  half_size, -half_size,  // 6: top-right-back
            -half_size,  half_size, -half_size,  // 7: top-left-back
        ];
        
        // 12 triangles (2 per face * 6 faces)
        let indices = vec![
            // Front face (z+)
            0, 1, 2,  0, 2, 3,
            
            // Back face (z-)
            5, 4, 7,  5, 7, 6,
            
            // Left face (x-)
            4, 0, 3,  4, 3, 7,
            
            // Right face (x+)
            1, 5, 6,  1, 6, 2,
            
            // Bottom face (y-)
            4, 5, 1,  4, 1, 0,
            
            // Top face (y+)
            3, 2, 6,  3, 6, 7,
        ];
        
        // Normals for each vertex
        let normals = vec![
            // Front face (z+)
            0.0, 0.0, 1.0,
            0.0, 0.0, 1.0,
            0.0, 0.0, 1.0,
            0.0, 0.0, 1.0,
            
            // Back face (z-)
            0.0, 0.0, -1.0,
            0.0, 0.0, -1.0,
            0.0, 0.0, -1.0,
            0.0, 0.0, -1.0,
            
            // This is incorrect for a cube with shared vertices
            // Real normal mapping would use a separate vertex for each face
            // but this is simplified for example purposes
        ];
        
        // Simple UV mapping
        let uvs = vec![
            // Front face
            0.0, 1.0,
            1.0, 1.0,
            1.0, 0.0,
            0.0, 0.0,
            
            // Back face
            1.0, 1.0,
            0.0, 1.0,
            0.0, 0.0,
            1.0, 0.0,
        ];
        
        self.create_simple_mesh(None, &positions, &indices, Some(&normals), Some(&uvs), None)
    }

    /// Create a box with the specified material
    pub fn create_box_with_material(&mut self, size: f32, material: Option<usize>) -> usize {
        // Box centered at origin with given size
        let half_size = size / 2.0;
        
        // For a proper cube with separate normals per face, we need to duplicate vertices
        // 24 vertices for a cube (4 per face * 6 faces)
        let positions = vec![
            // Front face (z+)
            -half_size, -half_size,  half_size,  // 0: bottom-left
             half_size, -half_size,  half_size,  // 1: bottom-right
             half_size,  half_size,  half_size,  // 2: top-right
            -half_size,  half_size,  half_size,  // 3: top-left
            
            // Back face (z-)
            -half_size, -half_size, -half_size,  // 4: bottom-left
            -half_size,  half_size, -half_size,  // 5: bottom-right
             half_size,  half_size, -half_size,  // 6: top-right
            half_size,  -half_size, -half_size,  // 7: top-left
            
            // Top face (y+)
            -half_size,  half_size, -half_size,  // 8: back-left
            -half_size,  half_size,  half_size,  // 9: back-right
             half_size,  half_size,  half_size,  // 10: front-right
             half_size,  half_size, -half_size,  // 11: front-left
            
            // Bottom face (y-)
            -half_size, -half_size, -half_size,  // 12: back-left
             half_size, -half_size, -half_size,  // 13: back-right
             half_size, -half_size,  half_size,  // 14: front-right
            -half_size, -half_size,  half_size,  // 15: front-left
            
            // Right face (x+)
             half_size, -half_size, -half_size,  // 16: bottom-back
             half_size,  half_size, -half_size,  // 17: top-back
             half_size,  half_size,  half_size,  // 18: top-front
             half_size, -half_size,  half_size,  // 19: bottom-front
            
            // Left face (x-)
            -half_size, -half_size, -half_size,  // 20: bottom-back
            -half_size, -half_size,  half_size,  // 21: top-back
            -half_size,  half_size,  half_size,  // 22: top-front
            -half_size,  half_size, -half_size,  // 23: bottom-front
        ];
        
        // 12 triangles (2 per face * 6 faces), now with correct indexing
        let indices = vec![
            // Front face
            0, 1, 2,  0, 2, 3,
            
            // Back face
            4, 5, 6,  4, 6, 7,
            
            // Top face
            8, 9, 10,  8, 10, 11,
            
            // Bottom face
            12, 13, 14,  12, 14, 15,
            
            // Right face
            16, 17, 18,  16, 18, 19,
            
            // Left face
            20, 21, 22,  20, 22, 23,
        ];
        
        // Normals for each vertex
        let normals = vec![
            // Front face (z+)
            0.0, 0.0, 1.0,
            0.0, 0.0, 1.0,
            0.0, 0.0, 1.0,
            0.0, 0.0, 1.0,
            
            // Back face (z-)
            0.0, 0.0, -1.0,
            0.0, 0.0, -1.0,
            0.0, 0.0, -1.0,
            0.0, 0.0, -1.0,
            
            // Top face (y+)
            0.0, 1.0, 0.0,
            0.0, 1.0, 0.0,
            0.0, 1.0, 0.0,
            0.0, 1.0, 0.0,
            
            // Bottom face (y-)
            0.0, -1.0, 0.0,
            0.0, -1.0, 0.0,
            0.0, -1.0, 0.0,
            0.0, -1.0, 0.0,
            
            // Right face (x+)
            1.0, 0.0, 0.0,
            1.0, 0.0, 0.0,
            1.0, 0.0, 0.0,
            1.0, 0.0, 0.0,
            
            // Left face (x-)
            -1.0, 0.0, 0.0,
            -1.0, 0.0, 0.0,
            -1.0, 0.0, 0.0,
            -1.0, 0.0, 0.0,
        ];
        
        // UVs for each face
        let uvs = vec![
            // Front face
            0.0, 1.0,
            1.0, 1.0,
            1.0, 0.0,
            0.0, 0.0,
            
            // Back face
            1.0, 1.0,
            1.0, 0.0,
            0.0, 0.0,
            0.0, 1.0,
            
            // Top face
            0.0, 1.0,
            0.0, 0.0,
            1.0, 0.0,
            1.0, 1.0,
            
            // Bottom face
            1.0, 1.0,
            0.0, 1.0,
            0.0, 0.0,
            1.0, 0.0,
            
            // Right face
            1.0, 1.0,
            1.0, 0.0,
            0.0, 0.0,
            0.0, 1.0,
            
            // Left face
            0.0, 1.0,
            1.0, 1.0,
            1.0, 0.0,
            0.0, 0.0,
        ];
        
        self.create_simple_mesh(None, &positions, &indices, Some(&normals), Some(&uvs), material)
    }
    
    /// Create a mesh with custom geometry and UV mapping
    /// 
    /// # Parameters
    /// * `name` - Optional name for the mesh
    /// * `positions` - Vertex positions as [x1, y1, z1, x2, y2, z2, ...]
    /// * `indices` - Vertex indices for triangles
    /// * `normals` - Optional vertex normals as [x1, y1, z1, x2, y2, z2, ...]
    /// * `texcoords` - Optional array of UV coordinate sets, each as [u1, v1, u2, v2, ...]. 
    ///                 The first set becomes TEXCOORD_0, the second TEXCOORD_1, etc.
    /// * `material` - Optional material index to use for the mesh
    /// 
    /// # Returns
    /// The index of the created mesh
    pub fn create_custom_mesh(&mut self, 
                            name: Option<String>,
                            positions: &[f32], 
                            indices: &[u16], 
                            normals: Option<&[f32]>, 
                            texcoords: Option<Vec<Vec<f32>>>,
                            material: Option<usize>) -> usize {
        // Calculate bounds for the positions
        let (min, max) = if !positions.is_empty() {
            let mut min = vec![f32::MAX; 3];
            let mut max = vec![f32::MIN; 3];
            
            for i in (0..positions.len()).step_by(3) {
                min[0] = min[0].min(positions[i]);
                min[1] = min[1].min(positions[i + 1]);
                min[2] = min[2].min(positions[i + 2]);
                
                max[0] = max[0].max(positions[i]);
                max[1] = max[1].max(positions[i + 1]);
                max[2] = max[2].max(positions[i + 2]);
            }
            
            (Some(min), Some(max))
        } else {
            (None, None)
        };
        
        // Add position data to buffer
        let pos_bytes = unsafe {
            std::slice::from_raw_parts(
                positions.as_ptr() as *const u8,
                positions.len() * std::mem::size_of::<f32>()
            )
        };
        let (pos_offset, pos_length) = self.add_buffer_data(pos_bytes);
        let pos_buffer_view = self.add_buffer_view(pos_offset, pos_length, Some(buffer_view_target::ARRAY_BUFFER));
        
        // Add position accessor
        let vertex_count = positions.len() / 3;
        let pos_accessor = self.add_accessor(
            pos_buffer_view,
            component_type::FLOAT,
            vertex_count,
            accessor_type::VEC3.to_string(),
            None,
            min,
            max
        );
        
        // Add index data to buffer
        let idx_bytes = unsafe {
            std::slice::from_raw_parts(
                indices.as_ptr() as *const u8,
                indices.len() * std::mem::size_of::<u16>()
            )
        };
        let (idx_offset, idx_length) = self.add_buffer_data(idx_bytes);
        let idx_buffer_view = self.add_buffer_view(idx_offset, idx_length, Some(buffer_view_target::ELEMENT_ARRAY_BUFFER));
        
        // Add index accessor
        let idx_accessor = self.add_accessor(
            idx_buffer_view,
            component_type::UNSIGNED_SHORT,
            indices.len(),
            accessor_type::SCALAR.to_string(),
            None,
            None,
            None
        );
        
        // Build attributes map
        let mut attributes = HashMap::new();
        attributes.insert("POSITION".to_string(), pos_accessor);
        
        // Add normals if provided
        let mut normal_accessor = None;
        if let Some(normal_data) = normals {
            let norm_bytes = unsafe {
                std::slice::from_raw_parts(
                    normal_data.as_ptr() as *const u8,
                    normal_data.len() * std::mem::size_of::<f32>()
                )
            };
            let (norm_offset, norm_length) = self.add_buffer_data(norm_bytes);
            let norm_buffer_view = self.add_buffer_view(norm_offset, norm_length, Some(buffer_view_target::ARRAY_BUFFER));
            
            normal_accessor = Some(self.add_accessor(
                norm_buffer_view,
                component_type::FLOAT,
                normal_data.len() / 3,
                accessor_type::VEC3.to_string(),
                None,
                None,
                None
            ));
            
            attributes.insert("NORMAL".to_string(), normal_accessor.unwrap());
        }
        
        // Add texture coordinates if provided
        let mut texcoord_accessors = Vec::new();
        if let Some(texcoord_sets) = texcoords {
            for (i, texcoord_data) in texcoord_sets.iter().enumerate() {
                let tc_bytes = unsafe {
                    std::slice::from_raw_parts(
                        texcoord_data.as_ptr() as *const u8,
                        texcoord_data.len() * std::mem::size_of::<f32>()
                    )
                };
                let (tc_offset, tc_length) = self.add_buffer_data(tc_bytes);
                let tc_buffer_view = self.add_buffer_view(tc_offset, tc_length, Some(buffer_view_target::ARRAY_BUFFER));
                
                let tc_accessor = self.add_accessor(
                    tc_buffer_view,
                    component_type::FLOAT,
                    texcoord_data.len() / 2, // 2 floats per UV
                    accessor_type::VEC2.to_string(),
                    None,
                    None,
                    None
                );
                
                attributes.insert(format!("TEXCOORD_{}", i), tc_accessor);
                texcoord_accessors.push(tc_accessor);
            }
        }
        
        // Create primitive
        let primitive = Primitive {
            attributes,
            indices: Some(idx_accessor),
            material,
            mode: None, // Default mode (triangles)
        };
        
        // Create and add mesh
        self.add_mesh(name, vec![primitive])
    }
    
    /// Create a mesh with custom geometry and single UV channel
    /// 
    /// Simplified version of create_custom_mesh for the common case of a single UV channel
    /// 
    /// # Parameters
    /// * `name` - Optional name for the mesh
    /// * `positions` - Vertex positions as [x1, y1, z1, x2, y2, z2, ...]
    /// * `indices` - Vertex indices for triangles
    /// * `normals` - Optional vertex normals as [x1, y1, z1, x2, y2, z2, ...]
    /// * `texcoords` - Optional UV coordinates as [u1, v1, u2, v2, ...]
    /// * `material` - Optional material index to use for the mesh
    /// 
    /// # Returns
    /// The index of the created mesh
    pub fn create_simple_mesh(&mut self, 
                            name: Option<String>,
                            positions: &[f32], 
                            indices: &[u16], 
                            normals: Option<&[f32]>, 
                            texcoords: Option<&[f32]>,
                            material: Option<usize>) -> usize {
        let texcoord_sets = if let Some(texcoords) = texcoords {
            let mut sets = Vec::new();
            sets.push(texcoords.to_vec());
            Some(sets)
        } else {
            None
        };
        
        self.create_custom_mesh(name, positions, indices, normals, texcoord_sets, material)
    }
    
    /// Create a plane mesh
    /// 
    /// # Parameters
    /// * `width` - Width of the plane along X axis
    /// * `depth` - Depth of the plane along Z axis 
    /// * `width_segments` - Number of subdivisions along width
    /// * `depth_segments` - Number of subdivisions along depth
    /// * `material` - Optional material index to use for the mesh
    /// 
    /// # Returns
    /// The index of the created mesh
    pub fn create_plane(&mut self, 
                      width: f32, 
                      depth: f32, 
                      width_segments: usize, 
                      depth_segments: usize,
                      material: Option<usize>) -> usize {
        let (positions, indices, normals, uvs) = primitives::generate_plane(
            width, depth, width_segments, depth_segments
        );
        
        self.create_simple_mesh(None, &positions, &indices, Some(&normals), Some(&uvs), material)
    }
    
    /// Create a sphere mesh
    /// 
    /// # Parameters
    /// * `radius` - Radius of the sphere
    /// * `width_segments` - Number of horizontal subdivisions
    /// * `height_segments` - Number of vertical subdivisions
    /// * `material` - Optional material index to use for the mesh
    /// 
    /// # Returns
    /// The index of the created mesh
    pub fn create_sphere(&mut self, 
                       radius: f32, 
                       width_segments: usize, 
                       height_segments: usize,
                       material: Option<usize>) -> usize {
        let (positions, indices, normals, uvs) = primitives::generate_sphere(
            radius, width_segments, height_segments
        );
        
        self.create_simple_mesh(None, &positions, &indices, Some(&normals), Some(&uvs), material)
    }
    
    /// Create a cylinder mesh
    /// 
    /// # Parameters
    /// * `radius_top` - Radius at the top of the cylinder
    /// * `radius_bottom` - Radius at the bottom of the cylinder
    /// * `height` - Height of the cylinder
    /// * `radial_segments` - Number of subdivisions around the circumference
    /// * `height_segments` - Number of subdivisions along the height
    /// * `open_ended` - Whether to include top and bottom caps
    /// * `material` - Optional material index to use for the mesh
    /// 
    /// # Returns
    /// The index of the created mesh
    pub fn create_cylinder(&mut self, 
                         radius_top: f32, 
                         radius_bottom: f32, 
                         height: f32, 
                         radial_segments: usize, 
                         height_segments: usize,
                         open_ended: bool,
                         material: Option<usize>) -> usize {
        let (positions, indices, normals, uvs) = primitives::generate_cylinder(
            radius_top, radius_bottom, height, radial_segments, height_segments, open_ended
        );
        
        self.create_simple_mesh(None, &positions, &indices, Some(&normals), Some(&uvs), material)
    }
    
    /// Create a cone mesh
    /// 
    /// # Parameters
    /// * `radius` - Radius at the base of the cone
    /// * `height` - Height of the cone
    /// * `radial_segments` - Number of subdivisions around the circumference
    /// * `height_segments` - Number of subdivisions along the height
    /// * `open_ended` - Whether to include the base cap
    /// * `material` - Optional material index to use for the mesh
    /// 
    /// # Returns
    /// The index of the created mesh
    pub fn create_cone(&mut self, 
                      radius: f32, 
                      height: f32, 
                      radial_segments: usize, 
                      height_segments: usize,
                      open_ended: bool,
                      material: Option<usize>) -> usize {
        let (positions, indices, normals, uvs) = primitives::generate_cone(
            radius, height, radial_segments, height_segments, open_ended
        );
        
        self.create_simple_mesh(None, &positions, &indices, Some(&normals), Some(&uvs), material)
    }
    
    /// Create a torus (donut shape) mesh
    /// 
    /// # Parameters
    /// * `radius` - Distance from the center of the tube to the center of the torus
    /// * `tube` - Radius of the tube
    /// * `radial_segments` - Number of subdivisions around the main circle
    /// * `tubular_segments` - Number of subdivisions around the tube
    /// * `material` - Optional material index to use for the mesh
    /// 
    /// # Returns
    /// The index of the created mesh
    pub fn create_torus(&mut self, 
                       radius: f32, 
                       tube: f32, 
                       radial_segments: usize, 
                       tubular_segments: usize,
                       material: Option<usize>) -> usize {
        let (positions, indices, normals, uvs) = primitives::generate_torus(
            radius, tube, radial_segments, tubular_segments
        );
        
        self.create_simple_mesh(None, &positions, &indices, Some(&normals), Some(&uvs), material)
    }
    
    /// Create an icosahedron (20-sided polyhedron) mesh
    /// 
    /// # Parameters
    /// * `radius` - Radius of the circumscribed sphere
    /// * `material` - Optional material index to use for the mesh
    /// 
    /// # Returns
    /// The index of the created mesh
    pub fn create_icosahedron(&mut self, 
                            radius: f32,
                            material: Option<usize>) -> usize {
        let (positions, indices, normals, uvs) = primitives::generate_icosahedron(radius);
        
        self.create_simple_mesh(None, &positions, &indices, Some(&normals), Some(&uvs), material)
    }
}
