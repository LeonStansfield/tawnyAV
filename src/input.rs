// Copyright © 2025 Leon Stansfield

use crate::globals;
use crate::scene_manager::SceneManager;
use macroquad::prelude::*;

pub async fn handle_input(scene_manager: &mut SceneManager) {
    // Handle beat detection toggle
    if is_key_pressed(KeyCode::Space) {
        let mut beat_detection_enabled = globals::BEAT_DETECTION_ENABLED.lock().unwrap();
        *beat_detection_enabled = !*beat_detection_enabled;
        println!("Beat detection enabled: {}", *beat_detection_enabled);
    }

    // Handle sensitivity adjustment
    if is_key_pressed(KeyCode::Up) {
        let mut sensitivity = globals::SENSITIVITY.lock().unwrap();
        *sensitivity += 0.05;
        if *sensitivity > 5.0 {
            *sensitivity = 5.0;
        }
        println!("Increased sensitivity to {}", *sensitivity);
    }

    if is_key_pressed(KeyCode::Down) {
        let mut sensitivity = globals::SENSITIVITY.lock().unwrap();
        *sensitivity -= 0.05;
        if *sensitivity < 0.1 {
            *sensitivity = 0.1;
        }
        println!("Decreased sensitivity to {}", *sensitivity);
    }

    // Handle scene switching with number keys
    const DIGIT_KEYS: [KeyCode; 10] = [
        KeyCode::Key1, KeyCode::Key2, KeyCode::Key3, KeyCode::Key4, KeyCode::Key5,
        KeyCode::Key6, KeyCode::Key7, KeyCode::Key8, KeyCode::Key9, KeyCode::Key0,
    ];
    
    for (i, &key) in DIGIT_KEYS.iter().enumerate() {
        if is_key_pressed(key) {
            scene_manager.switch_scene(i);
        }
    }

    // Handle scene switching with left and right arrow keys
    if is_key_pressed(KeyCode::Right) {
        let next_scene = (scene_manager.current_scene + 1) % scene_manager.scenes.len();
        scene_manager.switch_scene(next_scene);
    }

    if is_key_pressed(KeyCode::Left) {
        let prev_scene = if scene_manager.current_scene == 0 {
            scene_manager.scenes.len() - 1
        } else {
            scene_manager.current_scene - 1
        };
        scene_manager.switch_scene(prev_scene);
    }

    // Handle fullscreen toggle
    let mut is_fullscreen = globals::IS_FULLSCREEN.lock().unwrap();

    if is_key_pressed(KeyCode::F11) {
        *is_fullscreen = !*is_fullscreen;
        set_fullscreen(*is_fullscreen);
        if !*is_fullscreen {
            request_new_screen_size(854., 480.);
        }
    }
}
