// ToDo: 実行ロジック実装
use crate::app::config::Config;
use crate::cli::{output::OutputHandler, prompt::Prompt};
use crate::infrastructure::lm::client::Client;
use std::{collections::HashMap, pin::Pin};

use anyhow::{Context, Result};
use dialoguer::{
    console::{Style, StyledObject},
    Input,
    Select,
    theme::ColorfulTheme,
};
use indicatif::{ProgressBar, ProgressStyle};
use owo_colors::OwoColorize;
use tracing::{debug, error, info, instrument, warn};

type Handler = Box<dyn for<'a> Fn(&'a mut Client, &'a str) -> Pin<Box<dyn Future<Output = Result<String>> + 'a>> + Send + Sync>;

pub struct OpenCoder {
    client: Client,
    handlers: HashMap<String, Handler>,
    output: OutputHandler,
    prompt: Prompt
}

impl OpenCoder {
    pub fn new(config: Config) -> Result<Self> {
        let client = Client::new(config.clone())
            .context("Failed to initialize LM client")?;

        let mut app = Self {
            client,
            handlers: HashMap::new(),
            output: OutputHandler::new()?,
            prompt: Prompt::new()?
        };

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
        let parts: Vec<&str> = input.splitn(2, ' ').collect();
        let command_name = parts[0];
        let args = parts.get(1).unwrap_or(&"");

        if let Some(handler) = self.handlers.get(command_name) {
            match handler(&mut self.client, args).await {
                Ok(response) => self.output.print_command_response(&response)?,
                Err(e) => self.output.print_error(&format!("Command '{}' failed: {}", command_name, e))?,
            }
        } else {
            self.output.print_warning("Unknown command. Type /help for a list of commands.")?;
        }

        Ok(())
    }

    async fn handle_chat(&mut self, input: &str) -> Result<()> {
        println!();

        let spinner = ProgressBar::new_spinner();
        spinner.set_style(
            ProgressStyle::default_spinner()
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
                .template("{spinner:.blue} {msg}")?,
        );
        spinner.set_message("Generating response...");
        spinner.enable_steady_tick(std::time::Duration::from_millis(120));

        match self.client.chat_completions(input.to_string()).await {
            Ok(res) => {
                let formatted_res = self.output.format_model_response(res).await?;
                spinner.finish_and_clear();

                println!("{} Response generated! - {:?}", "✓".green(), formatted_res.model);
                println!("\n{}\n", formatted_res.message);

                Ok(())
            }
            Err(e) => {
                spinner.finish_and_clear();
                self.output.print_error(&format!("Failed to generate response: {}", e))?;
                Err(anyhow::anyhow!("Failed to generate response"))
            }
        }
    }
}