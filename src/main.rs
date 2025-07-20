// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;

slint::include_modules!();
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app = AppWindow::new()?;
    let weak = app.as_weak();
    tokio::spawn({
        let weak = weak.clone();
        async move {
            let mut n = 0;
            loop {
                tokio::time::sleep(std::time::Duration::from_nanos(1)).await;
                n += 1;
                let _ = weak.upgrade_in_event_loop(move |ui| {
                    ui.set_status_text(format!("async 루프: {n}").into());
                });
            }
        }
    });
    // ui.on_request_increase_value({
    //     let ui_handle = ui.as_weak();
    //     move || {
    //         let ui = ui_handle.unwrap();
    //         ui.set_counter(ui.get_counter() + 1);
    //     }
    // });
    // app.global::<TableViewPageAdapter>();
    // ui.global()
    app.run()?;

    Ok(())
}
