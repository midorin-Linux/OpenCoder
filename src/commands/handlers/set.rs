use crate::app::runner::OpenCoder;

use anyhow::Result;
use dialoguer::Select;
use serde_json::Value;
use tracing::warn;

pub async fn set(open_coder: &mut OpenCoder, arg: &str) -> Result<String> {
    match arg {
        "model" => set_model(open_coder, arg).await,
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

    let selection = Select::new().items(&models_ids).default(0).interact()?;
    let selected_model_name = models_ids[selection].clone();

    open_coder.model.name = selected_model_name.clone();

    Ok(format!("Set model to {}", selected_model_name))
}