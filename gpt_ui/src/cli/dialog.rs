#[cfg(all(not(docsrs), feature = "dialoguer", feature = "cliclack"))]
compile_error!("Only one of the features `dialoguer` and `cliclack` can be enabled at a time.");
#[cfg(all(not(docsrs), not(any(feature = "dialoguer", feature = "cliclack"))))]
compile_error!("At least one of the features `dialoguer` and `cliclack` must be enabled.");

use super::error::Result;
use std::fmt::Display;

pub(super) fn init_dialog() {
    ctrlc::set_handler(|| {}).expect("Error setting Ctrl-C handler");
}

pub(super) fn input(
    prompt: impl AsRef<str>,
    placeholder: impl AsRef<str>,
    #[cfg(feature = "cliclack")] multiline: bool,
) -> Result<String> {
    #[cfg(feature = "dialoguer")]
    let input = dialoguer::Input::<String>::with_theme(&dialoguer::theme::ColorfulTheme::default())
        .with_prompt(prompt.as_ref())
        .allow_empty(true)
        .interact()?;
    #[cfg(feature = "cliclack")]
    let input = cliclack::Input::new(prompt.as_ref())
        .placeholder(placeholder.as_ref())
        .multiline(multiline)
        .required(false)
        .interact()?;
    Ok(input)
}

pub(super) fn password(prompt: impl AsRef<str>) -> Result<String> {
    #[cfg(feature = "dialoguer")]
    let pwd = dialoguer::Password::with_theme(&dialoguer::theme::ColorfulTheme::default())
        .with_prompt(prompt.as_ref())
        .interact()?;
    #[cfg(feature = "cliclack")]
    let pwd = cliclack::Password::new(prompt.as_ref())
        .mask('*')
        .interact()?;
    Ok(pwd)
}

pub(super) fn select(
    prompt: impl AsRef<str>,
    #[cfg(feature = "dialoguer")] dialoguer_items: &[impl Display],
    #[cfg(feature = "cliclack")] cliclack_itemss: &[(usize, impl Display, impl Display)],
) -> Result<usize> {
    #[cfg(feature = "dialoguer")]
    let chosen = dialoguer::Select::with_theme(&dialoguer::theme::ColorfulTheme::default())
        .with_prompt(prompt.as_ref())
        .items(dialoguer_items)
        .interact()?;
    #[cfg(feature = "cliclack")]
    let chosen = cliclack::Select::new(prompt.as_ref())
        .items(cliclack_itemss)
        .interact()?;
    Ok(chosen)
}

pub(super) fn confirm(prompt: impl AsRef<str>) -> Result<bool> {
    #[cfg(feature = "dialoguer")]
    let res = dialoguer::Confirm::with_theme(&dialoguer::theme::ColorfulTheme::default())
        .with_prompt(prompt.as_ref())
        .interact()?;
    #[cfg(feature = "cliclack")]
    let res = cliclack::Confirm::new(prompt.as_ref()).interact()?;
    Ok(res)
}
