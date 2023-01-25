#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::fs;
use tomato_toolkit::commands::*;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![test_connection, mp_codegen])
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
