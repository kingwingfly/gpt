use super::{
    dialog::{confirm, input, select},
    error::Result,
};
use gpt_core::{chat::Chat, config::Config, msg::Role};

pub(crate) async fn chat() -> Result<()> {
    #[cfg(feature = "mock")]
    let mock = gpt_core::mock::Mock::new(3000, std::time::Duration::from_secs(60));
    let items = ["New", "History", "Quit"];
    #[cfg(all(feature = "cliclack", not(feature = "dialoguer")))]
    let items = (0..items.len())
        .map(|i| (i, items[i], ""))
        .collect::<Vec<_>>();
    let chosen = select("Let's chat!", &items).unwrap();
    match chosen {
        0 => new_chat(Chat::new()).await?,
        1 => history().await?,
        2 => {}
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
    let config = Config::new(crate::MOCK_SERVER, "");
    println!("{}", chat);
    let mut stdout = std::io::stdout();
    loop {
        let content = input("You:", "");
        match content {
            Ok(content) => {
                if !content.is_empty() {
                    chat.add_message(Role::User, content)
                } else {
                    if confirm("Quit?")? {
                        break;
                    }
                    continue;
                }
            }
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
    println!();
    if confirm("Do you want to save this chat?")? {
        let path = chat.save_to_dir(gpt_core::config::data_dir()?)?;
        println!("Chat saved: {}", path.to_string_lossy());
    }
    Ok(())
}

async fn history() -> Result<()> {
    let data_dir = gpt_core::config::data_dir()?;

    let mut paths: Vec<String> = std::fs::read_dir(&data_dir)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_file())
        .map(|entry| entry.file_name().to_string_lossy().to_string())
        .collect();
    loop {
        let mut ops = vec![
            "New".to_string(),
            "Delete all".to_string(),
            "Quit".to_string(),
        ];
        ops.extend_from_slice(&paths);
        #[cfg(all(feature = "cliclack", not(feature = "dialoguer")))]
        let ops = (0..ops.len()).map(|i| (i, &ops[i], "")).collect::<Vec<_>>();
        let chosen = select("Choose: ", &ops)?;
        match chosen {
            0 => {
                new_chat(Chat::new()).await?;
                break;
            }
            1 => {
                if confirm("Are you sure to delete all?")? {
                    for path in &paths {
                        std::fs::remove_file(data_dir.join(path))?;
                    }
                    paths.clear();
                    println!("All chats deleted.");
                }
            }
            2 => break,
            _ => {
                let idx = chosen - 3;
                let path = data_dir.join(&paths[idx]);
                let items = ["Open", "Delete"];
                #[cfg(all(feature = "cliclack", not(feature = "dialoguer")))]
                let items = (0..ops.len())
                    .map(|i| (i, items[i], ""))
                    .collect::<Vec<_>>();
                match select("What to do?", &items)? {
                    0 => {
                        let chat = Chat::read_from_path(path)?;
                        new_chat(chat).await?;
                        break;
                    }
                    1 => {
                        std::fs::remove_file(&path)?;
                        paths.remove(idx);
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
