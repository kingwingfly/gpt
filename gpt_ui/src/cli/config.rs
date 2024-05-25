use super::error::Result;
use dialoguer::{theme::ColorfulTheme, Input, Password, Select};
use gpt_core::config::Config;

pub(crate) fn config() -> Result<()> {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Action to your config?")
        .items(&["Display", "Modify"])
        .interact()
        .unwrap();
    match selection {
        0 => display(),
        1 => modify(),
        _ => unreachable!(),
    }
}

fn display() -> Result<()> {
    #[cfg(feature = "mock")]
    let config = Config::read()?;
    #[cfg(not(feature = "mock"))]
    let config = Config::read_masked()?;
    println!("{}", config);
    Ok(())
}

fn modify() -> Result<()> {
    let mut config = Config::read()?;
    let endpoint = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt(format!(
            " Endpoint? Current: {} [Empty to unchange]\n",
            config.endpoint()
        ))
        .allow_empty(true)
        .interact_text()
        .unwrap();
    if !endpoint.is_empty() {
        config.set_endpoint(endpoint);
    }
    let api_key = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("API Key? [Hidden]\n")
        .allow_empty_password(true)
        .interact()
        .unwrap();
    if !api_key.is_empty() {
        config.set_api_key(api_key);
    }
    config.save()?;
    display()
}
