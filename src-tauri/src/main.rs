#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use ketlms_dist_tool::dropbox::{self, DropboxState};
use std::sync::Mutex;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .manage(DropboxState {
            client: Mutex::new(None),
            token_cache: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![
            dropbox::commands::authorize,
            greet,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
