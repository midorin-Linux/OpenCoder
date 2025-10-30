use anyhow::Result;
use config::{Config as ConfigBuilder, ConfigError, Environment, File};
use serde::Deserialize;

fn default_api_url() -> String {
    "http://127.0.0.1:1234/v1".to_string()
}

fn default_api_key() -> String {
    "suwako".to_string()
}

fn default_model_name() -> String {
    "qwen3-30b-a3b-instruct-2507".to_string()
}

fn default_rust_log() -> String {
    "info".to_string()
}

fn default_timeout() -> u64 {
    60
}

fn default_top_p() -> f64 {
    0.95
}

fn default_top_k() -> u64 {
    40
}

fn default_temperature() -> f64 {
    0.7
}

fn default_presence_penalty() -> f64 {
    0.0
}

fn default_frequency_penalty() -> f64 {
    0.0
}

fn default_repeat_penalty() -> f64 {
    1.0
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(rename = "api_url", default = "default_api_url")]
    pub api_url: String,

    #[serde(rename = "api_key", default = "default_api_key")]
    pub api_key: String,

    #[serde(rename = "model.name", default = "default_model_name")]
    pub model_name: String,

    #[serde(rename = "model.top_p", default = "default_top_p")]
    pub top_p: f64,

    #[serde(rename = "model.top_k", default = "default_top_k")]
    pub top_k: u64,

    #[serde(rename = "model.temperature", default = "default_temperature")]
    pub temperature: f64,

    #[serde(rename = "model.presence_penalty", default = "default_presence_penalty")]
    pub presence_penalty: f64,

    #[serde(rename = "model.frequency_penalty", default = "default_frequency_penalty")]
    pub frequency_penalty: f64,

    #[serde(rename = "model.repeat_penalty", default = "default_repeat_penalty")]
    pub repeat_penalty: f64,

    #[serde(rename = "rust_log", default = "default_rust_log")]
    pub rust_log: String,

    #[serde(rename = "request_timeout_secs", default = "default_timeout")]
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
            .add_source(
                File::with_name("settings.toml")
                    .required(true)
                    .format(config::FileFormat::Toml),
            )
            .add_source(Environment::default())
            .build()?;

        config.try_deserialize()
    }
}
