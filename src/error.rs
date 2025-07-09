use thiserror::Error;

#[derive(Error, Debug)]
pub enum VoxmixError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),
    
    #[error("JSON parsing failed: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Audio processing error: {0}")]
    Audio(#[from] hound::Error),
    
    #[error("Speaker '{0}' not found")]
    SpeakerNotFound(String),
    
    #[error("VOICEVOX API error: {0}")]
    VoicevoxApi(String),
    
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),
}

pub type Result<T> = std::result::Result<T, VoxmixError>;