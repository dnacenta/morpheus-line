use reqwest::multipart;
use serde::Deserialize;

/// Groq Whisper speech-to-text client.
pub struct SttClient {
    client: reqwest::Client,
    api_key: String,
    model: String,
}

#[derive(Debug, Deserialize)]
struct TranscriptionResponse {
    text: String,
}

impl SttClient {
    pub fn new(api_key: String, model: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key,
            model,
        }
    }

    /// Transcribe WAV audio bytes to text using Groq Whisper.
    pub async fn transcribe(&self, wav_data: Vec<u8>) -> Result<String, SttError> {
        let file_part = multipart::Part::bytes(wav_data)
            .file_name("audio.wav")
            .mime_str("audio/wav")
            .map_err(|e| SttError::Request(e.to_string()))?;

        let form = multipart::Form::new()
            .text("model", self.model.clone())
            .text("language", "en")
            .part("file", file_part);

        let resp = self
            .client
            .post("https://api.groq.com/openai/v1/audio/transcriptions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .multipart(form)
            .send()
            .await
            .map_err(|e| SttError::Request(e.to_string()))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(SttError::Api(format!("{status}: {body}")));
        }

        let result: TranscriptionResponse = resp
            .json()
            .await
            .map_err(|e| SttError::Request(e.to_string()))?;

        Ok(result.text)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SttError {
    #[error("HTTP request failed: {0}")]
    Request(String),
    #[error("API error: {0}")]
    Api(String),
}
