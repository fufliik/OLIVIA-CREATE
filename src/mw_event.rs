use lyceris::minecraft::emitter::Event;
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
            //
            if enabled {
                mw.set_play_button_status_color(slint::Color::from_rgb_u8(255, 99, 99));
                mw.set_play_button_status("Запуск...".into());
            }
            if!enabled{
                mw.set_play_button_status_color(slint::Color::from_rgb_u8(35, 134, 54));
                mw.set_play_button_status("Играть".into());
            };
        }
    })
        .unwrap();
}

/*
pub fn hotbar(mw_weak: slint::Weak<MainWindow>, enabled: bool) {
    slint::invoke_from_event_loop(move || {
        if let Some(mw)= mw_weak.upgrade() {
            mw.set_hotbar_enabled(enabled.into());
        }
    })
    .unwrap();
}
pub fn event_mdr(mw_weak: slint::Weak<MainWindow>, prigres_bar: &str) {
    let prigres_bar_int = prigres_bar.to_string();
    slint::invoke_from_event_loop(move || {
        if let Some(mw) = mw_weak.upgrade() {
            mw.set_event_multiple_download_rogress(prigres_bar_int.into());

        }
    })
    .unwrap();
}
*/
// if let Some(pos) = path.rfind('\\')
pub fn hotbar(mw_weak: slint::Weak<MainWindow>,enabled: bool, path: &str, progres: &str ) {
    let path = path.to_string();
    let progres = progres.to_string();

    slint::invoke_from_event_loop(move || {
        if let Some(mw) = mw_weak.upgrade() {

            mw.set_hotbar_enabled(enabled.into());
            
            if path != "0" {
                if let Some(pos) = path.rfind(|c | c == '\\' || c == '/') {
                    let sline = &path[pos + 1..];

                    let truncated_name: String = sline.chars().take(25).collect();
                    mw.set_event_multiple_download_text(slint::SharedString::from(truncated_name));
                } 
                else { println!("No path found"); }
            };
            
            if progres != "0"{
                mw.set_event_multiple_download_rogress(slint::SharedString::from(&progres));
            }
        }
    })
    .unwrap();
}






