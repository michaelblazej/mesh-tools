use mesh_tools::{
    primitives::{create_cube, create_sphere, create_plane},
    modifiers::{flip_normals, generate_smooth_normals, scale_mesh},
    export::ExportMesh,
    Mesh, Vertex,
};
use std::fs::create_dir_all;
use glam::{Vec3, Vec2};

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
    // Create a low-poly UV sphere
    let mut flat_sphere = create_sphere(1.0, 8, 6);
    
    // Create copy of the sphere with flat (faceted) normals by duplicating vertices
    let mut faceted_sphere = Mesh::new();
    
    // Duplicate vertices for each triangle to create flat normals
    for triangle in &flat_sphere.triangles {
        let indices = triangle.indices;
        
        // Get original vertices
        let v0 = flat_sphere.vertices[indices[0]].clone();
        let v1 = flat_sphere.vertices[indices[1]].clone();
        let v2 = flat_sphere.vertices[indices[2]].clone();
        
        // Calculate flat normal for this face
        let pos0 = v0.position;
        let pos1 = v1.position;
        let pos2 = v2.position;
        
        let edge1 = pos1 - pos0;
        let edge2 = pos2 - pos0;
        let normal = edge1.cross(edge2).normalize();
        
        // Add new vertices with the flat normal
        let new_v0 = Vertex::with_all(pos0, normal, v0.uv.unwrap_or_default());
        let new_v1 = Vertex::with_all(pos1, normal, v1.uv.unwrap_or_default());
        let new_v2 = Vertex::with_all(pos2, normal, v2.uv.unwrap_or_default());
        
        // Add to mesh
        let i0 = faceted_sphere.add_vertex(new_v0);
        let i1 = faceted_sphere.add_vertex(new_v1);
        let i2 = faceted_sphere.add_vertex(new_v2);
        
        faceted_sphere.add_triangle(i0, i1, i2)?;
    }
    
    println!("Sphere with flat normals: {} vertices", faceted_sphere.vertices.len());
    faceted_sphere.export_glb("output/normal_sphere_flat.glb")?;
    
    // Generate smooth normals on the original sphere
    let mut smooth_sphere = flat_sphere.clone();
    generate_smooth_normals(&mut smooth_sphere);
    
    println!("Sphere with smooth normals created");
    smooth_sphere.export_glb("output/normal_sphere_smooth.glb")?;
    
    // Example 3: Creating a faceted look by duplicating vertices
    let mut faceted_cube = create_cube(1.0, 1.0, 1.0);
    
    // Make cube slightly larger for better visibility
    scale_mesh(&mut faceted_cube, Vec3::new(1.2, 1.2, 1.2));
    
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
        let pos0 = v0.position;
        let pos1 = v1.position;
        let pos2 = v2.position;
        
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
    let plane = create_plane(2.0, 2.0, 20, 20);
    let mut bumpy_plane = plane.clone();
    
    // Displace the geometry but keep smooth normals for normal mapping visualization
    for vertex in &mut bumpy_plane.vertices {
        // Create a bumpy surface
        let x = vertex.position.x * 5.0;
        let z = vertex.position.z * 5.0;
        
        // Apply a simple bump function
        vertex.position.y = 0.1 * (x.sin() * z.cos());
    }
    
    // Generate smooth normals
    generate_smooth_normals(&mut bumpy_plane);
    
    println!("Bumpy plane with smooth normals: {} vertices", bumpy_plane.vertices.len());
    bumpy_plane.export_glb("output/normal_bumpy_plane.glb")?;
    
    println!("All normal operation examples exported to the 'output' directory");
    Ok(())
}
