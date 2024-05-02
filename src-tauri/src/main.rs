// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod audio;

use crate::audio::*;
use anyhow::Result;
use macros::tauri_anyhow;
use specta::specta;
use tauri::Manager;

fn main() {
    let (invoke_handler, register_events) = {
        let builder = tauri_specta::ts::builder()
            .commands(tauri_specta::collect_commands![
                beep,
                get_input_devices,
                get_output_devices,
                record,
                create_window,
                get_current_input_device,
                get_current_output_device,
                set_input_device,
                set_output_device
            ])
            .events(tauri_specta::collect_events![StopRecording]);

        #[cfg(debug_assertions)] // <- Only export on non-release builds
        let builder = builder.path("../src/lib/bindings.ts");

        builder.build::<tauri::App<_>>().unwrap()
    };

    tauri::Builder::default()
        .manage(crate::audio::AudioDevices::default())
        .invoke_handler(invoke_handler)
        .setup(|app| {
            Ok({
                register_events(app);
            })
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
#[specta]
#[tauri_anyhow]
async fn create_window(app: tauri::AppHandle) -> Result<()> {
    let _webview_window = tauri::WebviewWindowBuilder::new(
        &app,
        "label",
        tauri::WebviewUrl::App("hello".parse().unwrap()),
    )
    .build()?;

    Ok(())
}

#[tauri::command]
#[specta]
#[tauri_anyhow]
async fn get_all_recordings() -> Result<Vec<String>> {
    let path = "../assets";

    Ok(std::fs::read_dir(path)?
        .flatten()
        .map(|x| x.path().to_string_lossy().to_string())
        .collect())
}
