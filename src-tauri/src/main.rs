// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::generate_context;
mod execute;

fn main() {
  tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![greet])
      .run(generate_context!())
      .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
  format!("Hello, {}!", name)
}
