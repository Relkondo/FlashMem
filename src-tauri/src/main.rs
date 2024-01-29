// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashSet;
use tauri::generate_context;
use std::sync::{Arc, Mutex};
use std::thread;
use rdev::{simulate, EventType, listen};
use std::sync::atomic::{AtomicBool, Ordering};

mod execute;

static IS_LISTENING: AtomicBool = AtomicBool::new(false);

fn main() {
  tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![start, stop, greet])
      .run(generate_context!())
      .expect("error while running tauri application");
}

#[tauri::command]
fn start() {
  IS_LISTENING.store(true, Ordering::SeqCst);
  thread::spawn(move || {
    let pressed_keys = Arc::new(Mutex::new(HashSet::new()));
    let pressed_keys_clone = Arc::clone(&pressed_keys);
      println!("Thread spawned.");
    match listen(move |event| {
      if IS_LISTENING.load(Ordering::SeqCst) {
        execute::execute(event, &pressed_keys_clone);
      } else {
        println!("Stop received");
        return;
      }
    }) {
      Ok(_) => println!("Listening for global keyboard events..."),
      Err(e) => println!("Error: {:?}", e),
    }
    println!("Passed listen.");
  });
}

#[tauri::command]
fn stop() {
  IS_LISTENING.store(false, Ordering::SeqCst);
  simulate(&EventType::KeyPress(rdev::Key::ControlRight)).unwrap();
}

#[tauri::command]
fn greet(name: &str) -> String {
  format!("Hello, {}!", name)
}
