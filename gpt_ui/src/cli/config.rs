use super::dialog::{input, password, select};
use super::error::Result;
use gpt_core::config::Config;

pub(crate) fn config() -> Result<()> {
    #[cfg(all(feature = "dialoguer", not(feature = "cliclack")))]
    let items = &["Display", "Modify"];
    #[cfg(all(feature = "cliclack", not(feature = "dialoguer")))]
    let items = &[(0, "Display", ""), (1, "Modify", "")];
    match select("Action to your config?", items) {
        Ok(0) => display(),
        Ok(1) => modify(),
        _ => Ok(()),
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
    let mut config = Config::read()?;
    match input(
        "Endpoint? [Empty to unchange]\n",
        config.endpoint(),
        #[cfg(feature = "cliclack")]
        false,
    ) {
        Ok(content) if !content.is_empty() => config.set_endpoint(content),
        _ => return Ok(()),
    }
    match password("API Key? [Empty to unchange]\n") {
        Ok(content) if !content.is_empty() => config.set_api_key(content),
        _ => return Ok(()),
    }
    config.save()?;
    display()
}
