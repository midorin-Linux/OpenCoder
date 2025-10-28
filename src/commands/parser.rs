pub fn parse_input(input: &str) -> Option<(String, &str)> {
    if !input.starts_with('/') {
        return None;
    }

    let parts: Vec<&str> = input.splitn(2, ' ').collect();
    let command = parts[0];
    let args = parts.get(1).unwrap_or(&"");
    
    Some((command.to_string(), args))
}