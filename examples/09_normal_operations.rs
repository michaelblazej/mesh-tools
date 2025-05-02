use mesh_tools::{
    Mesh, Vertex,
    primitives::{create_cube, create_sphere, create_plane, PlaneParams},
    modifiers::{flip_normals, generate_smooth_normals, generate_flat_normals, scale_mesh},
    export::ExportMesh,
};
use std::fs::create_dir_all;
use glam::Vec3;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create output directory if it doesn't exist
    create_dir_all("output")?;
    
    // Example 1: Flipping normals
    let mut cube = create_cube(1.0, 1.0, 1.0);
    
    // Export original cube
    println!("Original cube: {} vertices, {} triangles", 
             cube.vertices.len(), cube.triangles.len());
    cube.export_glb("output/normal_cube_original.glb")?;
    
    // Flip normals and export
    let mut flipped_cube = cube.clone();
    flip_normals(&mut flipped_cube);
    
    println!("Cube with flipped normals created");
    flipped_cube.export_glb("output/normal_cube_flipped.glb")?;
    
    // Example 2: Generating smooth normals for a sphere with initially flat normals
    // Create a low-poly UV sphere with flat normals first
    let mut flat_sphere = create_sphere(1.0, 8, 6);
    
    // Ensure we start with flat normals
    generate_flat_normals(&mut flat_sphere);
    
    println!("Sphere with flat normals: {} vertices", flat_sphere.vertices.len());
    flat_sphere.export_glb("output/normal_sphere_flat.glb")?;
    
    // Generate smooth normals
    let mut smooth_sphere = flat_sphere.clone();
    generate_smooth_normals(&mut smooth_sphere);
    
    println!("Sphere with smooth normals created");
    smooth_sphere.export_glb("output/normal_sphere_smooth.glb")?;
    
    // Example 3: Creating a faceted look by duplicating vertices
    let mut faceted_cube = create_cube(1.0, 1.0, 1.0);
    
    // Make cube slightly larger for better visibility
    scale_mesh(&mut faceted_cube, 1.2);
    
    // Convert to a faceted look by ensuring each triangle has its own vertices
    // This requires duplicating vertices so each face has unique normals
    let mut faceted_mesh = Mesh::new();
    
    for triangle in &faceted_cube.triangles {
        let indices = triangle.indices;
        
        // Get original vertices
        let v0 = faceted_cube.vertices[indices[0]].clone();
        let v1 = faceted_cube.vertices[indices[1]].clone();
        let v2 = faceted_cube.vertices[indices[2]].clone();
        
        // Calculate flat normal for this face
        let pos0 = v0.position.unwrap();
        let pos1 = v1.position.unwrap();
        let pos2 = v2.position.unwrap();
        
        let edge1 = pos1 - pos0;
        let edge2 = pos2 - pos0;
        let normal = edge1.cross(edge2).normalize();
        
        // Add new vertices with the flat normal
        let new_v0 = Vertex::with_all(pos0, normal, v0.uv.unwrap_or_default());
        let new_v1 = Vertex::with_all(pos1, normal, v1.uv.unwrap_or_default());
        let new_v2 = Vertex::with_all(pos2, normal, v2.uv.unwrap_or_default());
        
        // Add to mesh
        let i0 = faceted_mesh.add_vertex(new_v0);
        let i1 = faceted_mesh.add_vertex(new_v1);
        let i2 = faceted_mesh.add_vertex(new_v2);
        
        faceted_mesh.add_triangle(i0, i1, i2)?;
    }
    
    println!("Faceted cube created: {} vertices, {} triangles", 
             faceted_mesh.vertices.len(), faceted_mesh.triangles.len());
    faceted_mesh.export_glb("output/normal_cube_faceted.glb")?;
    
    // Example 4: Creating a normal mapped plane
    let plane_params = PlaneParams {
        width: 2.0,
        height: 2.0,
        width_segments: 20,
        height_segments: 20,
    };
    let mut bumpy_plane = create_plane(plane_params);
    
    // Displace the geometry but keep smooth normals for normal mapping visualization
    for vertex in &mut bumpy_plane.vertices {
        if let Some(position) = &mut vertex.position {
            // Create a bumpy surface
            let x = position.x * 5.0;
            let z = position.z * 5.0;
            
            // Apply a simple bump function
            position.y = 0.1 * (x.sin() * z.cos());
        }
    }
    
    // Generate smooth normals
    generate_smooth_normals(&mut bumpy_plane);
    
    println!("Bumpy plane with smooth normals: {} vertices", bumpy_plane.vertices.len());
    bumpy_plane.export_glb("output/normal_bumpy_plane.glb")?;
    
    println!("All normal operation examples exported to the 'output' directory");
    Ok(())
}
