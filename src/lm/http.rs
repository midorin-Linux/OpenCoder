use crate::config::Config;

use anyhow::{Context, Result};
use reqwest::{Client as HttpClient, Error, Response};
use serde_json::{Value, json};
use tracing::{debug, info, warn, error, instrument};

pub struct Client {
    api_key: String,
    http_client: HttpClient,
    openai_api_url: String,
}

impl Client {
    pub fn new(config: &Config) -> Result<Self> {
        debug!("Initializing http client...");

        let api_key = config.api_key.clone();

        let http_client = HttpClient::builder()
            .timeout(std::time::Duration::from_secs(config.request_timeout_secs))
            .build()
            .context("Failed to crate http client")?;

        let openai_api_url = config.openai_api_url.clone();

        debug!("Http client initialized");

        Ok(Self {
            api_key,
            http_client,
            openai_api_url,
        })
    }

    pub async fn get_models(&self) -> Result<()> {
        debug!("Checking health...");

        let health_check_url = format!("{}/models", self.openai_api_url);

        match self.http_client.get(&health_check_url).header("Content-Type", "application/json").send().await {
            Ok(res) => {
                if res.status().is_success() {
                    debug!("Health check successful");
                    Ok(())
                } else {
                    warn!("Health checking failed with status code {}", res.status());
                    Err(anyhow::anyhow!("Health checking failed"))
                }
            }
            Err(e) => {
                error!("Health checking failed with error: {}", e);
                Err(anyhow::anyhow!("Health checking failed"))
            }
        }
    }

    pub async fn chat_completions(&self, messages: Vec<Value>) -> Result<String> {
        let request_body = json!({
            "model": "qwen3-30b-a3b-instruct-2507",
            "messages": messages,
            "temperature": 0.7
        });

        debug!("Request body: {}", request_body.to_string());

        match self.http_client
            .post(format!("{}/chat/completions", self.openai_api_url))
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .body(request_body.to_string())
            .send()
            .await
        {
            Ok(res) => {
                if res.status().is_success() {
                    debug!("Generate successful");

                    let response_text = res.text().await?;
                    let response_json: Value = serde_json::from_str(&response_text)
                        .context("Failed to parse response JSON")?;

                    debug!("Response: {}", response_json.to_string());

                    let output = response_json["choices"][0]["message"]["content"]
                        .as_str()
                        .context("Failed to extract content from response")?
                        .to_string();

                    Ok(output)
                } else {
                    let status = res.status();
                    let error_text = res.text().await.unwrap_or_default();
                    warn!("Generate failed with status code {}: {}", status, error_text);
                    Err(anyhow::anyhow!("Generate failed with status {}", status))
                }
            }
            Err(e) => {
                error!("Generate failed with error: {}", e);
                Err(anyhow::anyhow!("Generate failed: {}", e))
            }
        }
    }
}