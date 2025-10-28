// ToDo: Helpコマンドを作る
use crate::infrastructure::lm::client::Client;

pub fn help(client: &mut Client, args: &str) -> anyhow::Result<String> {
    Ok("help command".to_string())
}