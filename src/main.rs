use rdev::{listen, Event, EventType, Key};
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use chrono::Local;

fn handle_event(event: Event, pressed_keys: &Arc<Mutex<HashSet<Key>>>) {
    let mut keys = pressed_keys.lock().unwrap();
    match event.event_type {
        EventType::KeyPress(key) => {
            keys.insert(key);
            if keys.contains(&Key::ControlLeft) && keys.contains(&Key::KeyG) {
                println!("Ctrl+G pressed!");
                capture_screenshot();
            }
        }
        EventType::KeyRelease(key) => {
            keys.remove(&key);
        }
        _ => (),
    }
}

fn capture_screenshot() {
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
                break;
            }
            Err(error) => {
                if error.kind() == std::io::ErrorKind::WouldBlock {
                    thread::sleep(Duration::from_millis(100));
                } else {
                    println!("Error: {}", error);
                    break;
                }
            }
        }
    }
}


fn main() {
    let pressed_keys = Arc::new(Mutex::new(HashSet::new()));
    let pressed_keys_clone = Arc::clone(&pressed_keys);
    match listen(move |event| handle_event(event, &pressed_keys_clone)) {
        Ok(_) => println!("Listening for global keyboard events..."),
        Err(e) => println!("Error: {:?}", e),
    }
}
