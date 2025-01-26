use super::error::Result;
use std::fmt::Display;

pub(super) fn init_dialog() {
    ctrlc::set_handler(|| {}).expect("Error setting Ctrl-C handler");
}

pub(super) fn input(
    prompt: impl AsRef<str>,
    placeholder: impl AsRef<str>,
    multiline: bool,
) -> Result<String> {
    let mut input = cliclack::Input::new(prompt.as_ref())
        .placeholder(placeholder.as_ref())
        .required(false);
    if multiline {
        input = input.multiline()
    }
    Ok(input.interact::<String>()?)
}

pub(super) fn password(prompt: impl AsRef<str>) -> Result<String> {
    let pwd = cliclack::Password::new(prompt.as_ref())
        .mask('*')
        .interact()?;
    Ok(pwd)
}

pub(super) fn select(
    prompt: impl AsRef<str>,
    cliclack_itemss: &[(usize, impl Display, impl Display)],
) -> Result<usize> {
    let chosen = cliclack::Select::new(prompt.as_ref())
        .items(cliclack_itemss)
        .interact()?;
    Ok(chosen)
}

pub(super) fn confirm(prompt: impl AsRef<str>) -> Result<bool> {
    let res = cliclack::Confirm::new(prompt.as_ref()).interact()?;
    Ok(res)
}
