use crate::app::config::Config;
use crate::cli::{output::OutputHandler, prompt::Prompt};
use crate::commands::{
    command::Command,
    handlers::{exit::exit, set::set},
    parser::parse_input,
    registry::CommandRegistry,
};
use crate::infrastructure::{
    lm::client::{Client, ModelSettings},
    storage::history_store::{HistoryStore, Message, Role},
};
use std::io::{self, Write};

use anyhow::{Context, Result};
use dialoguer::{console::Style, theme::ColorfulTheme};
use futures::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use owo_colors::OwoColorize;
use reqwest_eventsource::Event;
use tracing::{debug, error, info, instrument, warn};

pub struct OpenCoder {
    pub client: Client,
    output: OutputHandler,
    prompt: Prompt,
    pub model: ModelSettings,
    history_store: HistoryStore,
    pub theme: ColorfulTheme
}

impl OpenCoder {
    pub fn new(config: Config) -> Result<Self> {
        let client = Client::new(config.clone()).context("Failed to initialize LM client")?;

        let model = ModelSettings {
            name: "qwen3-30b-a3b-instruct-2507".to_string(),
            top_p: 0.95,
            top_k: 40,
            temperature: 0.7,
            presence_penalty: 0.0,
            frequency_penalty: 0.0,
            repeat_penalty: 1.0,
        };

        let theme = ColorfulTheme {
            prompt_prefix: Style::new().apply_to("".to_string()),
            prompt_suffix: Style::new().apply_to("".to_string()),
            success_prefix: Style::new().apply_to("".to_string()),
            success_suffix: Style::new().apply_to("".to_string()),
            ..ColorfulTheme::default()
        };

        let app = Self {
            client,
            output: OutputHandler::new()?,
            prompt: Prompt::new()?,
            model,
            history_store: HistoryStore::new("You are a helpful assistant.")?,
            theme
        };

        Ok(app)
    }

    pub async fn run(&mut self) -> Result<()> {
        self.output.show_banner();
        self.output.show_welcome_message();

        let mut registry = CommandRegistry::new()?;
        let commands = vec![
            Command {
                name: "/exit".to_string(),
                description: "Exit from application.".to_string(),
            },
            Command {
                name: "/set".to_string(),
                description: "Set model settings.".to_string(),
            },
        ];
        registry.register(
            commands[0].clone(),
            Box::new(|open_coder, args| Box::pin(async move { exit(open_coder, args) })),
        );
        registry.register(
            commands[1].clone(),
            Box::new(|open_coder, args| Box::pin(set(open_coder, args))),
        );

        loop {
            let input = self.prompt.read_input(&self.theme)?;

            self.output.echo_input(&input)?;

            if self.is_command(&input) {
                self.handle_command(&input, &registry).await?;
            } else {
                self.handle_chat(&input).await?;
            }
        }
    }

    fn is_command(&self, input: &str) -> bool {
        input.starts_with("/")
    }

    async fn handle_command(&mut self, input: &str, registry: &CommandRegistry) -> Result<()> {
        let parse = parse_input(input);

        if let Some((command, args)) = parse {
            match registry.execute(command.as_str(), args, self).await {
                Ok(command_response) => {
                    self.output.print_command_response(&command_response)?;
                }
                Err(e) => {
                    self.output
                        .print_error(&format!("Failed to execute command: {}", e))?
                }
            }
        }

        Ok(())
    }

    async fn handle_chat(&mut self, input: &str) -> Result<()> {
        // 処理中であることを示すスピナーを表示する
        println!();
        let spinner = ProgressBar::new_spinner();
        spinner.set_style(
            ProgressStyle::default_spinner()
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
                .template("{spinner}")?,
        );
        spinner.enable_steady_tick(std::time::Duration::from_millis(120));

        // historyにユーザーの入力を追加
        self.history_store.add_history(Role::User, input)?;
        let messages = self.history_store.history();

        // 完全なレスポンスを保存するための変数
        let mut full_response = String::new();

        // ストリーミング処理
        let mut es = self
            .client
            .stream_chat_completions(self.model.clone(), messages)
            .await?;
        while let Some(event) = es.next().await {
            match event {
                Ok(Event::Open) => {}
                Ok(Event::Message(message)) => {
                    if !spinner.is_finished() {
                        spinner.finish_and_clear();
                        println!(
                            "{} Response generated! - {:?}",
                            "✓".green(),
                            self.model.name
                        );
                    }

                    let formatted_message = self.output.format_model_stream_response(message.data).await?;
                    match formatted_message.0 {
                        true => {
                            es.close();
                            println!("\n");
                        }
                        false => {
                            full_response.push_str(&formatted_message.1);

                            print!("{}", formatted_message.1);
                            io::stdout().flush()?;
                        }
                    }
                }
                Err(err) => {
                    if !spinner.is_finished() {
                        spinner.finish_and_clear();
                        println!("{} Failed to generate response", "✗".red());
                    }
                    println!("Error: {}\n", err);
                    es.close();
                }
            }
        }

        if !full_response.is_empty() {
            self.history_store
                .add_history(Role::Assistant, &full_response)?;
        }

        Ok(())
    }
}
