use crate::app::config::Config;

use anyhow::{Context, Result};
use futures::StreamExt;
use reqwest::{Client as HttpClient, Error, Response};
use reqwest_eventsource::{Event, EventSource};
use serde_json::{json, Value};
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

    pub async fn get_model_list(&self) -> Result<Value> {
        debug!("Getting model list...");

        let res = self
            .http_client
            .get(format!("{}/models", self.api_url))
            .header("Content-Type", "application/json")
            .send()
            .await
            .context("Failed to get model list")?;

        if res.status().is_success() {
            debug!("Models list getting successful");

            let response_json: Value = res.json().await.context("Failed to parse response JSON")?;

            Ok(response_json)
        } else {
            let status = res.status();
            let text = res.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            warn!("Models getting failed with status code {}: {}", status, text);
            Err(anyhow::anyhow!("Failed to get model list with status: {}", status))
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

        let res = self
            .http_client
            .post(format!("{}/chat/completions", self.api_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request_body)
            .send()
            .await
            .context("Failed to post chat completions")?;

        if res.status().is_success() {
            debug!("Generating prompt successful");
            Ok(res)
        } else {
            let status = res.status();
            let text = res.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            warn!(
                "Generating prompt failed with status code {}: {}",
                status, text
            );
            Err(anyhow::anyhow!("Failed to generate prompt with status: {}", status))
        }
    }

    pub async fn stream_chat_completions(
        &self,
        model: ModelSettings,
        messages: Vec<Value>,
    ) -> Result<EventSource> {
        debug!("Streaming chat completions...");

        let request_body = json!({
            "model": model.name,
            "messages": messages,
            "top_p": model.top_p,
            "top_k": model.top_k,
            "temperature": model.temperature,
            "presence_penalty": model.presence_penalty,
            "frequency_penalty": model.frequency_penalty,
            "repeat_penalty": model.repeat_penalty,
            "stream": true,
        });

        let request = self
            .http_client
            .post(format!("{}/chat/completions", self.api_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request_body);

        EventSource::new(request).context("Failed to create event source for streaming")
    }
}