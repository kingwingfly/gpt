use super::{
    dialog::{confirm, input, select},
    error::Result,
};
use gpt_core::{chat::Chat, config::Config, msg::Role};

pub(crate) async fn chat() -> Result<()> {
    #[cfg(feature = "mock")]
    let mock = gpt_core::mock::Mock::new(3000, std::time::Duration::from_secs(60));
    let items = ["New", "History", "Config", "Quit"];
    #[cfg(all(feature = "cliclack", not(feature = "dialoguer")))]
    let items = (0..items.len())
        .map(|i| (i, items[i], ""))
        .collect::<Vec<_>>();
    match select("Let's chat!", &items) {
        Ok(0) => new_chat(Chat::new()).await?,
        Ok(1) => history().await?,
        Ok(2) => super::config::config()?,
        _ => {}
    }
    #[cfg(feature = "mock")]
    mock.close();
    Ok(())
}

pub(crate) async fn new_chat(mut chat: Chat) -> Result<()> {
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
    if let Ok(true) = confirm("Do you want to save this chat?") {
        if chat.topic().is_empty() {
            println!("generating summary...");
            let mut summary_chat = Chat::summary_extraction();
            let content = &format!("{}", chat);
            summary_chat.add_message(Role::User, content.to_owned());
            let mut topic = vec![];
            summary_chat.ask(&config, &mut topic).await?;
            chat.set_topic(String::from_utf8_lossy(&topic).trim().to_string());
        }
        let path = chat.save_to_dir(gpt_core::config::data_dir()?)?;
        println!("Chat saved: {}", path.to_string_lossy());
    }
    Ok(())
}

pub(crate) async fn history() -> Result<()> {
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
        match select("Choose: ", &ops) {
            Ok(0) => {
                new_chat(Chat::new()).await?;
                break;
            }
            Ok(1) => {
                if confirm("Are you sure to delete all?")? {
                    for path in &paths {
                        std::fs::remove_file(data_dir.join(path))?;
                    }
                    paths.clear();
                    println!("All chats deleted.");
                }
            }
            Ok(2) | Err(_) => break,
            Ok(chosen) => {
                let idx = chosen - 3;
                let path = data_dir.join(&paths[idx]);
                let items = ["Open", "Delete"];
                #[cfg(all(feature = "cliclack", not(feature = "dialoguer")))]
                let items = (0..items.len())
                    .map(|i| (i, items[i], ""))
                    .collect::<Vec<_>>();
                match select("What to do?", &items) {
                    Ok(0) => {
                        let chat = Chat::read_from_path(path)?;
                        new_chat(chat).await?;
                        break;
                    }
                    Ok(1) => {
                        std::fs::remove_file(&path)?;
                        paths.remove(idx);
                        println!("Chat deleted: {}", path.to_string_lossy());
                    }
                    _ => {}
                }
            }
        }
        println!()
    }
    Ok(())
}
