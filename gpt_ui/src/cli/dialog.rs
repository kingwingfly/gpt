use super::error::Result;
use std::fmt::Display;

pub(super) fn init_dialog() {
    ctrlc::set_handler(|| {}).expect("Error setting Ctrl-C handler");
}

#[cfg(all(feature = "dialoguer", not(feature = "cliclack")))]
pub(super) fn input(prompt: impl AsRef<str>, _: impl AsRef<str>) -> Result<String> {
    Ok(
        dialoguer::Input::<String>::with_theme(&dialoguer::theme::ColorfulTheme::default())
            .with_prompt(prompt.as_ref())
            .allow_empty(true)
            .interact()?,
    )
}

#[cfg(all(feature = "cliclack", not(feature = "dialoguer")))]
pub(super) fn input(prompt: impl AsRef<str>, placeholder: impl AsRef<str>) -> Result<String> {
    Ok(cliclack::Input::new(prompt.as_ref())
        .placeholder(placeholder.as_ref())
        .multiline(true)
        .required(false)
        .interact()?)
}

#[cfg(all(feature = "dialoguer", not(feature = "cliclack")))]
pub(super) fn password(prompt: impl AsRef<str>) -> Result<String> {
    Ok(
        dialoguer::Password::with_theme(&dialoguer::theme::ColorfulTheme::default())
            .with_prompt(prompt.as_ref())
            .interact()?,
    )
}

#[cfg(all(feature = "cliclack", not(feature = "dialoguer")))]
pub(super) fn password(prompt: impl AsRef<str>) -> Result<String> {
    Ok(cliclack::Password::new(prompt.as_ref())
        .mask('*')
        .interact()?)
}

#[cfg(all(feature = "dialoguer", not(feature = "cliclack")))]
pub(super) fn select<T: Display>(prompt: impl AsRef<str>, items: &[T]) -> Result<usize> {
    Ok(
        dialoguer::Select::with_theme(&dialoguer::theme::ColorfulTheme::default())
            .with_prompt(prompt.as_ref())
            .items(items)
            .interact()?,
    )
}

#[cfg(all(feature = "cliclack", not(feature = "dialoguer")))]
pub(super) fn select<K: Clone + Eq, T1: Display, T2: Display>(
    prompt: impl AsRef<str>,
    items: &[(K, T1, T2)],
) -> Result<K> {
    Ok(cliclack::Select::new(prompt.as_ref())
        .items(items)
        .interact()?)
}

#[cfg(all(feature = "dialoguer", not(feature = "cliclack")))]
pub(super) fn confirm(prompt: impl AsRef<str>) -> Result<bool> {
    Ok(
        dialoguer::Confirm::with_theme(&dialoguer::theme::ColorfulTheme::default())
            .with_prompt(prompt.as_ref())
            .interact()?,
    )
}

#[cfg(all(feature = "cliclack", not(feature = "dialoguer")))]
pub(super) fn confirm(prompt: impl AsRef<str>) -> Result<bool> {
    Ok(cliclack::Confirm::new(prompt.as_ref()).interact()?)
}
