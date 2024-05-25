use super::error::Result;
use dialoguer::{theme::ColorfulTheme, Input, Password, Select};
use gpt_core::config::Config;

pub(crate) fn config() -> Result<()> {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Action to your config?")
        .items(&["Display", "Modify"])
        .interact()?;
    match selection {
        0 => display(),
        1 => modify(),
        _ => unreachable!(),
    }
}

/// Display the current configuration.
/// If the feature `mock` is enabled, the API key will be displayed.
/// Otherwise, the API key will be masked.
fn display() -> Result<()> {
    #[cfg(feature = "mock")]
    let config = Config::read()?;
    #[cfg(not(feature = "mock"))]
    let config = Config::read_masked()?;
    println!("{}", config);
    Ok(())
}

fn modify() -> Result<()> {
    let mut config = Config::read_masked()?;
    let endpoint = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt(format!(
            " Endpoint? Current: {} [Empty to unchange]\n",
            config.endpoint()
        ))
        .allow_empty(true)
        .interact_text()?;
    if !endpoint.is_empty() {
        config.set_endpoint(endpoint);
    }
    let api_key = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("API Key? [Hidden]\n")
        .allow_empty_password(true)
        .interact()?;
    if !api_key.is_empty() {
        config.set_api_key(api_key);
    }
    config.save()?;
    display()
}
