use crate::app::runner::OpenCoder;

pub fn help(_client: &mut OpenCoder, _args: &str) -> anyhow::Result<String> {
    Ok(r#"
Usage: <command> [args]

Commands:
  /set [args]    Set a value
  /exit          Exit the application
  /help          Show this help message

For more information on a specific command, run /<command> help.
"#.to_string())
}
