use crate::lm::client::Client;

use anyhow::{Context, Result};
use dialoguer::{theme::SimpleTheme, Select, Input, console::Term};
use tracing::{debug, error, info, instrument, warn};

pub fn exit(_client: &mut Client, _arg: &str) -> Result<String> {
    info!("Exiting...");
    std::process::exit(0);
}

pub fn help(client: &mut Client, _arg: &str) -> Result<String> {
    let help_text = client.commands.iter()
        .map(|cmd| format!("  {} - {}", cmd.name, cmd.description))
        .collect::<Vec<String>>()
        .join("\n");

    Ok(format!("Available commands:\n{}", help_text))
}

pub async fn models(client: &mut Client, _arg: &str) -> Result<String> {
    let models_json = client.get_models().await?;
    let models_list = models_json["data"].as_array().context("Failed to extract models list")?;
    let models_ids: Vec<String> = models_list.iter()
        .filter_map(|model_obj| model_obj["id"].as_str())
        .map(|id| format!("  {}", id.to_string()))
        .collect();

    if models_ids.is_empty() {
        warn!("No models available");
        return Ok("No models available".to_string());
    }

    Ok(format!("Available models:\n{}", models_ids.join("\n")))
}

pub async fn set(client: &mut Client, arg: &str) -> Result<String> {
    match arg {
        "" => Ok("Argument cannot be empty".to_string()),
        "model" => {
            let models_json = client.get_models().await?;
            let models_list = models_json["data"].as_array().context("Failed to extract models list")?;
            let models_ids: Vec<String> = models_list.iter()
                .filter_map(|model_obj| model_obj["id"].as_str())
                .map(|id| id.to_string())
                .collect();

            if models_ids.is_empty() {
                warn!("No models available");
                return Ok("No models available".to_string());
            }

            let selection = Select::new().items(&models_ids).default(0).interact()?;
            let selected_model_name = models_ids[selection].clone();

            client.set_session_model("1", &selected_model_name).context(format!("Failed to set session model to '{}'", selected_model_name))?;

            Ok(format!("Model for the current session set to: {}", selected_model_name))
        },
        _ => { Ok("Invalid argument".to_string()) }
    }
}