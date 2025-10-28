mod app;
mod cli;
mod commands;
mod domain;
mod infrastructure;
mod utils;

use crate::app::config::Config;
use crate::utils::logging::init_tracing;

use anyhow::Result;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::from_env()?;
    let _guard = init_tracing(config.clone())?;
    
    let mut app = app::runner::OpenCoder::new(config)?;
    app.run().await
}
