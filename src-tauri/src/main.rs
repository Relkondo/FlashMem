// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{env};
use tauri::{generate_context, State};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, MutexGuard};

mod execute;
mod utils;

static DEBUG_TESSDATA_PATH: &'static str = "libs/tesseract/5.3.4/share/tessdata";
static RELEASE_TESSDATA_PATH: &'static str = "../Resources/libs/tesseract/5.3.4/share/tessdata";

static IS_RUNNING: AtomicBool = AtomicBool::new(false);
#[derive(Debug)]
struct SettingsState {
    target_language: String,
    origin_language: String,
    platform: String
}
type SharedSettings = Arc<Mutex<SettingsState>>;

impl Default for SettingsState {
    fn default() -> Self {
        SettingsState {
            target_language: "English".to_string(),
            origin_language: "Automatic".to_string(),
            platform: "Default".to_string()
        }
    }
}

fn set_tessdata_prefix_release(relative_path: &str) {
    if let Ok(exe_path) = env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            let absolute_path = exe_dir.join(relative_path);
            if let Some(absolute_path_str) = absolute_path.to_str() {
                println!("Setting TESSDATA_PREFIX to {}", absolute_path_str);
                env::set_var("TESSDATA_PREFIX", absolute_path_str);
            }
        } else {
            eprintln!("TESSDATA_PREFIX cloud not be set: Failed to get the executable directory.");
        }
    } else {
        eprintln!("TESSDATA_PREFIX cloud not be set: Failed to get the executable path.");
    }
    println!("$TESSDATA_PREFIX={}", env::var("TESSDATA_PREFIX").unwrap());
}

fn main() {
    let _ = fix_path_env::fix();
    #[cfg(not(debug_assertions))]
    set_tessdata_prefix_release(RELEASE_TESSDATA_PATH);
    #[cfg(debug_assertions)]
    set_tessdata_prefix_release(DEBUG_TESSDATA_PATH);

    tauri::Builder::default()
        .manage(SharedSettings::default())
        .invoke_handler(tauri::generate_handler![
        execute,
        set_target_language,
        set_origin_language,
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
    if settings.target_language == settings.origin_language {
        settings.origin_language = "Automatic".to_string();
    }
}

#[tauri::command]
fn set_origin_language(value: String, settings: State<'_, SharedSettings>) {
    let mut settings = settings.lock().unwrap();
    settings.origin_language = value;
    if settings.origin_language == settings.target_language {
        settings.target_language = if settings.origin_language != "English" {"English".to_string() } else { "Spanish".to_string() };
    }
}

#[tauri::command]
fn set_platform(value: String, settings: State<'_, SharedSettings>) {
    let mut settings = settings.lock().unwrap();
    settings.platform = value;
}