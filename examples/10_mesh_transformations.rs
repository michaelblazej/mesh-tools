use mesh_tools::{
    Mesh, Vertex,
    primitives::{create_cube, create_sphere, create_torus, TorusParams},
    modifiers::{transform_mesh, scale_mesh, rotate_mesh, translate_mesh, generate_smooth_normals},
    export::ExportMesh,
};
use std::fs::create_dir_all;
use glam::{Vec3, Mat4, Quat, Vec2};
use std::f32::consts::PI;

// Helper function to combine meshes since Mesh doesn't have a merge method
fn combine_meshes(meshes: &[&Mesh]) -> Mesh {
    let mut result = Mesh::new();
    
    for mesh in meshes {
        let base_vertex_index = result.vertices.len();
        
        // Add all vertices from this mesh
        for vertex in &mesh.vertices {
            result.add_vertex(vertex.clone());
        }
        
        // Add all triangles with adjusted indices
        for triangle in &mesh.triangles {
            let a = triangle.indices[0] + base_vertex_index;
            let b = triangle.indices[1] + base_vertex_index;
            let c = triangle.indices[2] + base_vertex_index;
            result.add_triangle(a, b, c).unwrap();
        }
    }
    
    result
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create output directory if it doesn't exist
    create_dir_all("output")?;
    
    // Example 1: Create a spiral staircase using transformations
    let step_count = 20;
    let step = create_cube(0.8, 0.1, 0.3);
    let mut steps = Vec::new();
    
    for i in 0..step_count {
        let mut step_instance = step.clone();
        
        // Calculate rotation and position along spiral
        let angle = i as f32 * (2.0 * PI / 8.0); // 8 steps make a full circle
        let height = i as f32 * 0.2;
        
        // Rotate step around Y axis
        rotate_mesh(&mut step_instance, Quat::from_rotation_y(angle));
        
        // Position step in spiral pattern
        let radius = 1.0;
        let x = radius * angle.cos();
        let z = radius * angle.sin();
        translate_mesh(&mut step_instance, Vec3::new(x, height, z));
        
        // Add to collection for later combination
        steps.push(step_instance);
    }
    
    // Combine all steps into one mesh
    let spiral = combine_meshes(&steps.iter().collect::<Vec<&Mesh>>());
    
    println!("Spiral staircase: {} vertices, {} triangles", 
             spiral.vertices.len(), spiral.triangles.len());
    spiral.export_glb("output/transform_spiral_staircase.glb")?;
    
    // Example 2: Creating a tower of differently scaled cubes
    let cube = create_cube(1.0, 1.0, 1.0);
    let levels = 8;
    let mut cubes = Vec::new();
    
    for i in 0..levels {
        let mut level_cube = cube.clone();
        
        // Scale the cube - gets smaller as it goes up
        let scale_factor = 1.0 - (i as f32 * 0.1);
        scale_mesh(&mut level_cube, Vec3::splat(scale_factor));
        
        // Position each level
        translate_mesh(&mut level_cube, Vec3::new(0.0, i as f32 * 1.1, 0.0));
        
        // Rotate each level slightly
        let rotation_angle = i as f32 * (PI / 8.0);
        rotate_mesh(&mut level_cube, Quat::from_rotation_y(rotation_angle));
        
        // Add to collection
        cubes.push(level_cube);
    }
    
    // Combine all cubes into tower
    let tower = combine_meshes(&cubes.iter().collect::<Vec<&Mesh>>());
    
    println!("Tower of cubes: {} vertices, {} triangles", 
             tower.vertices.len(), tower.triangles.len());
    tower.export_glb("output/transform_tower.glb")?;
    
    // Example 3: Create a complex transformation - a twisting torus
    let mut twisted_torus = create_torus(TorusParams {
        radius: 1.0,
        tube_radius: 0.2,
        radial_segments: 32,
        tubular_segments: 32,
    });
    
    // Apply a twisting transformation
    for vertex in &mut twisted_torus.vertices {
        // Calculate twist angle based on height
        let twist_factor = vertex.position.y * 2.0 * PI;
        
        // Create rotation matrix for the twist
        let rotation = Mat4::from_rotation_y(twist_factor);
        
        // Apply the twist transformation to x and z while preserving y
        let original_y = vertex.position.y;
        vertex.position = rotation.transform_point3(vertex.position);
        vertex.position.y = original_y; // Preserve the original height
    }
    
    // Recalculate normals after the transformation
    generate_smooth_normals(&mut twisted_torus);
    
    println!("Twisted torus: {} vertices, {} triangles", 
             twisted_torus.vertices.len(), twisted_torus.triangles.len());
    twisted_torus.export_glb("output/transform_twisted_torus.glb")?;
    
    // Example 4: Create a wave pattern using sine transformations
    let sphere = create_sphere(1.0, 32, 16);
    let mut wave_sphere = sphere.clone();
    
    // Apply wave pattern to vertices
    for vertex in &mut wave_sphere.vertices {
        // Create a wave pattern based on position
        let frequency = 5.0;
        let amplitude = 0.2;
        let wave = amplitude * (frequency * vertex.position.x).sin() * (frequency * vertex.position.z).cos();
        
        // Scale the position outward based on the wave
        vertex.position *= 1.0 + wave;
    }
    
    // Recalculate normals
    generate_smooth_normals(&mut wave_sphere);
    
    println!("Wave sphere: {} vertices, {} triangles", 
             wave_sphere.vertices.len(), wave_sphere.triangles.len());
    wave_sphere.export_glb("output/transform_wave_sphere.glb")?;
    
    // Example 5: Create a scene with multiple transformed objects
    let mut scene_objects = Vec::new();
    
    // Create ground plane
    let mut ground = create_cube(5.0, 0.1, 5.0);
    translate_mesh(&mut ground, Vec3::new(0.0, -0.5, 0.0));
    scene_objects.push(ground);
    
    // Add several objects with various transformations
    for i in 0..5 {
        // Create and transform a sphere
        let mut obj_sphere = create_sphere(0.3, 16, 8);
        translate_mesh(&mut obj_sphere, Vec3::new(
            (i as f32 - 2.0) * 0.8, 
            0.3, 
            -1.5
        ));
        scene_objects.push(obj_sphere);
        
        // Create and transform a cube
        let mut obj_cube = create_cube(0.4, 0.4, 0.4);
        translate_mesh(&mut obj_cube, Vec3::new(
            (i as f32 - 2.0) * 0.8, 
            0.2, 
            -0.5
        ));
        rotate_mesh(&mut obj_cube, Quat::from_rotation_y(PI / 4.0 * i as f32));
        scene_objects.push(obj_cube);
        
        // Create and transform a torus
        let mut obj_torus = create_torus(TorusParams {
            radius: 0.2,
            tube_radius: 0.05,
            radial_segments: 16,
            tubular_segments: 12,
        });
        translate_mesh(&mut obj_torus, Vec3::new(
            (i as f32 - 2.0) * 0.8, 
            0.2, 
            0.5
        ));
        rotate_mesh(&mut obj_torus, Quat::from_rotation_x(PI / 2.0));
        scene_objects.push(obj_torus);
    }
    
    // Combine all objects into scene
    let scene = combine_meshes(&scene_objects.iter().collect::<Vec<&Mesh>>());
    
    println!("Complex scene: {} vertices, {} triangles", 
             scene.vertices.len(), scene.triangles.len());
    scene.export_glb("output/transform_complex_scene.glb")?;
    
    println!("All transformation examples exported to the 'output' directory");
    Ok(())
}
