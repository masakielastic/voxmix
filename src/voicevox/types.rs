use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Speaker {
    pub name: String,
    pub speaker_uuid: String,
    pub styles: Vec<SpeakerStyle>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeakerStyle {
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioQuery {
    pub accent_phrases: Vec<AccentPhrase>,
    #[serde(rename = "speedScale")]
    pub speed_scale: f32,
    #[serde(rename = "pitchScale")]
    pub pitch_scale: f32,
    #[serde(rename = "intonationScale")]
    pub intonation_scale: f32,
    #[serde(rename = "volumeScale")]
    pub volume_scale: f32,
    #[serde(rename = "prePhonemeLength")]
    pub pre_phoneme_length: f32,
    #[serde(rename = "postPhonemeLength")]
    pub post_phoneme_length: f32,
    #[serde(rename = "outputSamplingRate")]
    pub output_sampling_rate: u32,
    #[serde(rename = "outputStereo")]
    pub output_stereo: bool,
    #[serde(rename = "pauseLength")]
    pub pause_length: Option<f32>,
    #[serde(rename = "pauseLengthScale")]
    pub pause_length_scale: f32,
    pub kana: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccentPhrase {
    pub moras: Vec<Mora>,
    pub accent: u32,
    pub pause_mora: Option<Mora>,
    pub is_interrogative: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mora {
    pub text: String,
    pub consonant: Option<String>,
    pub consonant_length: Option<f32>,
    pub vowel: String,
    pub vowel_length: f32,
    pub pitch: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AudioQueryRequest {
    pub text: String,
    pub speaker: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SynthesisRequest {
    pub audio_query: AudioQuery,
    pub speaker: u32,
}