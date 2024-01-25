mod translation_response;

use rdev::{listen, Event, EventType, Key};
use std::collections::HashSet;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use chrono::Local;
use reqwest;
use serde_json::json;
use translation_response::TranslationResponse;
use notify_rust::Notification;
use htmlentity::entity::{decode, ICodedDataTrait};

fn handle_event(event: Event, pressed_keys: &Arc<Mutex<HashSet<Key>>>) {
    let mut keys = pressed_keys.lock().unwrap();
    match event.event_type {
        EventType::KeyPress(key) => {
            keys.insert(key);
            if keys.contains(&Key::ControlLeft) && keys.contains(&Key::KeyG) {
                println!("Ctrl+G pressed!");
                let filename = capture_screenshot().expect("Couldn't capture screenshot.");
                let origin_text = execute_ocr(filename).expect("Couldn't execute OCR.");
                let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build().expect("Could not build tokio::runtime.");
                let (translated_text, detected_source_language) = runtime.block_on(translate_text(origin_text, "fr")).expect("Couldn't translate text.");
                let notification = format_notification(&translated_text, detected_source_language);
                send_notification("Translated Text", &notification).expect("Failed to send notification");
            }
        }
        EventType::KeyRelease(key) => {
            keys.remove(&key);
        }
        _ => (),
    }
}

fn capture_screenshot() -> Option<String> {
    // Screenshot capturing logic as before
    let display = scrap::Display::primary().expect("Couldn't find primary display.");
    let mut capturer = scrap::Capturer::new(display).expect("Couldn't begin capture.");
    let (w, h) = (capturer.width(), capturer.height());

    loop {
        match capturer.frame() {
            Ok(frame) => {
                let mut image_data = Vec::with_capacity(frame.len());
                for chunk in frame.chunks_exact(4) {
                    // Convert BGRA to RGBA
                    image_data.extend_from_slice(&[chunk[2], chunk[1], chunk[0], chunk[3]]);
                }
                let filename = format!("screenshot_{}.png", Local::now().format("%Y%m%d_%H%M%S"));
                let mut file = std::fs::File::create(&filename).expect("Couldn't create file.");
                let mut encoder = png::Encoder::new(&mut file, w as u32, h as u32);
                encoder.set_color(png::ColorType::Rgba); // Set the color type to RGBA
                encoder.set_depth(png::BitDepth::Eight); // Set the bit depth
                encoder.write_header().unwrap().write_image_data(&image_data).expect("Couldn't write image data.");
                println!("Screenshot saved as {}", filename);
                return Some(filename);
            }
            Err(error) => {
                if error.kind() == std::io::ErrorKind::WouldBlock {
                    thread::sleep(Duration::from_millis(100));
                } else {
                    println!("Error: {}", error);
                    return None;
                }
            }
        }
    }
}

fn execute_ocr(filename: String) -> Option<String> {
    println!("Starting Tesseract OCR...");
    let ocr_result = "ocr_result";
    let ocr_result_txt = ocr_result.to_owned() + ".txt";
    println!("Filename: {}, OCR result: {}", filename, ocr_result);
    thread::sleep(Duration::from_millis(150));
    let output_result = Command::new("tesseract")
        .arg(filename.to_owned())
        .arg(ocr_result.to_owned())
        .output();
    println!("Tesseract over.");
    match output_result {
        Ok(output) => {
            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                eprintln!("Standard Error: {}", stderr);
                None
            } else {
                let stdout = String::from_utf8_lossy(&output.stdout);
                println!("Standard Output: {}", stdout);
                println!("Tesseract executed successfully. Result:");
                let content = std::fs::read_to_string(ocr_result_txt.to_owned()).expect("Couldn't read file.");
                println!("{}", content);
                std::fs::remove_file(filename).expect("Couldn't delete file.");
                std::fs::remove_file(ocr_result_txt).expect("Couldn't delete file.");
                Some(content)
            }
        }
        Err(e) => {
            eprintln!("Failed to execute command: {:?}", e);
            None
        }
    }
}

async fn translate_text(text: String, target_language: &str) -> Result<(String, Option<String>), Box<dyn std::error::Error>> {
    let api_key = "AIzaSyAoTyGq4l6wdF3GFjyLHNdslpuQ7IHV96A"; // Replace with your API key
    let url = format!("https://translation.googleapis.com/language/translate/v2?key={}", api_key);

    println!("Calling Google Translate...");
    let response = reqwest::Client::new()
        .post(&url)
        .json(&json!({
            "q": text,
            "target": target_language
        }))
        .send()
        .await?;
    let response_body = response.text().await?;
    println!("Received response:\n{}", response_body);
    println!("Extracting json...");
    let translation_response: TranslationResponse = serde_json::from_str(&response_body)?;
    if let Some(translation) = translation_response.data.translations.get(0) {
        println!("Decoding...");
        let bytes = translation.translatedText.to_owned().into_bytes();
        let decoded_response = decode(&bytes).to_string().expect("Couldn't decode response.");
        println!("Decoded translation: {}", decoded_response);
        Ok((decoded_response.clone(), translation.detectedSourceLanguage.clone()))
    } else {
        Err("No translation found".into())
    }
}

fn format_notification(translated_text: &str, detected_source_language: Option<String>) -> String {
    let mut notification = translated_text.to_owned();
    if let Some(source_language) = detected_source_language {
        notification.push_str(&format!("\n[Detected Source Language: {:?}]", source_language));
    }
    notification
}

fn send_notification(summary: &str, body: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Sending the following notification:\n{}\n{}", summary, body);
    Notification::new()
        .summary(summary)
        .body(body)
        .show()?;
    println!("Notification sent.");
    Ok(())
}

fn main() {
    let pressed_keys = Arc::new(Mutex::new(HashSet::new()));
    let pressed_keys_clone = Arc::clone(&pressed_keys);
    match listen(move |event| handle_event(event, &pressed_keys_clone)) {
        Ok(_) => println!("Listening for global keyboard events..."),
        Err(e) => println!("Error: {:?}", e),
    }
}
