// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::generate_context;
use std::sync::atomic::{AtomicBool, Ordering};

mod execute;

static IS_LISTENING: AtomicBool = AtomicBool::new(false);
static IS_RUNNING: AtomicBool = AtomicBool::new(false);

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![activate, deactivate, execute, greet])
        .run(generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn execute() {
    if IS_LISTENING.load(Ordering::SeqCst) && !IS_RUNNING.load(Ordering::SeqCst) {
        IS_RUNNING.store(true, Ordering::SeqCst);
        execute::execute();
        IS_RUNNING.store(false, Ordering::SeqCst);
    } else if !IS_LISTENING.load(Ordering::SeqCst) {
        println!("Received signal but FlashMem is not activated.");
    } else {
        println!("Received signal but FlashMem is already running.");
    }

}

#[tauri::command]
fn activate() {
    IS_LISTENING.store(true, Ordering::SeqCst);
    println!("Activated FlashMem.");
}

#[tauri::command]
fn deactivate() {
    IS_LISTENING.store(false, Ordering::SeqCst);
    println!("Deactivated FlashMem.");
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
