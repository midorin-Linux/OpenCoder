use std::collections::HashMap;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use tracing::{debug, error, info, instrument, warn};

#[derive(Serialize, Deserialize, Debug)]
pub enum Role {
    System,
    User,
    Assistant,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message { role: String, content: String }

pub struct HistoryStore {
    history: Vec<Message>
}

impl HistoryStore {
    pub fn new(system_prompt: &str) -> Result<Self> {
        Ok(Self {
            history: vec![Message { role: "system".to_string(), content: system_prompt.to_string() }]
        })
    }

    pub fn history(&mut self) -> Vec<Value> {
        self.history.iter().map(|m| json!({ "role": m.role, "content": m.content })).collect()
    }

    pub fn add_history(&mut self, role: Role, content: &str) -> Result<()> {
        let role_str = match role {
            Role::System => "system",
            Role::User => "user",
            Role::Assistant => "assistant",
        };

        self.history.push(Message { role: role_str.to_string(), content: content.to_string() });

        Ok(())
    }
}