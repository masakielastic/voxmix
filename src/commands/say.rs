use clap::Args;
use anyhow::Result;
use std::path::PathBuf;
use tracing::{info, debug};

use crate::voicevox::VoicevoxClient;
use crate::error::VoxmixError;

#[derive(Args)]
pub struct SayCommand {
    /// Text to synthesize
    pub text: String,
    
    /// Speaker name or ID
    #[arg(long, default_value = "ずんだもん")]
    pub speaker: String,
    
    /// Output file path
    #[arg(short, long, default_value = "out.wav")]
    pub output: PathBuf,
    
    /// Speech speed (1.0 = default)
    #[arg(long, default_value = "1.0")]
    pub speed: f32,
    
    /// Voice pitch multiplier (1.0 = default, 0.5-2.0 range)
    #[arg(long, default_value = "1.0")]
    pub pitch: f32,
    
    /// Voice volume (1.0 = default)
    #[arg(long, default_value = "1.0")]
    pub volume: f32,
    
    /// VOICEVOX server host
    #[arg(long, default_value = "127.0.0.1")]
    pub host: String,
    
    /// VOICEVOX server port
    #[arg(long, default_value = "50021")]
    pub port: u16,
}

impl SayCommand {
    pub async fn execute(&self) -> Result<()> {
        self.validate_parameters()?;
        
        info!("Connecting to VOICEVOX server at {}:{}", self.host, self.port);
        let mut client = VoicevoxClient::new(&self.host, self.port)?;
        
        info!("Resolving speaker: {}", self.speaker);
        let speaker_id = client.resolve_speaker(&self.speaker).await?;
        debug!("Speaker ID: {}", speaker_id);
        
        info!("Creating audio query for text: {}", self.text);
        let audio_query = client.create_audio_query(&self.text, speaker_id).await?;
        
        info!("Applying voice parameters");
        let modified_query = client.modify_audio_query(
            audio_query,
            self.speed,
            self.pitch,
            self.volume,
        )?;
        
        info!("Synthesizing audio");
        let audio_data = client.synthesize_audio(&modified_query, speaker_id).await?;
        
        info!("Saving audio to: {}", self.output.display());
        client.save_audio(&audio_data, &self.output)?;
        
        info!("Speech synthesis completed successfully");
        Ok(())
    }
    
    fn validate_parameters(&self) -> Result<()> {
        if self.text.is_empty() {
            return Err(VoxmixError::InvalidParameter("Text cannot be empty".to_string()).into());
        }
        
        if self.speed <= 0.0 {
            return Err(VoxmixError::InvalidParameter("Speed must be greater than 0".to_string()).into());
        }
        
        if self.pitch <= 0.0 {
            return Err(VoxmixError::InvalidParameter("Pitch must be greater than 0".to_string()).into());
        }
        
        if self.volume <= 0.0 {
            return Err(VoxmixError::InvalidParameter("Volume must be greater than 0".to_string()).into());
        }
        
        Ok(())
    }
}