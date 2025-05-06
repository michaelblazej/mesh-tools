use glam::{Mat4, Vec3, Vec4};
use mesh_tools::{
    primitives::{BoxParameters, CylinderParameters, create_box, create_cylinder},
    gltf::{export_to_glb_with_options, GlbExportOptions},
    Triangle, Mesh,
};
use std::f32::consts::PI;

fn main() {
    // Create a box/cube
    let mut box_mesh = create_box(&BoxParameters {
        width: 1.0,
        height: 1.0,
        depth: 1.0,
        width_segments: 1,
        height_segments: 1,
        depth_segments: 1,
    });
    
    // Add blue color to all vertices in the box
    for vertex in &mut box_mesh.vertices {
        vertex.color = Some(Vec4::new(0.0, 0.0, 1.0, 1.0)); // Blue
    }
    
    // Create a cylinder
    let mut cylinder_mesh = create_cylinder(&CylinderParameters {
        radius_top: 0.5,
        radius_bottom: 0.5,
        height: 2.0,
        radial_segments: 16,
        height_segments: 1,
        theta_length: 2.0 * PI,
        open_ended: false,
    });
    
    // Add red color to all vertices in the cylinder
    for vertex in &mut cylinder_mesh.vertices {
        vertex.color = Some(Vec4::new(1.0, 0.0, 0.0, 1.0)); // Red
    }
    
    // Translate the cylinder up
    let translation = Mat4::from_translation(Vec3::new(0.0, 1.5, 0.0));
    cylinder_mesh.transform(&translation);
    
    // Combine meshes into a single scene
    let mut scene = box_mesh;
    let box_vertices = scene.vertices.len();
    
    // Add cylinder vertices to the scene
    scene.vertices.extend(cylinder_mesh.vertices);
    
    // Add cylinder triangles to the scene (with adjusted indices)
    for triangle in cylinder_mesh.triangles {
        scene.triangles.push(Triangle(
            triangle.0 + box_vertices as u32,
            triangle.1 + box_vertices as u32,
            triangle.2 + box_vertices as u32,
        ));
    }
    
    println!("Created scene with {} vertices and {} triangles", 
             scene.vertices.len(), scene.triangles.len());
    
    // Export the scene to a GLB file with colors
    let options = GlbExportOptions {
        include_normals: true,
        include_uvs: true,
        include_colors: true,
        include_tangents: false,
        material: None,
    };
    
    // Export to GLB
    export_to_glb_with_options(&scene, "colored_shapes.glb", &options).unwrap();
    println!("Exported scene to colored_shapes.glb");
}
