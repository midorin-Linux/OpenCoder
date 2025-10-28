use crate::infrastructure::lm::client::Client;

use tracing::info;

pub fn exit(_client: &mut Client, _arg: &str) -> anyhow::Result<String> {
    info!("Exiting...");
    std::process::exit(0);
}