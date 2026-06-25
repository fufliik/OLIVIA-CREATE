use crate::MainWindow;
//кнопка играть ивент
pub fn play(mw_weak: slint::Weak<MainWindow>, text: &str) {
    let text = text.to_string();

    slint::invoke_from_event_loop(move || {
        if let Some(mw) = mw_weak.upgrade() {
            mw.set_play_button_status(text.into());
        }
    })
        .unwrap();
}

pub fn look(mw_weak: slint::Weak<MainWindow>, enabled: bool) {

    slint::invoke_from_event_loop(move || {
        if let Some(mw) = mw_weak.upgrade() {
            mw.set_play_button_look(enabled.into());
        }
    })
        .unwrap();
}