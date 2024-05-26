use super::dialog::{input, password, select};
use super::error::Result;
use gpt_core::config::Config;

pub(crate) fn config() -> Result<()> {
    #[cfg(all(feature = "dialoguer", not(feature = "cliclack")))]
    let items = &["Display", "Modify"];
    #[cfg(all(feature = "cliclack", not(feature = "dialoguer")))]
    let items = &[(0, "Display", ""), (1, "Modify", "")];
    let chosen = select("Action to your config?", items)?;
    match chosen {
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
    let endpoint = input(" Endpoint? [Empty to unchange]\n", config.endpoint())?;
    if !endpoint.is_empty() {
        config.set_endpoint(endpoint);
    }
    let api_key = password("API Key? [Hidden]\n")?;
    if !api_key.is_empty() {
        config.set_api_key(api_key);
    }
    config.save()?;
    display()
}
