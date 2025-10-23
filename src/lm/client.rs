use crate::config::Config;
use crate::lm::{http, http::Message as LmMessage};

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

#[derive(Debug)]
struct SessionData {
    model: String,
    history: History,
}

pub struct Client {
    http_client: http::Client,
    pub commands: Vec<Command>,
    default_model: String,
    sessions: HashMap<String, SessionData>,
}

impl Client {
    pub fn new(config: Config, commands: Vec<Command>) -> Result<Self> {
        debug!("Initializing core...");

        let default_model = config.model.clone();
        let http_client = http::Client::new(&config)?;

        debug!("Core initialized!");

        Ok(Self {
            http_client,
            commands,
            default_model,
            sessions: HashMap::new(),
        })
    }

    pub async fn get_models(&mut self) -> Result<Value> {
        let json = self.http_client.get_models().await?;
        Ok(json)
    }

    pub fn set_session_model(&mut self, session_id: &str, model: &str) -> Result<()> {
        let session = self.sessions.entry(session_id.to_string()).or_insert_with(|| SessionData {
            model: self.default_model.clone(),
            history: Vec::new(),
        });
        session.model = model.to_string();
        debug!(session_id, model, "Session model updated");
        Ok(())
    }

    pub async fn handle_chat(&mut self, session_id: &str, input: &str) -> Result<LmMessage> {
        let session = self.sessions.entry(session_id.to_string()).or_insert_with(|| SessionData {
            model: self.default_model.clone(),
            history: Vec::new(),
        });

        session.history.push(Message {
            role: "user".to_string(),
            content: input.to_string(),
        });

        let messages: Vec<Value> = session.history
            .iter()
            .map(|m| {
                json!({
                    "role": m.role,
                    "content": m.content
                })
            })
            .collect();

        let output = self.http_client.chat_completions(&session.model, messages).await?;

        session.history.push(Message {
            role: "assistant".to_string(),
            content: output.message.clone(),
        });

        Ok(LmMessage {
            model: output.model,
            message: output.message,
        })
    }
}
