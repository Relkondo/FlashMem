use std::any::Any;
use std::sync::MutexGuard;
use std::thread;
use std::time::{Duration, Instant};
use std::io::Cursor;
use reqwest;
use translation_response::TranslationResponse;
use htmlentity::entity::{decode, ICodedDataTrait};
use image::RgbaImage;
use image::imageops::crop_imm;
use regex::Regex;
use crate::SettingsState;
use crate::utils::{get_google_language_code, get_platform_cropping, get_request_google_ocr, get_request_google_translate, get_tesseract_language_code};
use xcap::Monitor;
use crate::execute::vision_response::VisionResponse;
use base64::{Engine as _, engine::general_purpose};
use tesseract::Tesseract;
use crate::execute::saved_sub::SavedSub;
use crate::execute::tesseract_originated_error::TesseractOriginatedError;

mod translation_response;
mod vision_response;
mod tesseract_originated_error;
pub(crate) mod saved_sub;

static API_KEY: &'static str = "AIzaSyAoTyGq4l6wdF3GFjyLHNdslpuQ7IHV96A";

pub(crate) fn execute(settings: MutexGuard<SettingsState>) -> SavedSub {
    println!("Executing FlashMem...");
    let total = Instant::now();
    let step1 = Instant::now();
    let screenshot = capture_screenshot().expect("Couldn't capture screenshot.");
    println!("Screenshot captured in {}ms.", step1.elapsed().as_millis());
    let step2 = Instant::now();
    let image_data = crop_screenshot(screenshot.clone(), settings.platform.as_str());
    println!("Screenshot cropped in {}ms.", step2.elapsed().as_millis());
    let step3 = Instant::now();
    let origin_text = execute_ocr(image_data, settings.platform.as_str(), &settings.origin_language).expect("Couldn't execute OCR.");
    println!("Text extracted in {}ms.", step3.elapsed().as_millis());
    let step4 = Instant::now();
    let formatted_text = format_text(origin_text, settings.platform.as_str());
    println!("Text formatted in {}ms.", step4.elapsed().as_millis());
    let step5 = Instant::now();
    let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build().expect("Could not build tokio::runtime.");
    let (translated_text, detected_source_language) = runtime.block_on(translate_text(formatted_text.clone(), settings.origin_language.as_str(), settings.target_language.as_str())).expect("Couldn't translate text.");
    println!("Text translated in {}ms.", step5.elapsed().as_millis());
    let clean_translation: String;
    if detected_source_language == Some(get_google_language_code(settings.target_language.as_str()).to_string()) {
        clean_translation = translated_text.trim().to_string();
        println!("Detected source language is the same as the target language: {}. Skipping truncate.", &settings.target_language);
    } else {
        let step6 = Instant::now();
        clean_translation = truncate_translation(&formatted_text, &translated_text);
        println!("Translation cleaned in {}ms.", step6.elapsed().as_millis());
    }
    println!("TOTAL ELAPSED: {}ms.", total.elapsed().as_millis());
    println!("Sending the following notification:\n{}", &clean_translation);
    SavedSub{ original_text: formatted_text,
        translated_text: clean_translation,
        detected_source_language: detected_source_language.unwrap_or("".to_string())
    }
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

fn crop_screenshot(screenshot: RgbaImage, platform: &str) -> RgbaImage {
    println!("Cropping...");
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
    cropped_image
}

fn encode_as_tiff(image: &RgbaImage) -> Result<Vec<u8>, image::ImageError> {
    let mut bytes_tiff: Vec<u8> = Vec::new();
    image.write_to(&mut Cursor::new(&mut bytes_tiff), image::ImageOutputFormat::Tiff)?;
    Ok(bytes_tiff)
}

fn encode_as_webp(image: &RgbaImage) -> Result<Vec<u8>, image::ImageError> {
    let mut bytes: Vec<u8> = Vec::new();
    image.write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::WebP)?;
    Ok(bytes)
}

async fn execute_google_vision_ocr(file: &RgbaImage, lang: &String) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("https://vision.googleapis.com/v1/images:annotate?key={}", API_KEY);
    let b64 = general_purpose::STANDARD.encode(encode_as_webp(file)?);
    let request: serde_json::Value = get_request_google_ocr(b64, lang);
    println!("Calling Google Vision...");
    let now = Instant::now();
    let response = reqwest::Client::new()
        .post(&url)
        .json(&request)
        .send()
        .await?;
    let response_body = response.text().await?;
    println!("Google Vision response received in {}ms.", now.elapsed().as_millis());
    println!("Extracting json...");
    let json_response: VisionResponse = serde_json::from_str(&response_body)?;
    println!("json extracted...");
    if let Some(text) = json_response.responses.get(0) {
        if let Some(text_annotations) = &text.fullTextAnnotation {
            println!("Decoding...");
            let bytes = text_annotations.text.to_owned().into_bytes();
            let decoded_response = decode(&bytes).to_string().expect("Couldn't decode response.");
            println!("Decoded translation: {}", decoded_response);
            Ok(decoded_response)
        } else {
            Err("No text annotations found in response.".into())
        }
    } else {
        Err("No text found in response.".into())
    }
}

fn get_tesseract_result(file: &RgbaImage, lang: &String) -> Result<String, TesseractOriginatedError> {
    let code = get_tesseract_language_code(lang.as_str());
    let tesseract: Tesseract = Tesseract::new(None, Some(code)).unwrap();
    Ok(tesseract.set_image_from_mem(&encode_as_tiff(file)?)?.set_source_resolution(264).get_text()?)
}

fn execute_tesseract_ocr(file: &RgbaImage, lang: &String) -> Option<String> {
    println!("Using Tesseract...");
    let result = get_tesseract_result(file, lang);
    match result {
        Ok(text) => {
            println!("Tesseract executed successfully. Result:");
            println!("{}", text);
            Some(text)
        }
        Err(e) => {
            eprintln!("Failed to execute Tesseract: {:?}", e);
            None
        }
    }
}

fn execute_ocr(image_data: RgbaImage, platform: &str, lang: &String) -> Option<String> {
    if platform == "YouTube" || platform == "VLC" {
        let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build().expect("Could not build tokio::runtime.");
        match runtime.block_on(execute_google_vision_ocr(&image_data, lang)) {
            Ok(text) => Some(text),
            Err(e) => {
                eprintln!("Failed to execute Google Vision: {:?}", e);
                execute_tesseract_ocr(&image_data, lang)
            }
        }
    } else {
        execute_tesseract_ocr(&image_data, lang)
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

async fn translate_text(text: String, origin_lang_code: &str, target_lang_code: &str) -> Result<(String, Option<String>), Box<dyn std::error::Error>> {
    let url = format!("https://translation.googleapis.com/language/translate/v2?key={}", API_KEY);
    println!("Calling Google Translate...");
    let response = reqwest::Client::new()
        .post(&url)
        .json(&get_request_google_translate(text, target_lang_code, origin_lang_code))
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
