// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{generate_context, State};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, MutexGuard};

mod execute;
mod utils;

static IS_RUNNING: AtomicBool = AtomicBool::new(false);
#[derive(Debug)]
struct SettingsState {
    target_language: String,
    platform: String
}
type SharedSettings = Arc<Mutex<SettingsState>>;

impl Default for SettingsState {
    fn default() -> Self {
        SettingsState {
            target_language: "English".to_string(),
            platform: "Default".to_string()
        }
    }
}

fn main() {
    let _ = fix_path_env::fix();
    tauri::Builder::default()
        .manage(SharedSettings::default())
        .invoke_handler(tauri::generate_handler![
        execute,
        set_target_language,
        set_platform])
        .run(generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn execute(state: State<'_, SharedSettings>) -> String {
    if !IS_RUNNING.load(Ordering::SeqCst) {
        IS_RUNNING.store(true, Ordering::SeqCst);
        let settings: MutexGuard<SettingsState> = state.lock().unwrap();
        let notification = execute::execute(settings);
        IS_RUNNING.store(false, Ordering::SeqCst);
        notification
    } else {
        println!("Received signal but FlashMem is already running.");
        "###-Already Running-###".to_string()
    }
}

#[tauri::command]
fn set_target_language(value: String, settings: State<'_, SharedSettings>) {
    let mut settings = settings.lock().unwrap();
    settings.target_language = value;
}

#[tauri::command]
fn set_platform(value: String, settings: State<'_, SharedSettings>) {
    let mut settings = settings.lock().unwrap();
    settings.platform = value;
}