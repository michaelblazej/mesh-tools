use mesh_tools::{
    primitives::{
        create_cube, create_sphere, create_cylinder, create_torus, create_icosphere,
        CylinderParams, TorusParams, IcosphereParams,
    },
    export::ExportMesh,
};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create output directory if it doesn't exist
    std::fs::create_dir_all("output")?;

    // Example 1: Create a simple cube
    let cube = create_cube(2.0, 2.0, 2.0);
    println!("Cube created with {} vertices and {} triangles", 
             cube.vertices.len(), cube.triangles.len());
    cube.export_glb("output/cube.glb")?;
    
    // Example 2: Create a UV sphere
    let sphere = create_sphere(1.0, 32, 16);
    println!("UV Sphere created with {} vertices and {} triangles", 
             sphere.vertices.len(), sphere.triangles.len());
    sphere.export_glb("output/sphere.glb")?;
    
    // Example 3: Create a cylinder with custom parameters
    let cylinder_params = CylinderParams {
        radius: 1.0,
        height: 3.0,
        radial_segments: 24,
        height_segments: 1,
        top_cap: true,
        bottom_cap: true,
    };
    let cylinder = create_cylinder(cylinder_params);
    println!("Cylinder created with {} vertices and {} triangles", 
             cylinder.vertices.len(), cylinder.triangles.len());
    cylinder.export_glb("output/cylinder.glb")?;
    
    // Example 4: Create a torus (donut)
    let torus_params = TorusParams {
        radius: 1.0,
        tube_radius: 0.3,
        radial_segments: 32,
        tubular_segments: 24,
    };
    let torus = create_torus(torus_params);
    println!("Torus created with {} vertices and {} triangles", 
             torus.vertices.len(), torus.triangles.len());
    torus.export_glb("output/torus.glb")?;
    
    // Example 5: Create an icosphere (more evenly distributed vertices than UV sphere)
    let icosphere_params = IcosphereParams {
        radius: 1.0,
        subdivisions: 2,
    };
    let icosphere = create_icosphere(icosphere_params);
    println!("Icosphere created with {} vertices and {} triangles", 
             icosphere.vertices.len(), icosphere.triangles.len());
    icosphere.export_glb("output/icosphere.glb")?;
    
    println!("All primitive shapes created and exported to the 'output' directory");
    
    Ok(())
}
