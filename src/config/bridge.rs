//! Bridge translation configuration.
//!
//! When enabled, transcription output is translated between a pair of
//! languages via a Groq Chat API call (bidirectional — the model
//! auto-detects the source language from the configured pair).

use serde::{Deserialize, Serialize};

/// Bidirectional translation bridge configuration.
///
/// ```toml
/// [bridge]
/// enabled = true
/// pair = "ru:es"
/// ```
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct BridgeConfig {
    #[serde(default)]
    pub enabled: bool,

    /// Language pair in `lang_a:lang_b` format.
    /// Separators `:`, `-`, `_`, `/` are all accepted and normalised to `:`.
    #[serde(default)]
    pub pair: Option<String>,
}

/// Separator characters accepted inside the `pair` field.
const PAIR_SEPARATORS: &[char] = &[':', '-', '_', '/'];

impl BridgeConfig {
    /// `true` when the bridge is both enabled and has a valid language pair.
    pub fn is_active(&self) -> bool {
        self.enabled && self.pair.is_some()
    }

    /// Parse the `pair` field into a `(lang_a, lang_b)` tuple.
    ///
    /// Accepts `:`, `-`, `_`, `/` as separators. Returns `None` when the
    /// pair is missing, empty, or cannot be split into two non-empty parts.
    pub fn languages(&self) -> Option<(String, String)> {
        let pair = self.pair.as_ref()?;
        let sep = pair.chars().find(|c| PAIR_SEPARATORS.contains(c))?;
        let parts: Vec<&str> = pair.splitn(2, sep).collect();
        if parts.len() == 2 && !parts[0].is_empty() && !parts[1].is_empty() {
            Some((parts[0].trim().to_string(), parts[1].trim().to_string()))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── is_active ──────────────────────────────────────────────────

    #[test]
    fn is_active_default_is_false() {
        assert!(!BridgeConfig::default().is_active());
    }

    #[test]
    fn is_active_enabled_without_pair() {
        assert!(!BridgeConfig {
            enabled: true,
            pair: None,
        }
        .is_active());
    }

    #[test]
    fn is_active_pair_without_enabled() {
        assert!(!BridgeConfig {
            enabled: false,
            pair: Some("ru:es".into()),
        }
        .is_active());
    }

    #[test]
    fn is_active_enabled_with_pair() {
        assert!(BridgeConfig {
            enabled: true,
            pair: Some("ru:es".into()),
        }
        .is_active());
    }

    // ── languages parsing ──────────────────────────────────────────

    #[test]
    fn languages_colon_separator() {
        let cfg = BridgeConfig {
            enabled: true,
            pair: Some("ru:es".into()),
        };
        assert_eq!(cfg.languages(), Some(("ru".into(), "es".into())));
    }

    #[test]
    fn languages_dash_separator() {
        let cfg = BridgeConfig {
            enabled: true,
            pair: Some("ru-es".into()),
        };
        assert_eq!(cfg.languages(), Some(("ru".into(), "es".into())));
    }

    #[test]
    fn languages_underscore_separator() {
        let cfg = BridgeConfig {
            enabled: true,
            pair: Some("ru_es".into()),
        };
        assert_eq!(cfg.languages(), Some(("ru".into(), "es".into())));
    }

    #[test]
    fn languages_slash_separator() {
        let cfg = BridgeConfig {
            enabled: true,
            pair: Some("ru/es".into()),
        };
        assert_eq!(cfg.languages(), Some(("ru".into(), "es".into())));
    }

    #[test]
    fn languages_trim_whitespace() {
        let cfg = BridgeConfig {
            enabled: true,
            pair: Some("  ru  :  es  ".into()),
        };
        assert_eq!(cfg.languages(), Some(("ru".into(), "es".into())));
    }

    #[test]
    fn languages_none_when_no_separator() {
        let cfg = BridgeConfig {
            enabled: true,
            pair: Some("rues".into()),
        };
        assert_eq!(cfg.languages(), None);
    }

    #[test]
    fn languages_none_when_empty_pair() {
        let cfg = BridgeConfig {
            enabled: true,
            pair: Some("".into()),
        };
        assert_eq!(cfg.languages(), None);
    }

    #[test]
    fn languages_none_when_no_pair() {
        let cfg = BridgeConfig::default();
        assert_eq!(cfg.languages(), None);
    }

    #[test]
    fn languages_none_when_empty_left() {
        let cfg = BridgeConfig {
            enabled: true,
            pair: Some(":es".into()),
        };
        assert_eq!(cfg.languages(), None);
    }

    #[test]
    fn languages_none_when_empty_right() {
        let cfg = BridgeConfig {
            enabled: true,
            pair: Some("ru:".into()),
        };
        assert_eq!(cfg.languages(), None);
    }

    // ── serde round-trip ───────────────────────────────────────────

    #[test]
    fn serde_round_trip() {
        let cfg = BridgeConfig {
            enabled: true,
            pair: Some("ru:es".into()),
        };
        let json = serde_json::to_string(&cfg).unwrap();
        let deserialized: BridgeConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(cfg, deserialized);
    }

    #[test]
    fn serde_default_when_missing() {
        #[derive(Deserialize)]
        struct Wrapper {
            #[serde(default)]
            bridge: BridgeConfig,
        }
        let w: Wrapper = serde_json::from_str("{}").unwrap();
        assert_eq!(w.bridge, BridgeConfig::default());
    }

    // ── TOML deserialization ───────────────────────────────────────

    #[test]
    fn toml_deserialize_bridge_section() {
        let toml = r#"
            [bridge]
            enabled = true
            pair = "ru:es"
        "#;
        #[derive(Deserialize)]
        struct Wrapper {
            bridge: BridgeConfig,
        }
        let w: Wrapper = toml::from_str(toml).unwrap();
        assert!(w.bridge.enabled);
        assert_eq!(w.bridge.pair.as_deref(), Some("ru:es"));
    }

    #[test]
    fn toml_default_when_section_missing() {
        #[derive(Deserialize)]
        struct Wrapper {
            #[serde(default)]
            bridge: BridgeConfig,
        }
        let w: Wrapper = toml::from_str("").unwrap();
        assert_eq!(w.bridge, BridgeConfig::default());
    }
}
