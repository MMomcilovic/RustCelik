// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use idreader_lib::module_reader::reader::{PersonalId};
use pcsc::*;
use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, Window};
use tauri_plugin_positioner::{Position, WindowExt};
use pdfexporter_lib::exporter::pdf;
use directories::UserDirs;

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit").accelerator("Cmd+Q");
    let system_tray_menu = SystemTrayMenu::new().add_item(quit);
    tauri::Builder::default()
        .setup(|app| {
            card_info(app.get_window("main").unwrap());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![create_pdf])
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

#[tauri::command]
fn create_pdf(app: tauri::AppHandle) {
    let window = app.get_window("main").unwrap();
    if let Some(user_dirs) = UserDirs::new() {
        let path = user_dirs.document_dir();
        println!("{:?}", path.unwrap().to_str().unwrap());
        let ctx = match Context::establish(Scope::User) {
            Ok(ctx) => ctx,
            Err(_err) => {
                window.emit("card_info", "Error").unwrap();
                return;
            }
        };
        let mut readers_buf = [0; 2048];

        // List readers.
        let _readers = match ctx.list_readers(&mut readers_buf) {
            Ok(readers) => readers,
            Err(err) => {
                eprintln!("Failed to list readers: {}", err);
                window.emit("card_info", "No reader found").unwrap();
                return;
            }
        };
        let mut readers = match ctx.list_readers(&mut readers_buf) {
            Ok(readers) => readers,
            Err(err) => {
                eprintln!("Failed to list readers: {}", err);
                window.emit("card_info", "No reader found").unwrap();
                return;
            }
        };
                // Use the first reader.
                let reader = match readers.next() {
                    Some(reader) => reader,
                    None => {
                        return;
                    }
                };
                let mut reader_valid = true;
                let result = ctx.connect(reader, ShareMode::Shared, Protocols::ANY);
                let card = match result {
                    Ok(card) => card,
                    Err(Error::NoSmartcard) => {
                        println!("A smartcard is not present in the reader.");
                        window.emit("card_info", "No card inserted").unwrap();
                        return;
                    }
                    Err(Error::RemovedCard) => {
                        println!("The card was removed before we could read it.");
                        window.emit("card_info", "No card inserted").unwrap();
                        return;
                    }
                    Err(err) => {
                        eprintln!("Failed to connect to card: {}", err);
                        return;
                    }
                };
                let _buffer = match card.get_attribute_owned(Attribute::AtrString) {
                    Err(_) => {
                        let _ = window.emit("card_info", "No reader found").unwrap();
                        reader_valid = false;
                    }
                    _ => (),
                };
                if reader_valid {
                    let mut personal_id = PersonalId::new(&card).unwrap();
                    personal_id.read_id(&card).unwrap_or_else(|_| {
                        let _ = window.emit("card_info", "Error while reading card!");
                    });
                    pdf::copy_font();
                    let _ = pdf::topdf(&personal_id, path.unwrap().to_str().unwrap());
                    let _ = window.emit("card_info", "Doc saved!");
                };
                }
}

fn card_info(window: Window) {
    let mut card_read = false;
    let mut reader_states = vec![ReaderState::new(PNP_NOTIFICATION(), State::UNAWARE)];
    let ctx = match Context::establish(Scope::User) {
        Ok(ctx) => ctx,
        Err(_err) => {
            window.emit("card_info", "Error").unwrap();
            return;
        }
    };
    std::thread::spawn(move || loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
        // Remove disconnected readers.
        fn is_dead(rs: &ReaderState) -> bool {
            rs.event_state().intersects(State::UNKNOWN | State::IGNORE)
        }
        fn disconnected(rs: &ReaderState) -> bool {
            rs.event_state().intersects(State::EMPTY)
        }
        for rs in &reader_states {
            if is_dead(rs) {
                println!("Removing {:?}", rs.name());
            } else if disconnected(rs) {
                card_read = false;
            }
        }
        reader_states.retain(|rs| !is_dead(rs));

        let mut readers_buf = [0; 2048];

        // List readers.
        let readers = match ctx.list_readers(&mut readers_buf) {
            Ok(readers) => readers,
            Err(err) => {
                eprintln!("Failed to list readers: {}", err);
                window.emit("card_info", "No reader found").unwrap();
                card_read = false;
                continue;
            }
        };
        for reader in readers {
            if !reader_states.iter().any(|rs| rs.name() == reader) {
                println!("Adding {:?}", reader);
                reader_states.push(ReaderState::new(reader, State::UNAWARE));
            }
        }
        for rs in &mut reader_states {
            rs.sync_current_state();
        }

        let mut readers = match ctx.list_readers(&mut readers_buf) {
            Ok(readers) => readers,
            Err(err) => {
                eprintln!("Failed to list readers: {}", err);
                window.emit("card_info", "No reader found").unwrap();
                card_read = false;
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
        if reader_states.len() > 1 && !card_read {
            let mut reader_valid = true;
            // Connect to the card.
            let result = ctx.connect(reader, ShareMode::Shared, Protocols::ANY);
            let card = match result {
                Ok(card) => card,
                Err(Error::NoSmartcard) => {
                    println!("A smartcard is not present in the reader.");
                    window.emit("card_info", "No card inserted").unwrap();
                    card_read = false;
                    continue;
                }
                Err(Error::RemovedCard) => {
                    println!("The card was removed before we could read it.");
                    window.emit("card_info", "No card inserted").unwrap();
                    card_read = false;
                    continue;
                }
                Err(err) => {
                    eprintln!("Failed to connect to card: {}", err);
                    continue;
                }
            };
            let _buffer = match card.get_attribute_owned(Attribute::AtrString) {
                Err(_) => {
                    let _ = window.emit("card_info", "No reader found").unwrap();
                    reader_valid = false;
                }
                _ => (),
            };
            if reader_valid {
                let mut personal_id = PersonalId::new(&card).unwrap();
                personal_id.read_id(&card).unwrap_or_else(|_| {
                    let _ = window.emit("card_info", "Error while reading card!");
                });
                card_read = true;
                println!("Sending card info to frontend");
                window
                    .emit(
                        "card_info",
                        serde_json::from_str::<serde_json::Value>(&personal_id.to_json())
                            .unwrap()
                            .to_string(),
                    )
                    .unwrap();
            }
        }
        // Wait until the state changes.
        ctx.get_status_change(None, &mut reader_states)
            .expect("failed to get status change");

        // Print current state.
        println!();
        for rs in &reader_states {
            if rs.name() != PNP_NOTIFICATION() {
                println!("{:?} {:?} {:?}", rs.name(), rs.event_state(), rs.atr());
            }
        }
    });
}
