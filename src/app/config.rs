use anyhow::Result;
use config::{Config as ConfigBuilder, ConfigError, Environment, File};
use serde::Deserialize;

fn default_api_url() -> String {
    "http://127.0.0.1:1234/v1".to_string()
}

fn default_api_key() -> String {
    "suwako".to_string()
}

fn default_model() -> String {
    "Qwen/Qwen3-4B-Thinking-2507".to_string()
}

fn default_rust_log() -> String {
    "info".to_string()
}

fn default_timeout() -> u64 {
    60
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(rename = "API_URL", default = "default_api_url")]
    pub api_url: String,

    #[serde(rename = "API_KEY", default = "default_api_key")]
    pub api_key: String,

    #[serde(rename = "MODEL", default = "default_model")]
    pub model: String,

    #[serde(rename = "RUST_LOG", default = "default_rust_log")]
    pub rust_log: String,

    #[serde(rename = "TIMEOUT_SECS", default = "default_timeout")]
    pub request_timeout_secs: u64,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenvy::dotenv().ok();

        let config = ConfigBuilder::builder()
            .add_source(
                File::with_name(".env")
                    .format(config::FileFormat::Ini)
                    .required(false),
            )
            .add_source(Environment::default())
            .build()?;

        config.try_deserialize()
    }
}
