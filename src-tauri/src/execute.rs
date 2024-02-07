use std::any::Any;
use std::process::Command;
use std::sync::MutexGuard;
use std::thread;
use std::time::{Duration};
use chrono::Local;
use reqwest;
use serde_json::json;
use translation_response::TranslationResponse;
use htmlentity::entity::{decode, ICodedDataTrait};
use image::{RgbaImage};
use image::imageops::crop_imm;
use regex::Regex;
use crate::SettingsState;
use crate::utils::{get_language_code, get_platform_cropping};
use xcap::Monitor;
use crate::execute::vision_response::VisionResponse;

mod translation_response;
mod vision_response;

static FOOTER_START: &'static str = "[Detected Source Language:";
static CROPPED_PATH: &'static str = "assets/cropped/";
static API_KEY: &'static str = "AIzaSyAoTyGq4l6wdF3GFjyLHNdslpuQ7IHV96A";

pub(crate) fn execute(settings: MutexGuard<SettingsState>) -> String {
    println!("Executing FlashMem...");
    let screenshot = capture_screenshot().expect("Couldn't capture screenshot.");
    let filename = crop_screenshot(screenshot.clone(), settings.platform.as_str());
    let origin_text = execute_ocr(filename, settings.platform.as_str()).expect("Couldn't execute OCR.");
    let formatted_text = format_text(origin_text, settings.platform.as_str());
    let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build().expect("Could not build tokio::runtime.");
    let (translated_text, detected_source_language) = runtime.block_on(translate_text(formatted_text.clone(), get_language_code(settings.target_language.as_str()))).expect("Couldn't translate text.");
    let clean_translation = truncate_translation(&formatted_text, &translated_text);
    let notification = format_notification(&clean_translation, detected_source_language);
    println!("Sending the following notification:\n{}", notification);
    notification
}

fn capture_screenshot() -> Option<RgbaImage> {
    let monitors = Monitor::all().unwrap();
    for monitor in monitors {
        if monitor.is_primary() {
            loop {
                match monitor.capture_image() {
                    Ok(screenshot) => {
                        println!("Screenshot successful!");
                        return Some(screenshot);
                    }
                    Err(error) => {
                        if error.type_id() == std::io::ErrorKind::WouldBlock.type_id() {
                            println!("Blocked while capturing frame. Trying again...");
                            thread::sleep(Duration::from_millis(100));
                        } else {
                            println!("Error: {}", error);
                            return None;
                        }
                    }
                }
            }
        }
    }
    println!("Error: No primary screen found");
    return None;
}

fn crop_screenshot(screenshot: RgbaImage, platform: &str) -> String {
    let (width, height) = screenshot.dimensions();
    let (x_ratio, y_ratio, width_ratio, height_ratio) = get_platform_cropping(platform);
    let top = (height as f64 * y_ratio) as u32;
    let cropped_height = (height as f64 * height_ratio) as u32;
    let left = (width as f64 * x_ratio) as u32;
    let cropped_width = (width as f64 * width_ratio) as u32;
    if left + cropped_width > width || top + cropped_height > height {
        println!("Cropping dimensions are out of bounds.");
    }
    let cropped_image = crop_imm(&screenshot, left, top, cropped_width, cropped_height).to_image();
    let filename = format!("{}cropped_{}.png", CROPPED_PATH, Local::now().format("%Y%m%d_%H%M%S"));
    cropped_image.save(filename.to_owned()).unwrap();
    println!("Cropped screenshot saved as {}", filename);
    filename
}

async fn execute_google_vision_ocr(file: String) -> Option<String> {
    let url = format!("https://vision.googleapis.com/v1/images:annotate?key={}", API_KEY);
    println!("Calling Google Vision...");
    let response = reqwest::Client::new()
        .post(&url)
        .json(&json!({
            "requests": [
                {
                  "image": {
                    "content": file
                  },
                  "features": [
                    {
                      "type": "TEXT_DETECTION"
                    }
                  ]
                }
            ]
        }))
        .send()
        .await
        .expect("Failed to send request.");
    let response_body = response.text().await?;
    let json_response: VisionResponse = serde_json::from_str(&response_body)?;
    if let Some(text) = json_response.responses.get(0) {
        if let Some(text_annotations) = text.fullTextAnnotation() {
            let content = text_annotations.text.to_owned();
            println!("Received response:\n{}", content);
            Some(content)
        } else {
            eprintln!("No text annotations found in response.");
            None
        }
    } else {
        eprintln!("No text found in response.");
        None
    }
}

fn execute_tesseract_ocr(filename: String) -> Option<String> {
    let ocr_result = "assets/ocr_result";
    let ocr_result_txt = ocr_result.to_owned() + ".txt";
    thread::sleep(Duration::from_millis(150));
    let output_result = Command::new("tesseract")
        .arg(filename.to_owned())
        .arg(ocr_result.to_owned())
        .output();
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

fn execute_ocr(filename: String, platform: &str) -> Option<String> {
    if platform == "YouTube" || platform == "VLC" {
        execute_google_vision_ocr(filename)
    } else {
        execute_tesseract_ocr(filename)
    }
}

fn valid_end_sentence(text: &str) -> bool {
    text.ends_with('.') || text.ends_with('!') || text.ends_with('?') || text.ends_with(':')
}

fn line_is_invalid(text: &str) -> bool {
    let len = text.len();
    let mut numeric_nb = 0;
    let mut alpha_nb = 0;
    for c in text.chars() {
        if c == '©' || c == '©' || c == '™' {
            return false;
        } else if c.is_alphabetic() {
            alpha_nb += 1;
        } else if c.is_numeric() {
            numeric_nb += 1;
        }
    }
    len == 0 || len == numeric_nb || alpha_nb == 0 || numeric_nb + alpha_nb <= text.len() / 2 ||
        (len < 5 && !valid_end_sentence(text))
}

fn is_title_line(text: &str, platform: &str) -> bool {
    platform == "Amazon Prime Video" &&
        Regex::new(r"\w+ \d+, Ep. \d+ \wpisode \d+").unwrap().is_match(text)
}

fn is_valid_time_format(time: &str) -> bool {
    let re = Regex::new(r"^\d{2}:\d{2}$").unwrap();
    re.is_match(time) && time.trim().split(':').all(|part| part.parse::<u32>().is_ok())
}

fn format_text(text: String, platform: &str) -> String {
    let lines = text.split("\n").map(|line| line.trim()).collect::<Vec<&str>>();
    let mut result = String::new();
    let mut i = 0;
    while i < lines.len() && !is_valid_time_format(lines[i]) {
        if !line_is_invalid(lines[i]) {
            if is_title_line(lines[i], platform) {
                result = String::new();
            } else {
                result.push_str(&*lines[i].replace('|', "I"));
                result.push('\n');
            }
        }
        i += 1;
    }
    println!("Text after cleaning: {}", result);
    result
}

async fn translate_text(text: String, target_language: &str) -> Result<(String, Option<String>), Box<dyn std::error::Error>> {
    let url = format!("https://translation.googleapis.com/language/translate/v2?key={}", API_KEY);

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
