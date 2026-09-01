//! Translation bridge — bidirectional translation via Groq Chat API.
//!
//! After Whisper transcribes speech in auto-detect mode, the bridge
//! optionally translates the text between a configured language pair
//! using a single Groq Chat completion call.

use crate::config::BridgeConfig;
use serde_json::{json, Value};
use std::time::Duration;

/// Timeout for translation API calls.
const TRANSLATION_TIMEOUT_SECS: u64 = 15;

/// Bidirectional translation bridge.
///
/// Uses the Groq Chat completions API (OpenAI-compatible) to translate
/// text between two languages. The model auto-detects the source
/// language from the pair and translates to the other.
pub struct TranslationBridge {
    api_key: String,
    endpoint: String,
    model: String,
    timeout: Duration,
}

/// Errors that can occur during translation.
#[derive(Debug)]
pub enum BridgeError {
    /// The API request timed out.
    Timeout,
    /// The API returned an error or non-success status.
    ApiError(String),
    /// The API returned an empty translation.
    Empty,
    /// No API key was provided.
    NoApiKey,
}

impl std::fmt::Display for BridgeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BridgeError::Timeout => write!(f, "translation request timed out"),
            BridgeError::ApiError(msg) => write!(f, "translation API error: {}", msg),
            BridgeError::Empty => write!(f, "translation API returned empty response"),
            BridgeError::NoApiKey => write!(f, "no API key configured for translation bridge"),
        }
    }
}

impl TranslationBridge {
    /// Build a bridge from config. Returns `None` when the bridge is
    /// inactive or the required API key is missing.
    pub fn from_config(
        bridge: &BridgeConfig,
        whisper_api_key: Option<&str>,
        whisper_endpoint: Option<&str>,
    ) -> Option<Self> {
        if !bridge.is_active() {
            return None;
        }
        let api_key = whisper_api_key?.to_string();
        let endpoint = whisper_endpoint
            .unwrap_or("https://api.groq.com/openai")
            .to_string();
        Some(Self {
            api_key,
            endpoint,
            model: "llama-3.3-70b-versatile".to_string(),
            timeout: Duration::from_secs(TRANSLATION_TIMEOUT_SECS),
        })
    }

    /// Translate `text` between the two languages in `pair`.
    ///
    /// Sends a single Chat completion request asking the model to
    /// auto-detect whether the text is in `pair.0` or `pair.1` and
    /// translate to the other.
    pub async fn translate(
        &self,
        text: &str,
        pair: (String, String),
    ) -> Result<String, BridgeError> {
        let prompt = format!(
            "You will receive text in either {} or {}. \
             If it is {}, translate to {}. \
             If it is {}, translate to {}. \
             Output only the translation, nothing else. \
             Text: \"{}\"",
            pair.0, pair.1, pair.0, pair.1, pair.1, pair.0, text
        );

        let url = format!(
            "{}/v1/chat/completions",
            self.endpoint.trim_end_matches('/')
        );

        let body = json!({
            "model": self.model,
            "messages": [
                { "role": "user", "content": prompt }
            ],
            "temperature": 0.3,
        });

        let client = reqwest::Client::builder()
            .timeout(self.timeout)
            .build()
            .map_err(|e| BridgeError::ApiError(format!("failed to build HTTP client: {}", e)))?;

        let response = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| {
                if e.is_timeout() {
                    BridgeError::Timeout
                } else {
                    BridgeError::ApiError(format!("request failed: {}", e))
                }
            })?;

        if !response.status().is_success() {
            let status = response.status();
            let body_text = response.text().await.unwrap_or_default();
            return Err(BridgeError::ApiError(format!(
                "HTTP {}: {}",
                status, body_text
            )));
        }

        let json: Value = response
            .json()
            .await
            .map_err(|e| BridgeError::ApiError(format!("failed to parse response JSON: {}", e)))?;

        let translated = json
            .get("choices")
            .and_then(|c| c.get(0))
            .and_then(|c| c.get("message"))
            .and_then(|m| m.get("content"))
            .and_then(|c| c.as_str())
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .ok_or(BridgeError::Empty)?
            .to_string();

        Ok(translated)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_config_returns_none_when_inactive() {
        let bridge = BridgeConfig::default();
        assert!(TranslationBridge::from_config(&bridge, Some("key"), None).is_none());
    }

    #[test]
    fn from_config_returns_none_without_api_key() {
        let bridge = BridgeConfig {
            enabled: true,
            pair: Some("ru:es".into()),
        };
        assert!(TranslationBridge::from_config(&bridge, None, None).is_none());
    }

    #[test]
    fn from_config_returns_bridge_when_active() {
        let bridge = BridgeConfig {
            enabled: true,
            pair: Some("ru:es".into()),
        };
        let tb = TranslationBridge::from_config(&bridge, Some("sk-test"), None).unwrap();
        assert_eq!(tb.model, "llama-3.3-70b-versatile");
        assert_eq!(tb.endpoint, "https://api.groq.com/openai");
        assert_eq!(tb.timeout, Duration::from_secs(15));
    }

    #[test]
    fn from_config_uses_custom_endpoint() {
        let bridge = BridgeConfig {
            enabled: true,
            pair: Some("ru:es".into()),
        };
        let tb = TranslationBridge::from_config(
            &bridge,
            Some("sk-test"),
            Some("https://custom.api.com"),
        )
        .unwrap();
        assert_eq!(tb.endpoint, "https://custom.api.com");
    }

    #[test]
    fn bridge_error_display() {
        assert_eq!(
            BridgeError::Timeout.to_string(),
            "translation request timed out"
        );
        assert_eq!(
            BridgeError::Empty.to_string(),
            "translation API returned empty response"
        );
        assert_eq!(
            BridgeError::NoApiKey.to_string(),
            "no API key configured for translation bridge"
        );
        assert!(BridgeError::ApiError("test".into())
            .to_string()
            .contains("test"));
    }
}
