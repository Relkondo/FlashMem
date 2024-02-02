// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::generate_context;
use std::sync::atomic::{AtomicBool, Ordering};

mod execute;
static IS_RUNNING: AtomicBool = AtomicBool::new(false);

fn main() {
    let _ = fix_path_env::fix();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![execute, greet])
        .run(generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn execute() -> String {
    if !IS_RUNNING.load(Ordering::SeqCst) {
        IS_RUNNING.store(true, Ordering::SeqCst);
        let notification = execute::execute();
        IS_RUNNING.store(false, Ordering::SeqCst);
        notification
    } else {
        println!("Received signal but FlashMem is already running.");
        "###-Already Running-###".to_string()
    }
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
