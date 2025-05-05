use glam::{Mat4, Vec3, Vec4};
use mesh_tools::{
    gltf::{GlbExportOptions, Material, PbrMetallicRoughness},
    primitives::{create_box, create_cylinder, BoxParameters, CylinderParameters},
    ExportMesh, Mesh,
};
use std::path::Path;

fn main() -> std::io::Result<()> {
    // Create a blue box
    let box_params = BoxParameters {
        width: 1.0,
        height: 1.0,
        depth: 1.0,
        width_segments: 2,
        height_segments: 2,
        depth_segments: 2,
        ..BoxParameters::default()
    };
    
    let mut blue_box = create_box(&box_params);
    
    // Add blue color to all vertices
    let blue_color = Vec4::new(0.0, 0.0, 1.0, 1.0); // RGBA blue
    for vertex in &mut blue_box.vertices {
        vertex.color = Some(blue_color);
    }
    
    // Position the box at (-1.5, 0, 0)
    let box_transform = Mat4::from_translation(Vec3::new(-1.5, 0.0, 0.0));
    blue_box.transform(&box_transform);
    
    // Create a red cylinder
    let cylinder_params = CylinderParameters {
        radius_top: 0.5,
        radius_bottom: 0.5,
        height: 2.0,
        radial_segments: 32,
        height_segments: 1,
        open_ended: false,
        ..CylinderParameters::default()
    };
    
    let mut red_cylinder = create_cylinder(&cylinder_params);
    
    // Add red color to all vertices
    let red_color = Vec4::new(1.0, 0.0, 0.0, 1.0); // RGBA red
    for vertex in &mut red_cylinder.vertices {
        vertex.color = Some(red_color);
    }
    
    // Position the cylinder at (1.5, 0, 0)
    let cylinder_transform = Mat4::from_translation(Vec3::new(1.5, 0.0, 0.0));
    red_cylinder.transform(&cylinder_transform);
    
    // Combine the meshes into a single scene
    let mut scene = Mesh::with_name("blue_box_and_red_cylinder");
    
    // Track original indices before merging
    let box_vertex_count = blue_box.vertices.len() as u32;
    
    // Add box vertices
    scene.vertices.extend(blue_box.vertices);
    
    // Add cylinder vertices
    scene.vertices.extend(red_cylinder.vertices);
    
    // Add box triangles
    for triangle in blue_box.triangles {
        scene.triangles.push(triangle);
    }
    
    // Add cylinder triangles (offset indices by box vertex count)
    for triangle in red_cylinder.triangles {
        scene.triangles.push(mesh_tools::Triangle::new(
            triangle.0 + box_vertex_count,
            triangle.1 + box_vertex_count,
            triangle.2 + box_vertex_count,
        ));
    }
    
    println!("Created scene with {} vertices and {} triangles", 
             scene.vertex_count(), scene.triangle_count());
    
    // Create export options that include vertex colors
    let export_options = GlbExportOptions {
        include_normals: true,
        include_uvs: true,
        include_colors: true, // Important to include colors
        include_tangents: false,
        material: Some(create_default_material()),
    };
    
    // Export the scene to a GLB file
    let output_path = Path::new("colored_shapes.glb");
    scene.export_glb_with_options(output_path, &export_options)?;
    
    println!("Exported scene to {}", output_path.display());
    
    Ok(())
}

// Create a default material that allows vertex colors to show through
fn create_default_material() -> Material {
    let pbr = PbrMetallicRoughness {
        baseColorFactor: Some([1.0, 1.0, 1.0, 1.0]), // white base color
        metallicFactor: Some(0.0),                   // non-metallic
        roughnessFactor: Some(0.5),                  // medium roughness
        ..PbrMetallicRoughness::default()
    };
    
    Material {
        name: Some("default_material".to_string()),
        pbrMetallicRoughness: Some(pbr),
        doubleSided: Some(true),
        ..Material::default()
    }
}
