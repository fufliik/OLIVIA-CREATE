mod mw_event;
mod minecraft;

slint::include_modules!();


#[tokio::main]
async fn main() {
    let mw = MainWindow::new().unwrap();
    let mw_weak = mw.as_weak();
    mw.set_play_button_status("Играть".into());
    mw.on_button_play_run(move || {

        let mw_weak = mw_weak.clone();

        tokio::spawn(async move {
            let _ = minecraft::main_minecrat(mw_weak).await;
        });
    });

    mw.run().unwrap();
}
