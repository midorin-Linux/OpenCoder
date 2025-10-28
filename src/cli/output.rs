// ToDo: 出力フォーマット実装
use anyhow::{Context, Result};
use cfonts::{say, Align, BgColors, Colors, Env, Fonts, Options};
use dialoguer::console::Term;
use owo_colors::OwoColorize;
use reqwest::Response;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug)]
pub struct Message { pub model: String, pub message: String, }

pub struct OutputHandler {}

impl OutputHandler {
    pub fn new() -> Result<Self> {
        Ok(Self{})
    }

    pub async fn format_model_response(&self, input: Response) -> Result<Message> {
        let response_text = input.text().await?;
        let response_json: Value = serde_json::from_str(&response_text)
            .context("Failed to parse response JSON")?;

        let output_model = response_json["model"]
            .as_str()
            .context("Failed to extract model from response")?
            .to_string();

        let output_message = response_json["choices"][0]["message"]["content"]
            .as_str()
            .context("Failed to extract content from response")?
            .to_string();

        Ok(Message{ model: output_model, message: output_message })
    }

    pub async fn format_model_stream_response(&self, input: String) -> Result<(bool, String)> {
        let response_text = input.as_str();
        let response_json: Value = serde_json::from_str(&response_text)
            .context("Failed to parse response JSON")?;

        let output_message = response_json["choices"][0]["delta"]["content"]
            .as_str()
            .unwrap_or("")
            .to_string();

        let is_finish = response_json["choices"][0]["finish_reason"]
            .as_str()
            .map(|s| s == "stop".to_string())
            .unwrap_or(false);

        Ok((is_finish, output_message))
    }

    pub fn print_command_response(&self, input: &str) -> Result<()> {
        println!("{}", input);

        Ok(())
    }

    // ToDo: 独自の出力方法にする
    pub fn print_warning(&self, input: &str) -> Result<()> {
        println!("{}", input);

        Ok(())
    }

    // ToDo: 独自の出力方法にする
    pub fn print_error(&self, input: &str) -> Result<()> {
        println!("{}", input);

        Ok(())
    }

    pub fn echo_input(&self, input: &String) -> Result<()> {
        let term = Term::stdout();
        term.move_cursor_up(1)?;
        term.clear_line()?;
        term.write_line(&format!("{}", format!("> {}", input).bright_black()))?;

        Ok(())
    }

    pub fn show_banner(&self) {
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
    }

    pub fn show_welcome_message(&self) {
        println!("We trust you have knowledge of language models.");
        println!("It usually boils down to three things:");
        println!();
        println!("  #1) Respect the privacy of others.");
        println!("  #2) Language models are never right.");
        println!("  #3) Non-coding means great responsibility.");
        println!();
    }
}