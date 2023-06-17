// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod displays;

use anyhow::Result;
use displays::Displays;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() -> Result<()> {
    let displays: Displays = Displays::new()?;

    for active_display in displays.get_active_displays().iter() {
        // displays.set_value_for_vcp(
        //     &mut active_display.display,
        //     monitaur::VCPFeatureCode::Brightness,
        //     100,
        // );

        println!(
            "brightness for monitor '{}' is {}%",
            active_display
                .display
                .info()
                .model_name
                .expect("Model name for monitor not available"),
            active_display.brightness.value()
        );
    }

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
