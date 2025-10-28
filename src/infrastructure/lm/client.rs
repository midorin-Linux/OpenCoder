use crate::app::config::Config;

use anyhow::{Context, Result};
use futures::StreamExt;
use reqwest::{Client as HttpClient, Error, Response};
use reqwest_eventsource::{Event, EventSource};
use serde_json::json;
use tracing::{debug, error, info, instrument, warn};

#[derive(Clone)]
pub struct ModelSettings {
    pub name: String,
    pub top_p: f64,
    pub top_k: u64,
    pub temperature: f64,
    pub presence_penalty: f64,
    pub frequency_penalty: f64,
    pub repeat_penalty: f64,
}

pub struct Client {
    api_url: String,
    api_key: String,
    http_client: HttpClient,
}

impl Client {
    pub fn new(config: Config) -> Result<Self> {
        let api_url = config.api_url.to_string();

        let api_key = config.api_key.to_string();

        let http_client = HttpClient::builder()
            .timeout(std::time::Duration::from_secs(config.request_timeout_secs))
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Self {
            api_url,
            api_key,
            http_client,
        })
    }

    pub async fn get_model_list(&self) -> Result<Response> {
        debug!("Getting model list...");

        match self
            .http_client
            .get(format!("{}/models", self.api_url))
            .header("Content-Type", "application/json")
            .send()
            .await
        {
            Ok(res) => {
                if res.status().is_success() {
                    debug!("Models list getting successful");
                    Ok(res)
                } else {
                    warn!("Models getting failed with status code {}", res.status());
                    Err(anyhow::anyhow!("Health checking failed"))
                }
            }
            Err(e) => {
                error!("Models getting failed with error: {}", e);
                Err(anyhow::anyhow!("Models getting failed"))
            }
        }
    }

    pub async fn chat_completions(&self, model: ModelSettings, prompt: String) -> Result<Response> {
        debug!("Posting chat completions...");

        let request_body = json!({
            "model": model.name,
            "messages": [
                { "role": "user", "content": prompt }
            ],
            "top_p": model.top_p,
            "top_k": model.top_k,
            "temperature": model.temperature,
            "presence_penalty": model.presence_penalty,
            "frequency_penalty": model.frequency_penalty,
            "repeat_penalty": model.repeat_penalty
        });

        match self
            .http_client
            .post(format!("{}/chat/completions", self.api_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request_body)
            .send()
            .await
        {
            Ok(res) => {
                if res.status().is_success() {
                    debug!("Generating prompt successful");
                    Ok(res)
                } else {
                    warn!(
                        "Generating prompt failed with status code {}: {}",
                        res.status(),
                        res.text().await?
                    );
                    Err(anyhow::anyhow!("Generating prompt failed"))
                }
            }
            Err(e) => {
                error!("Generating prompt failed with error: {}", e);
                Err(anyhow::anyhow!("Generating prompt failed"))
            }
        }
    }

    pub async fn stream_chat_completions(&self, model: ModelSettings, prompt: String) -> Result<EventSource> {
        debug!("Streaming chat completions...");

        let request_body = json!({
            "model": model.name,
            "messages": [
                { "role": "user", "content": prompt }
            ],
            "top_p": model.top_p,
            "top_k": model.top_k,
            "temperature": model.temperature,
            "presence_penalty": model.presence_penalty,
            "frequency_penalty": model.frequency_penalty,
            "repeat_penalty": model.repeat_penalty,
            "stream": true,
        });
        
        let request = self.http_client
            .post(format!("{}/chat/completions", self.api_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request_body);

        let es = EventSource::new(request)?;
        
        Ok(es)
    }
}
