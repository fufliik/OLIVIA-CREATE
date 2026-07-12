mod mw_event;
mod minecraft;
mod directories;
mod auth;
use auth::{load_username, save_username};
slint::include_modules!();


#[tokio::main]
async fn main() {
    let ui = MainWindow::new().unwrap();

    let username = load_username();
    ui.set_username(username.clone().into());

    ui.on_save_username(move |username: slint::SharedString| {
        println!("Сохранение username: {}", username);
        let _ = save_username(username);
    });


    let mw_weak = ui.as_weak();
    ui.set_play_button_status("Играть".into());
    ui.on_button_play_run(move || {

        let mw_weak = mw_weak.clone();

        tokio::spawn(async move {
            let _ = minecraft::main_minecrat(mw_weak).await;
        });
    });







    ui.run().unwrap();
}
