mod commands;
mod config;
mod lm;

use crate::config::Config;
use crate::lm::client::{Client, Command};

use anyhow::{Context, Result};
use dialoguer::{theme::SimpleTheme, Select, Input, console::Term};
use cfonts::{say, Align, BgColors, Colors, Env, Fonts, Options};
use regex::Regex;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, future::Future, pin::Pin};
use tokio::signal;
use tracing::{debug, error, info, instrument, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

static RE_COMMAND: Lazy<Regex> = Lazy::new(|| Regex::new(r"^/.*").unwrap());

type Handler = Box<dyn for<'a> Fn(&'a mut Client, &'a str) -> Pin<Box<dyn Future<Output = Result<String>> + 'a>> + Send + Sync>;

struct OpenCoder {
    client: Client,
    handlers: HashMap<String, Handler>
}

impl OpenCoder {
    fn new(config: Config) -> Result<Self> {
        let commands = vec![
            Command{ name: "/exit".to_string(), description: "Exit the application".to_string()},
            Command{ name: "/help".to_string(), description: "Display help information".to_string()},
            Command{ name: "/models".to_string(), description: "Display available models".to_string()},
            Command{ name: "/set".to_string(), description: "Settings".to_string()},
        ];

        let mut app = Self {
            client: Client::new(config, commands)?,
            handlers: HashMap::new()
        };

        app.handlers.insert("/exit".to_string(), Box::new(|client, input| Box::pin(async move { commands::exit(client, input) })));
        app.handlers.insert("/help".to_string(), Box::new(|client, input| Box::pin(async move { commands::help(client, input) })));
        app.handlers.insert("/models".to_string(), Box::new(|client, input| Box::pin(commands::models(client, input))));
        app.handlers.insert("/set".to_string(), Box::new(|client, input| Box::pin(commands::set(client, input) )));

        Ok(app)
    }

    async fn run(&mut self) -> Result<()> {
        loop {
            let input: String = Input::with_theme(&SimpleTheme)
                .with_prompt("Enter your message")
                .interact_text()
                .context("Failed to read input")?;

            if RE_COMMAND.is_match(&input) {
                let parts: Vec<&str> = input.splitn(2, ' ').collect();
                let command_name = parts[0];
                let args = parts.get(1).unwrap_or(&"");

                if let Some(handler) = self.handlers.get(command_name) {
                    match handler(&mut self.client, args).await {
                        Ok(response) => println!("\n{}\n", response),
                        Err(e) => error!("Command '{}' failed: {}", command_name, e),
                    }
                } else {
                    warn!("Unknown command. Type /help for a list of commands.");
                }
            } else {
                match self.client.handle_chat("1", &input).await {
                    Ok(response) => println!("\n{:?}\n{}\n", response.model, response.message),
                    Err(e) => error!("Chat failed: {}", e),
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::from_env().context("Failed to load configuration")?;

    let mut app = OpenCoder::new(config.clone()).context("Application Error")?;

    say(Options {
        text: String::from("Open Coder"),
        font: Fonts::FontBlock,
        colors: vec![Colors::System],
        background: BgColors::Transparent,
        align: Align::Left,
        letter_spacing: 1,
        line_height: 1,
        spaceless: false,
        max_length: 0,
        gradient: Vec::new(),
        independent_gradient: false,
        transition_gradient: false,
        raw_mode: false,
        env: Env::Cli,
        ..Options::default()
    });

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::new(config.rust_log),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    app.run().await
}
