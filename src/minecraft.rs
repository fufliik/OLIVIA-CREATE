use lyceris::minecraft::loader::neoforge::NeoForge;
use lyceris::minecraft::config::Memory;
use lyceris::minecraft::{
    config::ConfigBuilder,
    emitter::{Emitter, Event},
    install::install,
    launch::launch,
};
use std::path::PathBuf;
use crate::{directories, mw_event, MainWindow};

//mw_event::play(mw_weak.clone(), "Загрузка...");
pub async fn main_minecrat(mw_weak: slint::Weak<MainWindow>, ) -> Result<(), Box<dyn std::error::Error>> {
    mw_event::look(mw_weak.clone(), true);
    let emitter = Emitter::default();
    let mw_weak2 = mw_weak.clone();


    emitter
        .on(
            Event::MultipleDownloadProgress,
            move |(path, current, total): (String, u64, u64)| {
                let event = format!("{} / {}", current, total);
                mw_event::hotbar(mw_weak2.clone(), false, &path, &event);
            },
        )
        .await;

    //в настройки
    emitter
        .on(Event::Console, |line: String| {
            println!("Line: {}", line);
        })
        .await;

    let launcher_dir = directories::dir().expect("ERROR - dirs");
    let config = ConfigBuilder::new(
        &launcher_dir,
        "1.21.1".into(),
        lyceris::auth::AuthMethod::Offline {
            username: "Lyceris".into(),
            uuid: None,

        },

    )
        .custom_args(vec![
            "--quickPlayMultiplayer".into(),
            "95.216.48.148:25993".into(),
        ])
        .memory(Memory::Gigabyte(6))
        .loader(NeoForge("21.1.233".to_string()).into())
        .build();


    install(&config, Some(&emitter)).await?;  //ll

    mw_event::hotbar(mw_weak.clone(), true,"0","0");

    let mut child = launch(&config, Some(&emitter)).await?; //ll

    child.wait().await?; //ll

    mw_event::look(mw_weak.clone(), false);

    Ok(())
}