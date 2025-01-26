use super::dialog::{input, password, select};
use super::error::Result;
use gpt_core::{config::Config, model::ModelVersion};

pub(crate) fn config() -> Result<()> {
    let items = &[(0, "Display", ""), (1, "Modify", "")];
    match select("Action to your config?", items) {
        Ok(0) => display(),
        Ok(1) => modify(),
        _ => Ok(()),
    }
}

pub(crate) fn choose_model() -> Result<()> {
    let mut config = Config::load().unwrap_or_default();
    let items = &[
        (0, ModelVersion::DeepSeekChat, ""),
        (1, ModelVersion::DeepSeekReasoner, ""),
        (2, ModelVersion::GPT4o, ""),
        (3, ModelVersion::GPT4Turbo, ""),
        (4, ModelVersion::Llama405B, ""),
        (5, ModelVersion::Llama70B, ""),
        (6, ModelVersion::Llama8B, ""),
    ];
    if let Ok(i) = select("Choose default model:", items) {
        config.model = items[i].1;
        config.store()?;
    }
    Ok(())
}

/// Display the current configuration.
/// If the feature `mock` is enabled, the API key will be displayed.
/// Otherwise, the API key will be masked.
fn display() -> Result<()> {
    #[cfg(feature = "mock")]
    let config = Config::load().unwrap_or_default();
    #[cfg(not(feature = "mock"))]
    let config = Config::load().unwrap_or_default();
    println!("{}", config);
    Ok(())
}

fn modify() -> Result<()> {
    let mut config = Config::load().unwrap_or_default();
    let mut changed = false;
    match input("Endpoint? [Empty to unchange]\n", &config.endpoint, false) {
        Ok(content) if !content.is_empty() => {
            config.endpoint = content.parse().expect("Invalid URL");
            changed = true;
        }
        _ => {}
    }
    match password("API Key? [Empty to unchange]\n") {
        Ok(content) if !content.is_empty() => {
            config.api_key = content;
            changed = true;
        }
        _ => {}
    }
    if changed {
        config.store()?;
    }
    display()
}
