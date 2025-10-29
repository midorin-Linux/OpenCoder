use anyhow::{Context, Result};
use dialoguer::{Input, theme::ColorfulTheme};

pub struct Prompt {}

impl Prompt {
    pub fn new() -> Result<Self> {
        Ok(Self{})
    }
    
    pub fn read_input(&self, theme: &ColorfulTheme) -> Result<String> {
        let input: String = Input::with_theme(theme)
            .with_prompt(">")
            .interact_text()
            .context("Failed to read input")?;
        
        Ok(input)
    }
}