// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::{cell::RefCell, error::Error};
use std::env;
use futures_util::{future, pin_mut, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_tungstenite::connect_async;
use slint::{ModelRc, SharedString, VecModel};


const MAX_CHAT_MESSAGES: usize = 1000;
slint::include_modules!();
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url =
        env::args().nth(1).unwrap_or_else(|| panic!("this program requires at least one argument"));
        // let messages = Arc::new(Mutex::new(Vec::<String>::new()));
    let app = AppWindow::new()?;
    let weak = app.as_weak();
    // let messages_mem = messages.clone();
    tokio::spawn({
        let weak = weak.clone();
        // let messages = messages.clone();
        async move {
            let (ws_stream, _) = connect_async(&url).await.expect("Failed to connect");
            let (_, mut read) = ws_stream.split();
            let mut n = 0;
            
            let mut buf = vec![];
            loop {
                if let Some(Ok(msg)) = read.next().await{
                    if let tokio_tungstenite::tungstenite::Message::Text(text)=msg{
                        let chat_msg = text.to_string();
                        // let mut messages = messages.lock().unwrap();
                        // messages.push(chat_msg.into());
                        if buf.len() >= MAX_CHAT_MESSAGES {
                            buf.remove(0);
                        }
                        buf.push(chat_msg);
                        // println!("DOne");
                        let v: Vec<SharedString>= buf.clone().into_iter().map(Into::into).collect();
                        let _ = weak.upgrade_in_event_loop(move |ui| {
                            
                            let model = ModelRc::new(VecModel::from(v));
                            // let v: Vec<SharedString> = buf.iter().clone();
                            // let v: Vec<slint::SharedString> = messages.iter().map(|s| s.into()).collect();
                            ui.set_socket_messages(model);
                            // ui.set_status_text(chat_msg.into());
                        });
                        // if let Some(ui) = weak.upgrade() {
                            
                            
                        //     println!("DOne");
                        // }
                        // let _ = weak.upgrade_in_event_loop(move |ui| {
                        //     ui.set_status_text(chat_msg.into());
                        // });
                    }
                }
                // tokio::time::sleep(std::time::Duration::from_nanos(1)).await;
                // n += 1;
                
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

