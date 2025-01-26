use super::dialog::{input, password, select};
use super::error::Result;
use gpt_core::config::Configs;
use gpt_core::{config::Config, model::ModelVersion};

pub(crate) fn config() -> Result<()> {
    let items = &[(0, "Display", ""), (1, "Modify", ""), (2, "Delete", "")];
    match select("Action to your config?", items) {
        Ok(0) => display(),
        Ok(1) => modify(),
        Ok(2) => delete(),
        _ => Ok(()),
    }
}

pub(crate) fn choose_model() -> Result<()> {
    let mut configs = Configs::load().unwrap_or_default();
    let config = configs.current_mut();
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
        configs.store()?;
    }
    Ok(())
}

/// Display the current configuration.
/// If the feature `mock` is enabled, the API key will be displayed.
/// Otherwise, the API key will be masked.
fn display() -> Result<()> {
    #[cfg(feature = "mock")]
    let config = Configs::load().unwrap_or_default();
    #[cfg(not(feature = "mock"))]
    let config = Configs::load().unwrap_or_default();
    println!("{}", config);
    Ok(())
}

fn modify() -> Result<()> {
    let mut configs = Configs::load().unwrap_or_default();
    match select(
        "Which one",
        &configs
            .configs
            .iter()
            .enumerate()
            .map(|(i, c)| (i, c.to_string(), ""))
            .chain([(configs.configs.len(), "New".to_string(), "")])
            .collect::<Vec<_>>(),
    ) {
        Ok(i) => {
            if i == configs.configs.len() {
                configs.configs.push(Config::default());
            }
            configs.current = i
        }
        _ => return Ok(()),
    }
    match input(
        "Endpoint? [Empty to unchange]\n",
        &configs.current().endpoint,
        false,
    ) {
        Ok(content) if !content.is_empty() => {
            configs.current_mut().endpoint = content.parse().expect("Invalid URL");
        }
        _ => {}
    }
    match password("API Key? [Empty to unchange]\n") {
        Ok(content) if !content.is_empty() => {
            configs.current_mut().api_key = content;
        }
        _ => {}
    }
    configs.store()?;
    display()
}

fn delete() -> Result<()> {
    let mut configs = Configs::load().unwrap_or_default();
    match select(
        "Which one to delete?",
        &configs
            .configs
            .iter()
            .enumerate()
            .map(|(i, c)| (i, c.to_string(), ""))
            .collect::<Vec<_>>(),
    ) {
        Ok(i) => {
            configs.configs.remove(i);
            match i.cmp(&configs.current) {
                std::cmp::Ordering::Less => configs.current -= 1,
                std::cmp::Ordering::Equal => configs.current = 0,
                std::cmp::Ordering::Greater => {}
            }
            configs.store()?;
        }
        _ => return Ok(()),
    }
    display()
}
