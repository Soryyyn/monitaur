// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// use ddc_hi::{traits::Ddc, Display};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    // for mut display in Display::enumerate() {
    //     display.update_capabilities().unwrap();

    //     println!(
    //         "{:?} {}: {:?} {:?}",
    //         display.backend(),
    //         display.id,
    //         display.info().manufacturer_id,
    //         display.info().model_name
    //     );

    //     if let Some(feature) = display.capabilities {
    //         match feature.ty {
    //             Some()
    //         }

    //         let value = display.handle.get_vcp_feature(0x10);
    //         println!("{:?}", value);
    //     }
    // }

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
