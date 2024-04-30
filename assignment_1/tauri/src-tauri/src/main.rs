// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
const APIURL: &str = "https://api.vndb.org/kana/vn";

struct vnDataRaw {
    title: String,
    aliases: Vec<String>,
    devstatus: u8,
    released: String
}

struct imageRaw {
    url: String,
    sexual: u8,
    violence: u8
}

#[tauri::command]
async fn search(code: String) {
    
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
