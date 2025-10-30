use crate::app::runner::OpenCoder;
use crate::commands::command::Command;
use crate::infrastructure::lm::client::Client;
use std::{collections::HashMap, pin::Pin};

use anyhow::{anyhow, Result};

type Handler = Box<dyn for<'a> Fn(&'a mut OpenCoder, &'a str) -> Pin<Box<dyn Future<Output = Result<String>> + 'a>> + Send + Sync>;

pub struct CommandRegistry {
    commands: HashMap<String, Command>,
    handlers: HashMap<String, Handler>
}

impl CommandRegistry {
    pub fn new() -> Result<Self> {
        Ok(Self{commands: HashMap::new(), handlers: HashMap::new()})
    }

    pub fn register(&mut self, command: Command, handler: Handler) {
        let name = command.name.clone();
        self.commands.insert(name.clone(), command);
        self.handlers.insert(name, handler);
    }

    pub fn get_all_commands(&self) -> Vec<&Command> {
        self.commands.values().collect()
    }

    pub async fn execute(&self, name: &str, args: &str, open_coder: &mut OpenCoder) -> Result<String> {
        if let Some(handler) = self.handlers.get(name) {
            match handler(open_coder, args).await {
                Ok(result) => Ok(result),
                Err(err) => Err(err)
            }
        } else {
            Err(anyhow!("Unknown command. Type /help for a list of commands."))
        }
    }
}