// Copyright © 2025 Leon Stansfield

use lazy_static::lazy_static;
use std::sync::Mutex;
use std::fs;
use std::collections::HashMap;

lazy_static! {
    pub static ref SCREEN_WIDTH: Mutex<f32> = Mutex::new(854.0);
    pub static ref SCREEN_HEIGHT: Mutex<f32> = Mutex::new(480.0);
    pub static ref RENDER_WIDTH: Mutex<f32> = Mutex::new(1920.0);
    pub static ref RENDER_HEIGHT: Mutex<f32> = Mutex::new(1080.0);
    pub static ref IS_FULLSCREEN: Mutex<bool> = Mutex::new(false);
    pub static ref BEAT_DETECTION_ENABLED: Mutex<bool> = Mutex::new(true);
    pub static ref BEAT_DETECTED: Mutex<bool> = Mutex::new(false);
    pub static ref SENSITIVITY: Mutex<f32> = Mutex::new(1.5);
    pub static ref IMAGE_FILEPATH: Mutex<String> = Mutex::new("resources/images/logo.png".to_string());
}

pub fn load_params_from_file(filepath: &str) {
    if let Ok(contents) = fs::read_to_string(filepath) {
        let params: HashMap<String, String> = contents
            .lines()
            .filter_map(|line| {
                let mut parts = line.splitn(2, '=');
                Some((parts.next()?.trim().to_string(), parts.next()?.trim().to_string()))
            })
            .collect();

        if let Some(screen_width) = params.get("SCREEN_WIDTH") {
            if let Ok(value) = screen_width.parse::<f32>() {
                *SCREEN_WIDTH.lock().unwrap() = value;
            } else {
                eprintln!("Failed to parse SCREEN_WIDTH. Using default: 854.0");
            }
        } else {
            eprintln!("SCREEN_WIDTH not found. Using default: 854.0");
        }

        if let Some(screen_height) = params.get("SCREEN_HEIGHT") {
            if let Ok(value) = screen_height.parse::<f32>() {
                *SCREEN_HEIGHT.lock().unwrap() = value;
            } else {
                eprintln!("Failed to parse SCREEN_HEIGHT. Using default: 480.0");
            }
        } else {
            eprintln!("SCREEN_HEIGHT not found. Using default: 480.0");
        }

        if let Some(render_width) = params.get("RENDER_WIDTH") {
            if let Ok(value) = render_width.parse::<f32>() {
                *RENDER_WIDTH.lock().unwrap() = value;
            } else {
                eprintln!("Failed to parse RENDER_WIDTH. Using default: 1920.0");
            }
        } else {
            eprintln!("RENDER_WIDTH not found. Using default: 1920.0");
        }

        if let Some(render_height) = params.get("RENDER_HEIGHT") {
            if let Ok(value) = render_height.parse::<f32>() {
                *RENDER_HEIGHT.lock().unwrap() = value;
            } else {
                eprintln!("Failed to parse RENDER_HEIGHT. Using default: 1080.0");
            }
        } else {
            eprintln!("RENDER_HEIGHT not found. Using default: 1080.0");
        }

        if let Some(is_fullscreen) = params.get("IS_FULLSCREEN") {
            if let Ok(value) = is_fullscreen.parse::<bool>() {
                *IS_FULLSCREEN.lock().unwrap() = value;
            } else {
                eprintln!("Failed to parse IS_FULLSCREEN. Using default: false");
            }
        } else {
            eprintln!("IS_FULLSCREEN not found. Using default: false");
        }

        if let Some(beat_detection_enabled) = params.get("BEAT_DETECTION_ENABLED") {
            if let Ok(value) = beat_detection_enabled.parse::<bool>() {
                *BEAT_DETECTION_ENABLED.lock().unwrap() = value;
            } else {
                eprintln!("Failed to parse BEAT_DETECTION_ENABLED. Using default: true");
            }
        } else {
            eprintln!("BEAT_DETECTION_ENABLED not found. Using default: true");
        }

        if let Some(sensitivity) = params.get("SENSITIVITY") {
            if let Ok(value) = sensitivity.parse::<f32>() {
                *SENSITIVITY.lock().unwrap() = value;
            } else {
                eprintln!("Failed to parse SENSITIVITY. Using default: 1.5");
            }
        } else {
            eprintln!("SENSITIVITY not found. Using default: 1.5");
        }

        if let Some(image_filepath) = params.get("IMAGE_FILEPATH") {
            *IMAGE_FILEPATH.lock().unwrap() = image_filepath.clone();
        } else {
            eprintln!("IMAGE_FILEPATH not found. Using default: resources/images/logo.png");
        }
    } else {
        eprintln!("Failed to read parameters file: {}. Using all default values.", filepath);
    }
}