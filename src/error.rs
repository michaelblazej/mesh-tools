use std::io;
use thiserror::Error;
use crate::texture;

/// Error type for glTF export operations
#[derive(Error, Debug)]
pub enum GltfError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    
    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("Invalid data: {0}")]
    InvalidData(String),
    
    #[error("Invalid index")]
    InvalidIndex,
    
    #[error("Texture error: {0:?}")]
    Texture(#[from] texture::TextureError),
}

/// Result type for glTF export operations
pub type Result<T> = std::result::Result<T, GltfError>;
