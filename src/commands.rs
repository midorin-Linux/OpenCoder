use crate::lm::client::Client;

use anyhow::{Context, Result};
use tracing::{debug, error, info, instrument, warn};

pub fn exit(_client: &mut Client, _input: &str) -> Result<String> {
    info!("Exiting...");
    std::process::exit(0);
}

pub fn help(client: &mut Client, _input: &str) -> Result<String> {
    let help_text = client.commands.iter()
        .map(|cmd| format!("  {} - {}", cmd.name, cmd.description))
        .collect::<Vec<String>>()
        .join("\n");

    let response = format!("Available commands:\n{}", help_text);
    Ok(response)
}