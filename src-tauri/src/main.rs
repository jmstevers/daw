// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod audio;

use crate::audio::{play_sound, record_sound};
use macros::tauri_anyhow;
use tauri::generate_handler;

fn main() {
    tauri::Builder::default()
        .invoke_handler(generate_handler![create_window, play_sound, record_sound])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
#[tauri_anyhow]
async fn create_window(app: tauri::AppHandle) -> anyhow::Result<()> {
    let webview_window = tauri::WebviewWindowBuilder::new(
        &app,
        "label",
        tauri::WebviewUrl::App("hello".parse().unwrap()),
    )
    .build()?;

    Ok(())
}
