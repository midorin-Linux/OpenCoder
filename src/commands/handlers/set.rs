use crate::app::runner::OpenCoder;

use anyhow::{Context, Result};
use dialoguer::{Input, Select};
use tracing::warn;

pub async fn set(open_coder: &mut OpenCoder, arg: &str) -> Result<String> {
    match arg {
        "model" => set_model(open_coder, arg).await,
        "top_p" => set_top_p(open_coder, arg),
        "top_k" => set_top_k(open_coder, arg),
        "temperature" => set_temperature(open_coder, arg),
        "pre_p" => set_presence_penalty(open_coder, arg),
        "fre_p" => set_frequency_penalty(open_coder, arg),
        "rep_p" => set_repeat_penalty(open_coder, arg),
        "help" => {
            Ok(
                "Usage: /set <model|top_p|top_k|temperature|pre_p|fre_p|rep_p>\n  model: Set model\n  top_p: Set top_p\n  top_k: Set top_k\n  temperature: Set temperature\n  pre_p: Set presence_penalty\n  fre_p: Set frequency_penalty\n  rep_p: Set repeat_penalty".to_string()
            )
        }
        _ => Ok("Invalid argument".to_string())
    }
}

async fn set_model(open_coder: &mut OpenCoder, arg: &str) -> Result<String> {
    let models_json = open_coder.client.get_model_list().await?;
    let models_list = models_json["data"].as_array().unwrap();
    let models_ids: Vec<String> = models_list.iter()
        .filter_map(|model_obj| model_obj["id"].as_str())
        .map(|id| id.to_string())
        .collect();

    if models_ids.is_empty() {
        warn!("No models available");
        return Ok("No models available".to_string());
    }

    println!();
    let selection = Select::new().items(&models_ids).default(0).interact()?;
    let selected_model_name = models_ids[selection].clone();

    open_coder.model.name = selected_model_name.clone();

    Ok(format!("Set model to {}", selected_model_name))
}

fn set_top_p(open_coder: &mut OpenCoder, arg: &str) -> Result<String> {
    println!();
    let input: f64 = Input::with_theme(&open_coder.theme)
        .with_prompt(format!("top_p(Current: {}):", open_coder.model.top_p))
        .interact_text()
        .context("Failed to read input. (Input should be f64)")?;

    open_coder.model.top_p = input;

    Ok(format!("Set top_p to {}", open_coder.model.top_p))
}

fn set_top_k(open_coder: &mut OpenCoder, arg: &str) -> Result<String> {
    println!();
    let input: u64 = Input::with_theme(&open_coder.theme)
        .with_prompt(format!("top_k(Current: {}):", open_coder.model.top_k))
        .interact_text()
        .context("Failed to read input. (Input should be u64)")?;

    open_coder.model.top_k = input;

    Ok(format!("Set top_k to {}", open_coder.model.top_k))
}

fn set_temperature(open_coder: &mut OpenCoder, arg: &str) -> Result<String> {
    println!();
    let input: f64 = Input::with_theme(&open_coder.theme)
        .with_prompt(format!("temperature(Current: {}):", open_coder.model.temperature))
        .interact_text()
        .context("Failed to read input. (Input should be f64)")?;

    open_coder.model.temperature = input;

    Ok(format!("Set temperature to {}", open_coder.model.temperature))
}

fn set_presence_penalty(open_coder: &mut OpenCoder, arg: &str) -> Result<String> {
    println!();
    let input: f64 = Input::with_theme(&open_coder.theme)
        .with_prompt(format!("presence_penalty(Current: {}):", open_coder.model.presence_penalty))
        .interact_text()
        .context("Failed to read input. (Input should be f64)")?;

    open_coder.model.presence_penalty = input;

    Ok(format!("Set presence_penalty to {}", open_coder.model.presence_penalty))
}

fn set_frequency_penalty(open_coder: &mut OpenCoder, arg: &str) -> Result<String> {
    println!();
    let input: f64 = Input::with_theme(&open_coder.theme)
        .with_prompt(format!("frequency_penalty(Current: {}):", open_coder.model.frequency_penalty))
        .interact_text()
        .context("Failed to read input. (Input should be f64)")?;

    open_coder.model.frequency_penalty = input;

    Ok(format!("Set frequency_penalty to {}", open_coder.model.frequency_penalty))
}

fn set_repeat_penalty(open_coder: &mut OpenCoder, arg: &str) -> Result<String> {
    println!();
    let input: f64 = Input::with_theme(&open_coder.theme)
        .with_prompt(format!("repeat_penalty(Current: {}):", open_coder.model.repeat_penalty))
        .interact_text()
        .context("Failed to read input. (Input should be f64)")?;

    open_coder.model.repeat_penalty = input;

    Ok(format!("Set repeat_penalty to {}", open_coder.model.repeat_penalty))
}