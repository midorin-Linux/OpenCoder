use crate::app::config::Config;
use crate::cli::{output::OutputHandler, prompt::Prompt};
use crate::commands::handlers::exit::exit;
use crate::commands::command::Command;
use crate::commands::registry::CommandRegistry;
use crate::commands::parser::parse_input;
use crate::infrastructure::lm::client::Client;
use std::sync::{Arc, Mutex};

use anyhow::{Context, Result};
use dialoguer::{
    Input, Select,
    console::{Style, StyledObject},
    theme::ColorfulTheme,
};
use futures::Future;
use indicatif::{ProgressBar, ProgressStyle};
use owo_colors::OwoColorize;
use tracing::{debug, error, info, instrument, warn};

pub struct OpenCoder {
    client: Client,
    output: OutputHandler,
    prompt: Prompt,
    registry: CommandRegistry,
    pub commands: Vec<Command>
}

impl OpenCoder {
    pub fn new(config: Config) -> Result<Self> {
        let client = Client::new(config.clone()).context("Failed to initialize LM client")?;

        let registry = CommandRegistry::new()?;
        let commands = vec![
            Command{ name: "/exit".to_string(), description: "Exit from application.".to_string() }
        ];

        let mut app = Self {
            client,
            output: OutputHandler::new()?,
            prompt: Prompt::new()?,
            registry,
            commands
        };

        app.registry.register(app.commands[0].clone(), Box::new(|client, args| Box::pin(async move { exit(client, args)})));

        Ok(app)
    }

    pub async fn run(&mut self) -> Result<()> {
        self.output.show_banner();
        self.output.show_welcome_message();

        loop {
            let theme = ColorfulTheme {
                prompt_prefix: Style::new().apply_to("".to_string()),
                prompt_suffix: Style::new().apply_to("".to_string()),
                success_prefix: Style::new().apply_to("".to_string()),
                success_suffix: Style::new().apply_to("".to_string()),
                ..ColorfulTheme::default()
            };

            let input = self.prompt.read_input(&theme)?;

            self.output.echo_input(&input)?;

            if self.is_command(&input) {
                self.handle_command(&input).await?;
            } else {
                self.handle_chat(&input).await?;
            }
        }
    }

    fn is_command(&self, input: &str) -> bool {
        input.starts_with("/")
    }

    async fn handle_command(&mut self, input: &str) -> Result<()> {
        let parse = parse_input(input);

        if let Some((command, args)) = parse {
            self.registry.execute(command.as_str(), args, &mut self.client).await?;
        }

        Ok(())
    }

    async fn handle_chat(&mut self, input: &str) -> Result<()> {
        println!();

        let spinner = ProgressBar::new_spinner();
        spinner.set_style(
            ProgressStyle::default_spinner()
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
                .template("{spinner} {msg}")?,
        );
        spinner.set_message("Generating response...");
        spinner.enable_steady_tick(std::time::Duration::from_millis(120));

        match self.client.chat_completions(input.to_string()).await {
            Ok(res) => {
                let formatted_res = self.output.format_model_response(res).await?;
                spinner.finish_and_clear();

                println!(
                    "{} Response generated! - {:?}",
                    "✓".green(),
                    formatted_res.model
                );
                println!("\n{}\n", formatted_res.message);

                Ok(())
            }
            Err(e) => {
                spinner.finish_and_clear();
                self.output
                    .print_error(&format!("Failed to generate response: {}", e))?;
                Err(anyhow::anyhow!("Failed to generate response"))
            }
        }
    }
}
