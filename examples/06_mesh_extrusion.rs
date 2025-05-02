use mesh_tools::{
    primitives::create_plane,
    modifiers::{extrude_faces, translate_mesh},
    export::ExportMesh,
};
use std::fs::create_dir_all;
use glam::Vec3;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create output directory if it doesn't exist
    create_dir_all("output")?;
    
    // Example 1: Extrude faces from a plane to create a 3D grid
    let base_plane = create_plane(2.0, 2.0, 2, 2);
    
    // Extrude all faces together in the same direction
    let mut extruded_plane = base_plane.clone();
    // Get indices of all triangles
    let face_indices: Vec<usize> = (0..extruded_plane.triangles.len()).collect();
    extrude_faces(&mut extruded_plane, &face_indices, 0.5)?;
    
    println!("Extruded plane: {} vertices, {} triangles", 
             extruded_plane.vertices.len(), extruded_plane.triangles.len());
    extruded_plane.export_glb("output/extruded_plane.glb")?;
    
    // Example 2: Extrude different triangle sets in different directions
    let mut group_extrusions = base_plane.clone();
    
    // Get two groups of triangles (first half, second half)
    let triangles_count = group_extrusions.triangles.len();
    let first_half: Vec<usize> = (0..triangles_count/2).collect();
    let second_half: Vec<usize> = (triangles_count/2..triangles_count).collect();
    
    // Extrude first group up
    extrude_faces(&mut group_extrusions, &first_half, 0.5)?;
    
    // Extrude second group down (we need to reindex after the first extrusion)
    // The newly created faces will have indices starting from the original triangle count
    let new_second_half: Vec<usize> = (triangles_count..triangles_count + second_half.len()).collect();
    extrude_faces(&mut group_extrusions, &new_second_half, 0.3)?;
    
    println!("Group extruded plane: {} vertices, {} triangles", 
             group_extrusions.vertices.len(), group_extrusions.triangles.len());
    group_extrusions.export_glb("output/group_extrusions.glb")?;
    
    // Example 3: Create a stepped pyramid by successive extrusions
    let mut pyramid = create_plane(2.0, 2.0, 1, 1);
    translate_mesh(&mut pyramid, Vec3::new(0.0, -1.0, 0.0)); // Move down for better visibility
    
    let steps = 5;
    
    for i in 0..steps {
        // Get the top face indices (after each step, the top faces will be the new ones)
        let face_count = pyramid.triangles.len();
        let top_faces: Vec<usize> = if i == 0 {
            // First iteration uses original faces
            (0..face_count).collect()
        } else {
            // After first iteration, use only the most recently created faces (the top)
            let new_face_start = face_count - 2; // Each step adds 2 triangles
            (new_face_start..face_count).collect()
        };
        
        // Extrude upward
        extrude_faces(&mut pyramid, &top_faces, 0.2)?;
        
        // Scale just the extruded part to make a stepped pyramid
        if i < steps - 1 {
            // Get the newly created faces (top of the extrusion)
            let new_face_count = pyramid.triangles.len();
            for j in (new_face_count - 2)..new_face_count {
                if j < pyramid.triangles.len() {
                    let tri = &pyramid.triangles[j];
                    // Move vertices inward slightly
                    for &idx in &tri.indices {
                        // Get the vertex position directly (not an Option)
                        let pos = &mut pyramid.vertices[idx].position;
                        // Calculate center point at same height as current vertex
                        let center = Vec3::new(0.0, pos.y, 0.0);
                        // Calculate direction from center to vertex
                        let dir = (*pos - center).normalize();
                        // Scale the vertex position toward center
                        *pos = center + dir * (*pos - center).length() * 0.8;
                    }
                }
            }
        }
    }
    
    println!("Stepped pyramid: {} vertices, {} triangles", 
             pyramid.vertices.len(), pyramid.triangles.len());
    pyramid.export_glb("output/stepped_pyramid.glb")?;
    
    println!("All extrusion examples exported to the 'output' directory");
    Ok(())
}
