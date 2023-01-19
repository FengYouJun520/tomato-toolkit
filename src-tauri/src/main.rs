#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::fs;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            let resource_path = app
                .path_resolver()
                .resolve_resource("templates/a.java")
                .expect("文件没有找到");

            let content = fs::read_to_string(&resource_path).expect("读取文件失败");
            println!("文件路径: {}", resource_path.display());
            println!("文件内容: {content}");

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
