#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tomato_toolkit::commands::*;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            test_connection,
            mp_codegen,
            generate_preview
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
