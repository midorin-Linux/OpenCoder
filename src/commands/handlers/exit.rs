use crate::app::runner::OpenCoder;

use tracing::info;

pub fn exit(_open_coder: &mut OpenCoder, _arg: &str) -> anyhow::Result<String> {
    info!("Exiting...");
    std::process::exit(0);
}