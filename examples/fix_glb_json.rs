use std::fs;
use std::io::{Cursor, Read, Seek, SeekFrom, Write};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use serde_json::{self, Value};

const GLB_MAGIC: u32 = 0x46546C67; // "glTF" in ASCII
const GLB_VERSION: u32 = 2;
const GLB_CHUNK_TYPE_JSON: u32 = 0x4E4F534A; // "JSON" in ASCII
const GLB_CHUNK_TYPE_BIN: u32 = 0x004E4942; // "BIN\0" in ASCII

fn main() {
    // Process the multi-material scene GLB file
    let file_path = "output/multi_material_scene.glb";
    println!("Fixing GLB file: {}", file_path);
    
    fix_glb_file(file_path).unwrap();
    
    println!("GLB file fixed successfully.");
}

fn fix_glb_file(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Read the entire GLB file
    let glb_data = fs::read(file_path)?;
    let mut reader = Cursor::new(&glb_data);
    
    // Read GLB header
    let magic = reader.read_u32::<LittleEndian>()?;
    assert_eq!(magic, GLB_MAGIC, "Not a valid GLB file");
    
    let version = reader.read_u32::<LittleEndian>()?;
    assert_eq!(version, GLB_VERSION, "Unsupported GLB version");
    
    let total_length = reader.read_u32::<LittleEndian>()?;
    
    // Read JSON chunk
    let json_chunk_length = reader.read_u32::<LittleEndian>()?;
    let json_chunk_type = reader.read_u32::<LittleEndian>()?;
    assert_eq!(json_chunk_type, GLB_CHUNK_TYPE_JSON, "First chunk is not JSON");
    
    // Read the JSON data
    let mut json_data = vec![0u8; json_chunk_length as usize];
    reader.read_exact(&mut json_data)?;
    
    // Parse JSON and fix formatting
    let json_str = String::from_utf8_lossy(&json_data);
    let json_value: Value = serde_json::from_str(&json_str)?;
    
    // Re-serialize to compact, well-formed JSON without control characters
    let fixed_json = serde_json::to_string(&json_value)?;
    let fixed_json_bytes = fixed_json.as_bytes();
    let fixed_json_length = fixed_json_bytes.len();
    
    // Calculate padding for JSON chunk (must be aligned to 4 bytes)
    let json_padding_length = (4 - (fixed_json_length % 4)) % 4;
    let json_padded_length = fixed_json_length + json_padding_length;
    
    // Read BIN chunk
    let bin_chunk_length = reader.read_u32::<LittleEndian>()?;
    let bin_chunk_type = reader.read_u32::<LittleEndian>()?;
    assert_eq!(bin_chunk_type, GLB_CHUNK_TYPE_BIN, "Second chunk is not BIN");
    
    // Read the binary data
    let mut bin_data = vec![0u8; bin_chunk_length as usize];
    reader.read_exact(&mut bin_data)?;
    
    // Calculate padding for BIN chunk (must be aligned to 4 bytes)
    let bin_padding_length = (4 - (bin_data.len() % 4)) % 4;
    let bin_padded_length = bin_data.len() + bin_padding_length;
    
    // Calculate new total file length
    let new_total_length = 12 + // GLB header (magic + version + length)
                           8 + json_padded_length + // JSON chunk header + padded data
                           8 + bin_padded_length; // BIN chunk header + padded data
    
    // Create new GLB file
    let fixed_file_path = format!("{}.fixed", file_path);
    let file = fs::File::create(&fixed_file_path)?;
    let mut writer = std::io::BufWriter::new(file);
    
    // Write GLB header
    writer.write_u32::<LittleEndian>(GLB_MAGIC)?;
    writer.write_u32::<LittleEndian>(GLB_VERSION)?;
    writer.write_u32::<LittleEndian>(new_total_length as u32)?;
    
    // Write JSON chunk
    writer.write_u32::<LittleEndian>(json_padded_length as u32)?;
    writer.write_u32::<LittleEndian>(GLB_CHUNK_TYPE_JSON)?;
    writer.write_all(fixed_json_bytes)?;
    
    // Write JSON padding (to align to 4 bytes)
    for _ in 0..json_padding_length {
        writer.write_u8(0)?;
    }
    
    // Write BIN chunk
    writer.write_u32::<LittleEndian>(bin_padded_length as u32)?;
    writer.write_u32::<LittleEndian>(GLB_CHUNK_TYPE_BIN)?;
    writer.write_all(&bin_data)?;
    
    // Write BIN padding (to align to 4 bytes)
    for _ in 0..bin_padding_length {
        writer.write_u8(0)?;
    }
    
    // Ensure all data is written
    writer.flush()?;
    
    // Replace the original file with the fixed one
    fs::rename(fixed_file_path, file_path)?;
    
    Ok(())
}
