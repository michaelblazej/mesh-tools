use std::fs;
use std::path::Path;
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{Cursor, Read, Seek, SeekFrom};

const GLB_MAGIC: u32 = 0x46546C67; // "glTF" in ASCII
const GLB_CHUNK_TYPE_JSON: u32 = 0x4E4F534A; // "JSON" in ASCII

fn main() {
    let file_path = "output/multi_material_scene.glb";
    println!("Inspecting GLB file: {}", file_path);
    
    let glb_data = fs::read(file_path).expect("Failed to read GLB file");
    let mut reader = Cursor::new(&glb_data);
    
    // Read GLB header
    let magic = reader.read_u32::<LittleEndian>().expect("Failed to read magic");
    assert_eq!(magic, GLB_MAGIC, "Not a valid GLB file");
    
    let version = reader.read_u32::<LittleEndian>().expect("Failed to read version");
    println!("GLB version: {}", version);
    
    let length = reader.read_u32::<LittleEndian>().expect("Failed to read length");
    println!("GLB file length: {} bytes", length);
    
    // Read JSON chunk
    let chunk_length = reader.read_u32::<LittleEndian>().expect("Failed to read chunk length");
    let chunk_type = reader.read_u32::<LittleEndian>().expect("Failed to read chunk type");
    
    assert_eq!(chunk_type, GLB_CHUNK_TYPE_JSON, "First chunk is not JSON");
    println!("JSON chunk length: {} bytes", chunk_length);
    
    let mut json_data = vec![0u8; chunk_length as usize];
    reader.read_exact(&mut json_data).expect("Failed to read JSON data");
    
    // Convert to string
    let json_str = String::from_utf8_lossy(&json_data);
    
    // Print JSON for inspection, highlighting any control characters
    println!("\nJSON data with control character markers:");
    for (i, c) in json_str.chars().enumerate() {
        if c.is_control() {
            println!("Control character at position {}: U+{:04X}", i, c as u32);
        }
    }
    
    // Print a section around the problematic area (line 9, col 33, char 151)
    println!("\nJSON section around error (chars 140-160):");
    if json_str.len() > 160 {
        let start = 140.min(json_str.len());
        let end = 160.min(json_str.len());
        let section = &json_str[start..end];
        
        println!("{}", section);
        println!("{}^", " ".repeat(151 - start));
    }
    
    // Write JSON to file for inspection
    fs::write("output/debug_json.json", json_str.as_bytes()).expect("Failed to write JSON debug file");
    println!("\nWrote JSON to output/debug_json.json for inspection");
}
