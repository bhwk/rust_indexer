#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::collections::HashMap;

use tauri::generate_handler;

mod search;

#[tauri::command]
async fn build_index(dir_path: Vec<&str>) -> Result<(),()> {
    search::build_index(dir_path);
    Ok(())
}

#[tauri::command]
async fn search_files(term: String) -> Result<HashMap<String, String>, search::Error> {
    search::search_files(term)
}

#[tauri::command]
async fn open_file(path: String) -> Result<(), search::Error> {
    search::open_file(path)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(generate_handler![search_files, build_index, open_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
