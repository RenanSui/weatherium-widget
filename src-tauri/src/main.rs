// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dotenvy::dotenv;
use std::env;
use tauri::{App, AppHandle, LogicalPosition, Manager, SystemTray, SystemTrayEvent};
use window_shadows::set_shadow;
use window_vibrancy::{apply_blur, apply_vibrancy, NSVisualEffectMaterial};

fn main() {
    let tray: SystemTray = SystemTray::new();
    dotenv().expect(".env file not found");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_env])
        .setup(setup_window)
        .system_tray(tray)
        .on_system_tray_event(tray_event)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn setup_window(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let window = app.get_window("main").unwrap();

    #[cfg(target_os = "macos")]
    apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, None)
        .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

    #[cfg(target_os = "windows")]
    apply_blur(&window, Some((18, 18, 18, 125)))
        .expect("Unsupported platform! 'apply_blur' is only supported on Windows");

    set_shadow(&window, true).expect("Unsupported platform!");

    Ok(())
}

fn tray_event(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::LeftClick {
            position: _,
            size: _,
            ..
        } => {
            // hide window if visible
            let window = match app.get_window("main") {
                Some(window) => match window.is_visible().expect("winvis") {
                    true => {
                        window.hide().expect("winhide");
                        return;
                    }
                    false => window,
                },
                None => {
                    return;
                }
            };

            // show window if not visible
            #[cfg(not(target_os = "macos"))]
            {
                window.show().unwrap();
            }
            window.set_focus().unwrap();

            // position window
            let _ = window.set_position(LogicalPosition { x: 8, y: 8 });
        }
        _ => {}
    }
}

#[tauri::command]
fn get_env(name: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| "".to_string())
}
