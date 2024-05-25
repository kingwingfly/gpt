use super::error::Result;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use gpt_core::{chat::Chat, config::Config, msg::Role};

pub(crate) async fn chat() -> Result<()> {
    #[cfg(feature = "mock")]
    let mock = gpt_core::mock::Mock::new(3000, std::time::Duration::from_secs(60));
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Let's chat!")
        .items(&["New", "History"])
        .interact()
        .unwrap();
    match selection {
        0 => new_chat().await?,
        _ => unreachable!(),
    }
    #[cfg(feature = "mock")]
    mock.close();
    Ok(())
}

async fn new_chat() -> Result<()> {
    #[cfg(not(feature = "mock"))]
    let config = Config::read()?;
    #[cfg(feature = "mock")]
    let config = Config::new("http://127.0.0.1:3000", "");
    let mut chat = Chat::new();
    let mut stdout = std::io::stdout();
    loop {
        let content = Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("You: ")
            .interact();
        match content {
            Ok(content) => chat.add_message(Role::User, content),
            Err(_) => break,
        }
        tokio::select! {
            ret = chat.ask(&config, &mut stdout) => {
                match ret {
                    // add the assistant's message to the chat
                    Ok(content) => chat.add_message(Role::Assistant, content),
                    Err(e) => {
                        eprintln!("{}", e);
                        break;
                    }
                }
            },
            _ = tokio::signal::ctrl_c() => break,
        }
    }
    if Confirm::new()
        .with_prompt("Do you want to save this chat?")
        .interact()?
    {
        // chat.save()?;
        println!("Chat saved!");
    }
    Ok(())
}
