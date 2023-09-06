// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use idreader_lib::module_reader::reader::PersonalId;
use pcsc::*;
use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, Window};
use tauri_plugin_positioner::{Position, WindowExt};
use viuer::Config;

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit").accelerator("Cmd+Q");
    let system_tray_menu = SystemTrayMenu::new().add_item(quit);
    tauri::Builder::default()
        .setup(|app| {
            card_info(app.get_window("main").unwrap());
            Ok(())
        })
        .plugin(tauri_plugin_positioner::init())
        .system_tray(SystemTray::new().with_menu(system_tray_menu))
        .on_system_tray_event(|app, event| {
            tauri_plugin_positioner::on_tray_event(app, &event);
            match event {
                SystemTrayEvent::LeftClick {
                    position: _,
                    size: _,
                    ..
                } => {
                    let window = app.get_window("main").unwrap();
                    let _ = window.move_window(Position::TrayCenter);
                    if window.is_visible().unwrap() {
                        window.hide().unwrap();
                    } else {
                        window.show().unwrap();
                        window.set_focus().unwrap();
                    }
                }
                SystemTrayEvent::RightClick {
                    position: _,
                    size: _,
                    ..
                } => {
                    println!("system tray received a right click");
                }
                SystemTrayEvent::DoubleClick {
                    position: _,
                    size: _,
                    ..
                } => {
                    println!("system tray received a double click");
                }
                SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                    "quit" => {
                        std::process::exit(0);
                    }
                    "hide" => {
                        let window = app.get_window("main").unwrap();
                        window.hide().unwrap();
                    }
                    _ => {}
                },
                _ => {}
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn card_info(window: Window) {
    let mut old_readers = 0;
    let mut info_sent = false;
    let mut counter = 0;
    std::thread::spawn(move || loop {
        counter += 1;
        std::thread::sleep(std::time::Duration::from_secs(1));
        let ctx = match Context::establish(Scope::User) {
            Ok(ctx) => ctx,
            Err(err) => {
                eprintln!("Failed to establish context: {}", err);
                continue;
            }
        };
        if counter > 30 {
            info_sent = false;
            counter = 0;
        }
        if ctx.list_readers_len().unwrap() < 40 {
            window.emit("card_info", "No reader found").unwrap();
            continue;
        }
        if old_readers == ctx.list_readers_len().unwrap() && info_sent {
            continue;
        }
        old_readers = ctx.list_readers_len().unwrap();
        let mut readers_buf = [0; 2048];

        let mut readers = match ctx.list_readers(&mut readers_buf) {
            Ok(readers) => readers,
            Err(err) => {
                eprintln!("Failed to list readers: {}", err);
                window.emit("card_info", "No reader found").unwrap();
                continue;
            }
        };

        // Use the first reader.
        let reader = match readers.next() {
            Some(reader) => reader,
            None => {
                continue;
            }
        };

        // Connect to the card.
        let result = ctx.connect(reader, ShareMode::Shared, Protocols::ANY);
        let card = match result {
            Ok(card) => card,
            Err(Error::NoSmartcard) => {
                println!("A smartcard is not present in the reader.");
                window.emit("card_info", "No card inserted").unwrap();
                continue;
            }
            Err(Error::RemovedCard) => {
                println!("The card was removed before we could read it.");
                window.emit("card_info", "No card inserted").unwrap();
                continue;
            }
            Err(err) => {
                eprintln!("Failed to connect to card: {}", err);
                continue;
            }
        };

        let mut personal_id = PersonalId::new(&card).unwrap();
        personal_id.read_id(&card).unwrap();

        let conf = Config {
            absolute_offset: false,
            x: 0,
            y: 0,
            width: Some(42),
            height: Some(28),
            ..Default::default()
        };
        let img = image::load_from_memory(&personal_id.image).expect("Could not be read");

        viuer::print(&img, &conf).expect("Image printing failed.");
        for (_tag, item) in personal_id.personal.iter() {
            println!("{}", item);
        }
        println!(
            "{:?}",
            serde_json::from_str::<serde_json::Value>(&personal_id.to_json())
        );
        info_sent = true;

        window
            .emit(
                "card_info",
                serde_json::from_str::<serde_json::Value>(&personal_id.to_json())
                    .unwrap()
                    .to_string(),
            )
            .unwrap();
    });
}

// fn look_for_readers() {
//     // Establish a PC/SC context.
//     let ctx = match Context::establish(Scope::User) {
//         Ok(ctx) => ctx,
//         Err(err) => {
//             eprintln!("Failed to establish context: {}", err);
//             return;
//         }
//     };
//     let mut old_readers = 0;
//     loop {
//         // List available readers.
//         let mut readers_buf = [0; 2048];
//         match ctx.list_readers_len() {
//             Ok(len) => {
//                 if len == old_readers {
//                     std::thread::sleep(std::time::Duration::from_secs(1));
//                     continue;
//                 }
//                 old_readers = len;
//             }
//             Err(err) => {
//                 eprintln!("Failed to list readers: {}", err);
//                 continue;
//             }
//         }
//         let mut readers = match ctx.list_readers(&mut readers_buf) {
//             Ok(readers) => readers,
//             Err(err) => {
//                 eprintln!("Failed to list readers: {}", err);
//                 continue;
//             }
//         };

//         if old_readers == 0 {
//             println!("No smart card readers found.");
//         } else {
//             println!("Smart card readers:");
//             loop {
//                 let reader = match readers.next() {
//                     Some(reader) => reader,
//                     None => {
//                         break;
//                     }
//                 };
//                 println!("{:?}", reader);
//             }
//         }
//     }
// }
