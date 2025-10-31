// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{async_runtime::spawn, Manager};

mod renderer;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_transparent_wry::init())
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();
            main_window.set_title("Transparent Wry").unwrap();

            let app_handle = app.handle().clone();

            spawn(async {
                let mut renderer = renderer::Renderer::new(app_handle, main_window).await;
                renderer.run().await;
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
