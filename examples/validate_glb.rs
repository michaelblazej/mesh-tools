use std::fs;
use std::io::{Cursor, Read, Seek, SeekFrom};
use byteorder::{LittleEndian, ReadBytesExt};
use serde_json::{self, Value};

const GLB_MAGIC: u32 = 0x46546C67; // "glTF" in ASCII
const GLB_VERSION: u32 = 2;
const GLB_CHUNK_TYPE_JSON: u32 = 0x4E4F534A; // "JSON" in ASCII
const GLB_CHUNK_TYPE_BIN: u32 = 0x004E4942; // "BIN\0" in ASCII

fn main() {
    let file_path = "output/multi_material_scene.glb";
    println!("Validating GLB file against glTF 2.0 spec: {}", file_path);
    
    match validate_glb(file_path) {
        Ok(_) => println!("GLB file passed validation checks."),
        Err(e) => println!("ERROR: GLB validation failed: {}", e),
    }
}

fn validate_glb(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Read the entire GLB file
    let glb_data = fs::read(file_path)?;
    let mut reader = Cursor::new(&glb_data);
    
    // Check 1: Validate GLB header
    let magic = reader.read_u32::<LittleEndian>()?;
    if magic != GLB_MAGIC {
        return Err(format!("Invalid GLB magic number: 0x{:X}, expected: 0x{:X}", magic, GLB_MAGIC).into());
    }
    
    let version = reader.read_u32::<LittleEndian>()?;
    if version != GLB_VERSION {
        return Err(format!("Unsupported GLB version: {}, expected: {}", version, GLB_VERSION).into());
    }
    
    let total_length = reader.read_u32::<LittleEndian>()? as usize;
    if total_length != glb_data.len() {
        return Err(format!("GLB length mismatch: header says {} bytes, file is {} bytes", 
            total_length, glb_data.len()).into());
    }
    
    // Check 2: Validate JSON chunk
    let json_chunk_length = reader.read_u32::<LittleEndian>()? as usize;
    let json_chunk_type = reader.read_u32::<LittleEndian>()?;
    if json_chunk_type != GLB_CHUNK_TYPE_JSON {
        return Err(format!("First chunk is not JSON, type: 0x{:X}, expected: 0x{:X}", 
            json_chunk_type, GLB_CHUNK_TYPE_JSON).into());
    }
    
    // Check if JSON chunk is aligned to 4 bytes
    if json_chunk_length % 4 != 0 {
        return Err(format!("JSON chunk length {} is not aligned to 4 bytes", json_chunk_length).into());
    }
    
    // Read the JSON data
    let mut json_data = vec![0u8; json_chunk_length];
    reader.read_exact(&mut json_data)?;
    
    // Parse and validate the JSON against the glTF 2.0 schema
    let json_str = String::from_utf8(json_data)?;
    let json_value: Value = serde_json::from_str(&json_str)?;
    
    // Required fields check
    validate_required_json_fields(&json_value)?;
    
    // Check 3: Validate BIN chunk
    // First, ensure we're at the right position
    let expected_bin_chunk_position = 12 + 8 + json_chunk_length; // header + json chunk header + json data
    let current_position = reader.position() as usize;
    if current_position != expected_bin_chunk_position {
        return Err(format!("Unexpected position before BIN chunk. Expected: {}, got: {}", 
            expected_bin_chunk_position, current_position).into());
    }
    
    // Read BIN chunk header
    if current_position >= glb_data.len() {
        return Err("Missing BIN chunk".into());
    }
    
    let bin_chunk_length = reader.read_u32::<LittleEndian>()? as usize;
    let bin_chunk_type = reader.read_u32::<LittleEndian>()?;
    if bin_chunk_type != GLB_CHUNK_TYPE_BIN {
        return Err(format!("Second chunk is not BIN, type: 0x{:X}, expected: 0x{:X}", 
            bin_chunk_type, GLB_CHUNK_TYPE_BIN).into());
    }
    
    // Check if BIN chunk is aligned to 4 bytes
    if bin_chunk_length % 4 != 0 {
        return Err(format!("BIN chunk length {} is not aligned to 4 bytes", bin_chunk_length).into());
    }
    
    // Check 4: Validate buffers, buffer views, and accessors
    validate_buffer_references(&json_value, bin_chunk_length)?;
    
    // Check 5: Validate primitives format and modes
    validate_primitives(&json_value)?;
    
    Ok(())
}

fn validate_required_json_fields(json: &Value) -> Result<(), Box<dyn std::error::Error>> {
    // Check for required glTF properties
    let required_root_properties = ["asset", "scene", "scenes", "nodes", "meshes"];
    for prop in required_root_properties.iter() {
        if !json.as_object().unwrap().contains_key(*prop) {
            return Err(format!("Missing required root property: {}", prop).into());
        }
    }
    
    // Check asset version
    if let Some(asset) = json["asset"].as_object() {
        if !asset.contains_key("version") {
            return Err("Missing required asset.version property".into());
        }
        
        let version = asset["version"].as_str().unwrap_or("");
        if version != "2.0" {
            return Err(format!("Invalid asset version: {}, expected: 2.0", version).into());
        }
    } else {
        return Err("Missing asset object".into());
    }
    
    // Check scenes array
    if !json["scenes"].is_array() || json["scenes"].as_array().unwrap().is_empty() {
        return Err("Missing or empty scenes array".into());
    }
    
    // Check nodes array
    if !json["nodes"].is_array() || json["nodes"].as_array().unwrap().is_empty() {
        return Err("Missing or empty nodes array".into());
    }
    
    // Check meshes array
    if !json["meshes"].is_array() || json["meshes"].as_array().unwrap().is_empty() {
        return Err("Missing or empty meshes array".into());
    }
    
    // Validate that each mesh has primitives
    for (i, mesh) in json["meshes"].as_array().unwrap().iter().enumerate() {
        if !mesh.as_object().unwrap().contains_key("primitives") || 
           !mesh["primitives"].is_array() || 
           mesh["primitives"].as_array().unwrap().is_empty() {
            return Err(format!("Mesh {} is missing primitives", i).into());
        }
    }
    
    Ok(())
}

fn validate_buffer_references(json: &Value, bin_chunk_length: usize) -> Result<(), Box<dyn std::error::Error>> {
    // Check buffers
    if !json.as_object().unwrap().contains_key("buffers") || 
       !json["buffers"].is_array() || 
       json["buffers"].as_array().unwrap().is_empty() {
        return Err("Missing buffers array".into());
    }
    
    // Validate buffer lengths
    let buffer = &json["buffers"][0];
    let buffer_length = buffer["byteLength"].as_u64().unwrap_or(0) as usize;
    if buffer_length != bin_chunk_length {
        return Err(format!("Buffer byteLength ({}) doesn't match BIN chunk length ({})", 
            buffer_length, bin_chunk_length).into());
    }
    
    // Check buffer views
    if !json.as_object().unwrap().contains_key("bufferViews") || 
       !json["bufferViews"].is_array() || 
       json["bufferViews"].as_array().unwrap().is_empty() {
        return Err("Missing bufferViews array".into());
    }
    
    // Validate buffer view references and boundaries
    for (i, view) in json["bufferViews"].as_array().unwrap().iter().enumerate() {
        let view_obj = view.as_object().unwrap();
        
        // Check buffer reference
        if !view_obj.contains_key("buffer") {
            return Err(format!("bufferView {} is missing buffer reference", i).into());
        }
        
        // Validate buffer view boundaries
        let byte_offset = view_obj.get("byteOffset").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
        let byte_length = view_obj["byteLength"].as_u64().unwrap_or(0) as usize;
        
        if byte_offset + byte_length > buffer_length {
            return Err(format!("bufferView {} exceeds buffer bounds: offset {} + length {} > buffer size {}", 
                i, byte_offset, byte_length, buffer_length).into());
        }
    }
    
    // Check accessors
    if !json.as_object().unwrap().contains_key("accessors") || 
       !json["accessors"].is_array() || 
       json["accessors"].as_array().unwrap().is_empty() {
        return Err("Missing accessors array".into());
    }
    
    // Validate accessor references
    for (i, accessor) in json["accessors"].as_array().unwrap().iter().enumerate() {
        let accessor_obj = accessor.as_object().unwrap();
        
        // Check buffer view reference
        if !accessor_obj.contains_key("bufferView") {
            return Err(format!("accessor {} is missing bufferView reference", i).into());
        }
        
        let buffer_view_index = accessor_obj["bufferView"].as_u64().unwrap_or(0) as usize;
        let buffer_views = json["bufferViews"].as_array().unwrap();
        
        if buffer_view_index >= buffer_views.len() {
            return Err(format!("accessor {} references non-existent bufferView {}", 
                i, buffer_view_index).into());
        }
        
        // Validate required accessor properties
        if !accessor_obj.contains_key("componentType") {
            return Err(format!("accessor {} is missing componentType", i).into());
        }
        
        if !accessor_obj.contains_key("count") {
            return Err(format!("accessor {} is missing count", i).into());
        }
        
        if !accessor_obj.contains_key("type") {
            return Err(format!("accessor {} is missing type", i).into());
        }
    }
    
    Ok(())
}

fn validate_primitives(json: &Value) -> Result<(), Box<dyn std::error::Error>> {
    // Check each mesh primitive
    for (mesh_idx, mesh) in json["meshes"].as_array().unwrap().iter().enumerate() {
        for (prim_idx, primitive) in mesh["primitives"].as_array().unwrap().iter().enumerate() {
            let prim_obj = primitive.as_object().unwrap();
            
            // Check required primitive properties
            if !prim_obj.contains_key("attributes") {
                return Err(format!("mesh {} primitive {} is missing attributes", mesh_idx, prim_idx).into());
            }
            
            let attributes = prim_obj["attributes"].as_object().unwrap();
            if !attributes.contains_key("POSITION") {
                return Err(format!("mesh {} primitive {} is missing required POSITION attribute", 
                    mesh_idx, prim_idx).into());
            }
            
            // Validate indices (not required, but if present must be valid)
            if prim_obj.contains_key("indices") {
                let indices_index = prim_obj["indices"].as_u64().unwrap_or(0) as usize;
                let accessors = json["accessors"].as_array().unwrap();
                
                if indices_index >= accessors.len() {
                    return Err(format!("mesh {} primitive {} references non-existent accessor {} for indices", 
                        mesh_idx, prim_idx, indices_index).into());
                }
                
                // Validate indices accessor
                let indices_accessor = &accessors[indices_index];
                if indices_accessor["type"].as_str().unwrap_or("") != "SCALAR" {
                    return Err(format!("mesh {} primitive {} indices accessor must be SCALAR type", 
                        mesh_idx, prim_idx).into());
                }
            }
            
            // Validate primitive mode (default is TRIANGLES = 4 if not specified)
            if prim_obj.contains_key("mode") {
                let mode = prim_obj["mode"].as_u64().unwrap_or(4);
                if mode > 6 {
                    return Err(format!("mesh {} primitive {} has invalid mode {}", 
                        mesh_idx, prim_idx, mode).into());
                }
            }
            
            // Validate material reference
            if prim_obj.contains_key("material") {
                let material_index = prim_obj["material"].as_u64().unwrap_or(0) as usize;
                
                // Create a vector to hold materials or an empty vec if none exist
                let materials_vec = Vec::new();
                let materials = json["materials"].as_array().unwrap_or(&materials_vec);
                
                if material_index >= materials.len() {
                    return Err(format!("mesh {} primitive {} references non-existent material {}", 
                        mesh_idx, prim_idx, material_index).into());
                }
            }
        }
    }
    
    Ok(())
}
