use rdev::{Event, EventType, Key, listen};
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
use image::GenericImageView;
use regex::Regex;

mod translation_response;

static TITLE: &'static str = "FlashMem Translated Sub";
static FOOTER_START: &'static str = "[Detected Source Language:";
static SCREENSHOT_PATH: &'static str = "assets/screenshots/";
static CROPPED_PATH: &'static str = "assets/cropped/";

fn handle_event(event: Event, pressed_keys: &Arc<Mutex<HashSet<Key>>>) {
    let mut keys = pressed_keys.lock().unwrap();
    match event.event_type {
        EventType::KeyPress(key) => {
            keys.insert(key);
            if keys.contains(&Key::ControlLeft) && keys.contains(&Key::KeyG) {
                println!("Ctrl+G pressed!");
                let filename = capture_screenshot().expect("Couldn't capture screenshot.");
                let cropped_file = crop_image(filename.as_str());
                let origin_text = execute_ocr(cropped_file).expect("Couldn't execute OCR.");
                let formatted_text = format_text(origin_text);
                let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build().expect("Could not build tokio::runtime.");
                let (translated_text, detected_source_language) = runtime.block_on(translate_text(formatted_text.clone(), "fr")).expect("Couldn't translate text.");
                let clean_translation = truncate_translation(&formatted_text, &translated_text);
                let notification = format_notification(&clean_translation, detected_source_language);
                send_notification(TITLE, &notification).expect("Failed to send notification");
            }
        }
        EventType::KeyRelease(key) => {
            keys.remove(&key);
        }
        _ => (),
    }
}

fn capture_screenshot() -> Option<String> {
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
                let filename = format!( "{}screenshot_{}.png", SCREENSHOT_PATH, Local::now().format("%Y%m%d_%H%M%S"));
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

fn crop_image(image_path: &str) -> String {
    let mut img = image::open(image_path).expect("Failed to open image");
    let (width, height) = img.dimensions();
    let top = (height as f64 * 0.6) as u32;
    let crop_height = (height as f64 * 0.35) as u32;
    let cropped_image = img.crop(0, top, width, crop_height);
    let cropped_filename = format!("{}cropped_{}", CROPPED_PATH, image_path.split("screenshot_").last().unwrap());
    cropped_image.save(cropped_filename.to_owned()).unwrap();
    std::fs::remove_file(image_path).expect("Couldn't delete file.");
    cropped_filename
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

fn valid_end_sentence(text: &str) -> bool {
    text.ends_with('.') || text.ends_with('!') || text.ends_with('?') || text.ends_with(':')
}

fn line_is_invalid(text: &str) -> bool {
    text.contains("©") || text.contains("®") || text.contains("™") ||
        text.contains("&") || text.chars().all(|c| c.is_numeric()) ||
        !text.chars().any(|c| c.is_alphabetic()) ||
        (text.len() < 5 && !valid_end_sentence(text))
}

fn is_valid_time_format(time: &str) -> bool {
    let re = Regex::new(r"^\d{2}:\d{2}$").unwrap();
    re.is_match(time) && time.trim().split(':').all(|part| part.parse::<u32>().is_ok())
}

fn format_text(text: String) -> String {
    println!("Checking for noise...");
    let lines = text.split("\n").map(|line| line.trim()).collect::<Vec<&str>>();
    let mut result = String::new();
    let mut i = 0;
    while i < lines.len() && (line_is_invalid(lines[i]) || lines[i].is_empty()) {
        i += 1;
    }
    while i < lines.len() && !is_valid_time_format(lines[i]) {
        if !line_is_invalid(lines[i]) {
            result.push_str(&*lines[i].replace('|', "I"));
            result.push('\n');
        } else if i + 1 < lines.len() && lines[i + 1].is_empty() {
            i += 1;
        }
        i += 1;
    }
    println!("Text after cleaning: {}", result);
    result
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

fn truncate_translation(untranslated: &str, translated: &str) -> String {
    println!("Checking again for noise by comparing translations...");
    let untranslated_words: Vec<&str> = untranslated.split_whitespace().collect();
    let translated_words: Vec<&str> = translated.split_whitespace().collect();
    let mut matching_sequence = 0;
    let mut result = String::new();
    let limit = 4;

    for word in &translated_words {
        if untranslated_words.contains(word) {
            matching_sequence += 1;
            if matching_sequence < limit {
                result.push_str(word);
                result.push(' ');
            } else if matching_sequence == limit {
                println!("Truncating noise at \"{}\" (removing previous {} words)...", word, limit);
                for _ in 0..(limit - 1) {
                    if let Some(non_matching_word) = result.split_whitespace().next_back() {
                        result.truncate(result.len() - non_matching_word.len() - 1);
                    }
                }
                return result.trim_end().to_string();
            }
        } else {
            matching_sequence = 0;
            result.push_str(word);
            result.push(' ');
        }
    }
    result.trim_end().to_string()
}

fn format_notification(translated_text: &str, detected_source_language: Option<String>) -> String {
    let mut notification = translated_text.to_owned();
    if let Some(source_language) = detected_source_language {
        notification.push_str(&format!("\n{} {:?}]", FOOTER_START, source_language));
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

fn execute() {
    let pressed_keys = Arc::new(Mutex::new(HashSet::new()));
    let pressed_keys_clone = Arc::clone(&pressed_keys);
    match listen(move |event| handle_event(event, &pressed_keys_clone)) {
        Ok(_) => println!("Listening for global keyboard events..."),
        Err(e) => println!("Error: {:?}", e),
    }
}

// fn cropping_all_images_test() {
//     let mut files = std::fs::read_dir(SCREENSHOT_PATH).unwrap();
//     while let Some(file) = files.next() {
//         let filename = file.unwrap().path().display().to_string();
//         if filename.contains("screenshot") {
//             crop_image(filename.as_str());
//         }
//     }
// }
