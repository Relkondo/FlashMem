// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashSet;
use tauri::generate_context;
use std::sync::{Arc, Mutex};
use rdev::{EventType, Key, listen};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

mod execute;

static IS_LISTENING: AtomicBool = AtomicBool::new(false);
static IS_RUNNING: AtomicBool = AtomicBool::new(false);

fn main() {
    open_listening_channel();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![start_listening, stop_listening, greet])
        .run(generate_context!())
        .expect("error while running tauri application");
}

fn open_listening_channel() {
    thread::spawn(move || {
        println!("Opening global keyboard events listening channel...");
        let pressed_keys = Arc::new(Mutex::new(HashSet::new()));
        match listen(move |event| {
            let mut keys = pressed_keys.lock().unwrap();
            match event.event_type {
                EventType::KeyPress(key) => {
                    keys.insert(key);
                    if keys.contains(&Key::ControlLeft) && keys.contains(&Key::KeyG) {
                        println!("Ctrl+G pressed!");
                        if IS_LISTENING.load(Ordering::SeqCst) && !IS_RUNNING.load(Ordering::SeqCst) {
                            IS_RUNNING.store(true, Ordering::SeqCst);
                            execute::execute(event, &pressed_keys);
                            IS_RUNNING.store(false, Ordering::SeqCst);
                        }
                    }
                }
                EventType::KeyRelease(key) => {
                    keys.remove(&key);
                }
                _ => (),
            }
        }) {
            Ok(_) => println!("Channel for global keyboard events opened."),
            Err(e) => println!("Error while opening: {:?}", e),
        }
    });
}

#[tauri::command]
fn start_listening() {
    IS_LISTENING.store(true, Ordering::SeqCst);
}

#[tauri::command]
fn stop_listening() {
    IS_LISTENING.store(false, Ordering::SeqCst);
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
