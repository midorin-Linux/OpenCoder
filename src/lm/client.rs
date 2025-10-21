use crate::config::Config;
use crate::lm::http;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::HashMap;
use tracing::{debug, error, info, instrument, warn};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Command {
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Message { role: String, content: String }

type History = Vec<Message>;

pub struct Client {
    http_client: http::Client,
    pub commands: Vec<Command>,
    sessions: HashMap<String, History>,
}

impl Client {
    pub fn new(config: Config, commands: Vec<Command>) -> Result<Self> {
        debug!("Initializing core...");
        let http_client = http::Client::new(&config)?;

        debug!("Core initialized!");

        Ok(Self {
            http_client,
            commands,
            sessions: HashMap::new(),
        })
    }

    pub async fn handle_chat(&mut self, session_id: &str, input: &str) -> Result<String> {
        let history = self.sessions.entry(session_id.to_string()).or_insert_with(|| {
            vec![Message {
                role: "system".to_string(),
                content: "You are a helpful assistant.".to_string(),
            }]
        });

        history.push(Message {
            role: "user".to_string(),
            content: input.to_string(),
        });

        let messages: Vec<Value> = history
            .iter()
            .map(|m| {
                json!({
                    "role": m.role,
                    "content": m.content
                })
            })
            .collect();

        let output = self.http_client.chat_completions(messages).await?;

        history.push(Message {
            role: "assistant".to_string(),
            content: output.clone(),
        });

        Ok(output)
    }
}
