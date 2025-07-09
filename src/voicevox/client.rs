use reqwest::Client;
use std::path::Path;
use tracing::{debug, info};
use indicatif::{ProgressBar, ProgressStyle};

use crate::error::{Result, VoxmixError};
use super::types::{Speaker, AudioQuery};

pub struct VoicevoxClient {
    client: Client,
    base_url: String,
    speakers_cache: Option<Vec<Speaker>>,
}

impl VoicevoxClient {
    pub fn new(host: &str, port: u16) -> Result<Self> {
        let client = Client::new();
        let base_url = format!("http://{}:{}", host, port);
        
        Ok(Self {
            client,
            base_url,
            speakers_cache: None,
        })
    }
    
    pub async fn get_speakers(&mut self) -> Result<&Vec<Speaker>> {
        if self.speakers_cache.is_none() {
            let url = format!("{}/speakers", self.base_url);
            debug!("Fetching speakers from: {}", url);
            
            let response = self.client.get(&url).send().await?;
            
            if !response.status().is_success() {
                return Err(VoxmixError::VoicevoxApi(
                    format!("Failed to fetch speakers: {}", response.status())
                ));
            }
            
            let speakers: Vec<Speaker> = response.json().await?;
            info!("Loaded {} speakers", speakers.len());
            self.speakers_cache = Some(speakers);
        }
        
        Ok(self.speakers_cache.as_ref().unwrap())
    }
    
    pub async fn resolve_speaker(&mut self, speaker_name: &str) -> Result<u32> {
        // Try to parse as ID first
        if let Ok(id) = speaker_name.parse::<u32>() {
            return Ok(id);
        }
        
        // Search by name
        let speakers = self.get_speakers().await?;
        
        for speaker in speakers {
            if speaker.name == speaker_name {
                if let Some(style) = speaker.styles.first() {
                    return Ok(style.id);
                }
            }
            
            // Check styles with parentheses format like "四国めたん（あまあま）"
            for style in &speaker.styles {
                let full_name = format!("{}（{}）", speaker.name, style.name);
                if full_name == speaker_name {
                    return Ok(style.id);
                }
            }
        }
        
        Err(VoxmixError::SpeakerNotFound(speaker_name.to_string()))
    }
    
    pub async fn create_audio_query(&self, text: &str, speaker_id: u32) -> Result<AudioQuery> {
        let url = format!("{}/audio_query", self.base_url);
        debug!("Creating audio query for speaker {} at: {}", speaker_id, url);
        
        let params = [
            ("text", text),
            ("speaker", &speaker_id.to_string()),
        ];
        
        let response = self.client
            .post(&url)
            .query(&params)
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(VoxmixError::VoicevoxApi(
                format!("Failed to create audio query: {}", response.status())
            ));
        }
        
        let audio_query: AudioQuery = response.json().await?;
        Ok(audio_query)
    }
    
    pub fn modify_audio_query(
        &self,
        mut audio_query: AudioQuery,
        speed: f32,
        pitch: f32,
        volume: f32,
    ) -> Result<AudioQuery> {
        audio_query.speed_scale = speed;
        // pitchScale is an offset value, not a multiplier
        // Convert from multiplier (1.0 = default) to offset (0.0 = default)
        audio_query.pitch_scale = (pitch - 1.0) * 0.15;
        audio_query.volume_scale = volume;
        
        Ok(audio_query)
    }
    
    pub async fn synthesize_audio(&self, audio_query: &AudioQuery, speaker_id: u32) -> Result<Vec<u8>> {
        let url = format!("{}/synthesis", self.base_url);
        debug!("Synthesizing audio for speaker {} at: {}", speaker_id, url);
        
        let params = [("speaker", speaker_id.to_string())];
        
        let pb = ProgressBar::new_spinner();
        pb.set_style(ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap());
        pb.set_message("Synthesizing audio...");
        pb.enable_steady_tick(std::time::Duration::from_millis(100));
        
        let response = self.client
            .post(&url)
            .query(&params)
            .json(audio_query)
            .send()
            .await?;
        
        pb.finish_with_message("Audio synthesis completed");
        
        if !response.status().is_success() {
            return Err(VoxmixError::VoicevoxApi(
                format!("Failed to synthesize audio: {}", response.status())
            ));
        }
        
        let audio_data = response.bytes().await?;
        Ok(audio_data.to_vec())
    }
    
    pub fn save_audio(&self, audio_data: &[u8], output_path: &Path) -> Result<()> {
        debug!("Saving audio to: {}", output_path.display());
        
        // VOICEVOX returns WAV format, so we can write it directly
        std::fs::write(output_path, audio_data)?;
        
        info!("Audio saved successfully to: {}", output_path.display());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[test]
    fn test_client_creation() {
        let client = VoicevoxClient::new("localhost", 50021);
        assert!(client.is_ok());
    }
    
    #[test]
    fn test_modify_audio_query() {
        let client = VoicevoxClient::new("localhost", 50021).unwrap();
        
        let audio_query = AudioQuery {
            accent_phrases: vec![],
            speed_scale: 1.0,
            pitch_scale: 1.0,
            intonation_scale: 1.0,
            volume_scale: 1.0,
            pre_phoneme_length: 0.1,
            post_phoneme_length: 0.1,
            output_sampling_rate: 24000,
            output_stereo: false,
            pause_length: None,
            pause_length_scale: 1.0,
            kana: "".to_string(),
        };
        
        let modified = client.modify_audio_query(audio_query, 1.5, 1.2, 0.8).unwrap();
        
        assert_eq!(modified.speed_scale, 1.5);
        // pitch 1.2 should convert to offset (1.2 - 1.0) * 0.15 = 0.03
        assert!((modified.pitch_scale - 0.03).abs() < 0.001);
        assert_eq!(modified.volume_scale, 0.8);
    }
    
    #[test]
    fn test_save_audio() {
        let client = VoicevoxClient::new("localhost", 50021).unwrap();
        let temp_dir = TempDir::new().unwrap();
        let output_path = temp_dir.path().join("test.wav");
        
        let test_data = b"test audio data";
        let result = client.save_audio(test_data, &output_path);
        
        assert!(result.is_ok());
        assert!(output_path.exists());
        
        let saved_data = std::fs::read(&output_path).unwrap();
        assert_eq!(saved_data, test_data);
    }
}