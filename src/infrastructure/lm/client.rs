use crate::app::config::Config;

use anyhow::{Context, Result};
use reqwest::{Client as HttpClient, Error, Response};
use serde_json::json;
use tracing::{debug, error, info, instrument, warn};

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

    pub async fn chat_completions(&self, model: String, prompt: String) -> Result<Response> {
        debug!("Posting chat completions...");

        let request_body = json!({
            "model": model,
            "messages": [
                { "role": "user", "content": prompt }
            ],
            "temperature": 0.7
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
}
