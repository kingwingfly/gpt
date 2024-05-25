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
        0 => new_chat(Chat::new()).await?,
        1 => history().await?,
        _ => unreachable!(),
    }
    #[cfg(feature = "mock")]
    mock.close();
    Ok(())
}

async fn new_chat(mut chat: Chat) -> Result<()> {
    #[cfg(not(feature = "mock"))]
    let config = Config::read()?;
    #[cfg(feature = "mock")]
    let config = Config::new("http://127.0.0.1:3000", "");
    println!("{}", chat);
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
        let path = chat.save_to_dir(gpt_core::config::data_dir()?)?;
        println!("Chat saved: {}", path.to_string_lossy());
    }
    Ok(())
}

async fn history() -> Result<()> {
    let data_dir = gpt_core::config::data_dir()?;
    let mut paths = std::fs::read_dir(&data_dir)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_file())
        .map(|entry| entry.file_name().to_string_lossy().to_string())
        .collect::<Vec<_>>();
    loop {
        let chosen = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose")
            .item("New")
            .items(&paths)
            .max_length(10)
            .interact()?;
        match chosen {
            0 => {
                new_chat(Chat::new()).await?;
                break;
            }
            _ => {
                let path = data_dir.join(&paths[chosen - 1]);
                match Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("What to do?")
                    .items(&["Open", "Delete"])
                    .interact()?
                {
                    0 => {
                        let chat = Chat::read_from_path(path)?;
                        new_chat(chat).await?;
                        break;
                    }
                    1 => {
                        std::fs::remove_file(&path)?;
                        paths.remove(chosen - 1);
                        println!("Chat deleted: {}", path.to_string_lossy());
                    }
                    _ => unreachable!(),
                }
            }
        }
        println!()
    }
    Ok(())
}
