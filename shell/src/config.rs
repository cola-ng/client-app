//! Configuration management for Colang
//!
//! Handles loading configuration from multiple sources:
//! 1. Default values
//! 2. Config file (~/.colang/config.toml, ~/.colang/config.yaml, ~/.colang/config.json)
//! 3. Environment variables

use std::path::PathBuf;

use figment::Figment;
use figment::providers::{Env, Format, Json, Toml, Yaml};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Website URL for desktop auth
    #[serde(default = "default_website_url")]
    pub website_url: String,

    /// API URL for backend services
    #[serde(default = "default_api_url")]
    pub api_url: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            website_url: default_website_url(),
            api_url: default_api_url(),
        }
    }
}

fn default_website_url() -> String {
    "http://127.0.0.1:6108".to_string()
}

fn default_api_url() -> String {
    "http://127.0.0.1:6108/api".to_string()
}

impl Config {
    /// Load configuration from multiple sources with priority:
    /// 1. Environment variables (highest priority)
    /// 2. Config files (toml, yaml, json)
    /// 3. Default values (lowest priority)
    pub fn load() -> Result<Self, figment::Error> {
        let config_dir = Self::ensure_config_dir()?;

        Figment::new()
            // Start with defaults
            .merge(figment::providers::Serialized::defaults(Config::default()))
            // Try loading from config files (in order of preference)
            .merge(Toml::file(config_dir.join("config.toml")).nested())
            .merge(Yaml::file(config_dir.join("config.yaml")).nested())
            .merge(Json::file(config_dir.join("config.json")).nested())
            // Override with environment variables (COLANG_WEBSITE_URL, COLANG_API_URL)
            .merge(Env::prefixed("COLANG_"))
            .extract()
    }

    pub fn profile_url(&self) -> String {
        format!("{}/me/", self.website_url)
    }

    /// Ensure ~/.colang directory exists, create if it doesn't
    fn ensure_config_dir() -> Result<PathBuf, figment::Error> {
        let home_dir = dirs::home_dir().ok_or_else(|| {
            figment::Error::from("Unable to determine home directory".to_string())
        })?;

        let config_dir = home_dir.join(".colang");

        if !config_dir.exists() {
            std::fs::create_dir_all(&config_dir).map_err(|e| {
                figment::Error::from(format!("Failed to create config directory: {}", e))
            })?;

            // Create a default config.toml file
            let default_config_path = config_dir.join("config.toml");
            if !default_config_path.exists() {
                let default_content = r#"# Colang Configuration
# Website URL for desktop authentication
# website_url = "http://127.0.0.1:6108"

# API URL for backend services
# api_url = "http://127.0.0.1:5800"
"#;
                std::fs::write(&default_config_path, default_content).map_err(|e| {
                    figment::Error::from(format!("Failed to create default config file: {}", e))
                })?;
            }
        }

        Ok(config_dir)
    }
}
