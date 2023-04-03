#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::collections::HashMap;

use tauri::generate_handler;

mod search;


#[tauri::command]
async fn build_index(dir_path: Vec<&str>, app_handle: tauri::AppHandle) -> Result<(),()> {
    let mut app_data_dir = app_handle.path_resolver().app_data_dir().expect("failed to get data_dir");
    app_data_dir.push("index.json");
    search::build_index(dir_path, app_data_dir);
    Ok(())
}

#[tauri::command]
async fn search_files(term: String, app_handle: tauri::AppHandle) -> Result<HashMap<String, String>, search::Error> {
    let mut app_data_dir = app_handle.path_resolver().app_data_dir().expect("failed to get data_dir");
    app_data_dir.push("index.json");
    search::search_files(term, app_data_dir)
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
