mod config;
mod lm;
mod commands;

use crate::config::Config;
use crate::lm::client::{Client, Command};
use std::{
    collections::HashMap,
    fs::File,
    future::Future,
    pin::Pin,
    time::Duration
};

use anyhow::{Context, Result};
use cfonts::{say, Align, BgColors, Colors, Env, Fonts, Options};
use dialoguer::{
    console::{Term, Style, StyledObject},
    Input,
    Select,
    theme::ColorfulTheme,
};
use regex::Regex;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use indicatif::{ProgressBar, ProgressStyle};
use owo_colors::OwoColorize;
use tracing::{debug, error, info, instrument, warn};
use tracing_appender::{non_blocking, non_blocking::WorkerGuard};
use tracing_subscriber::EnvFilter;

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
            let theme = ColorfulTheme {
                prompt_prefix: Style::new().apply_to("".to_string()),
                prompt_suffix: Style::new().apply_to("".to_string()),
                success_prefix: Style::new().apply_to("".to_string()),
                success_suffix: Style::new().apply_to("".to_string()),
                ..ColorfulTheme::default()
            };

            let input: String = Input::with_theme(&theme)
                .with_prompt(">")
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
    let _guard = init_tracing(config.clone());
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

    println!("We trust you have knowledge of language models.\nIt usually boils down to three things:\n\n   #1) Respect the privacy of others.\n   #2) Language models are never right.\n   #3) Non-coding means great responsibility.\n");

    app.run().await
}

fn init_tracing(config: Config) -> Result<WorkerGuard> {
    let file = File::create("tracing.log").context("failed to create tracing.log")?;
    let (non_blocking, guard) = non_blocking(file);

    let env_filter = EnvFilter::new(config.rust_log);

    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_env_filter(env_filter)
        .init();

    Ok(guard)
}