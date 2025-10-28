use crate::app::config::Config;
use std::fs::File;

use anyhow::{Context, Result};
use tracing_appender::{non_blocking, non_blocking::WorkerGuard};
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_tracing(config: Config) -> Result<WorkerGuard> {
    let file = File::create("tracing.log").context("failed to create tracing.log")?;
    let (non_blocking, guard) = non_blocking(file);

    let env_filter = EnvFilter::new(config.rust_log);

    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_env_filter(env_filter)
        .init();

    Ok(guard)
}

// pub fn init_tracing(config: Config) -> Result<()> {
//     tracing_subscriber::registry()
//         .with(
//             EnvFilter::new(config.rust_log),
//         )
//         .with(tracing_subscriber::fmt::layer())
//         .init();
//
//     Ok(())
// }
