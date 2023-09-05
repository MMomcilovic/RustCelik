// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use pcsc::*;
use std::thread;
use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};
use tauri_plugin_positioner::{Position, WindowExt};

fn main() {
    let look_for_readers = thread::spawn(|| {
        // Establish a PC/SC context.
        let ctx = match Context::establish(Scope::User) {
            Ok(ctx) => ctx,
            Err(err) => {
                eprintln!("Failed to establish context: {}", err);
                std::process::exit(1);
            }
        };
        let mut old_readers = 0;
        loop {
            // List available readers.
            let mut readers_buf = [0; 2048];
            match ctx.list_readers_len() {
                Ok(len) => {
                    if len == old_readers {
                        std::thread::sleep(std::time::Duration::from_secs(1));
                        continue;
                    }
                    old_readers = len;
                }
                Err(err) => {
                    eprintln!("Failed to list readers: {}", err);
                    std::process::exit(1);
                }
            }
            let mut readers = match ctx.list_readers(&mut readers_buf) {
                Ok(readers) => readers,
                Err(err) => {
                    eprintln!("Failed to list readers: {}", err);
                    std::process::exit(1);
                }
            };

            if old_readers == 0 {
                println!("No smart card readers found.");
            } else {
                println!("Smart card readers:");
                loop {
                    let reader = match readers.next() {
                        Some(reader) => reader,
                        None => {
                            break;
                        }
                    };
                    println!("{:?}", reader);
                }
            }
        }
    });
    tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .invoke_handler(tauri::generate_handler![greet])
        .system_tray(
            SystemTray::new()
                .with_menu(SystemTrayMenu::new().add_item(
                    CustomMenuItem::new("quit".to_string(), "Quit").accelerator("Cmd+Q"),
                )),
        )
        .on_system_tray_event(|app, event| {
            tauri_plugin_positioner::on_tray_event(app, &event);
            match event {
                SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                    "quit" => {
                        std::process::exit(0);
                    }
                    _ => {}
                },
                SystemTrayEvent::LeftClick {
                    position: _,
                    size: _,
                    ..
                } => {
                    let window = app.get_window("main").unwrap();
                    // toggle application window
                    let _ = window.move_window(Position::TrayCenter);
                    if window.is_visible().unwrap() {
                        window.hide().unwrap();
                    } else {
                        window.show().unwrap();
                        window.set_focus().unwrap();
                    }
                }
                _ => {}
            }
        })
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::Focused(is_focused) => {
                // detect click outside of the focused window and hide the app
                if !is_focused {
                    event.window().hide().unwrap();
                }
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    look_for_readers.join().unwrap();
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
