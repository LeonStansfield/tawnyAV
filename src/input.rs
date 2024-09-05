use crate::globals;
use crate::scene_manager::SceneManager;
use macroquad::prelude::*;

pub async fn handle_input(scene_manager: &mut SceneManager) {
    if is_key_pressed(KeyCode::Space) {
        let mut beat_detection_enabled = globals::BEAT_DETECTION_ENABLED.lock().unwrap();
        *beat_detection_enabled = !*beat_detection_enabled;
        println!("Beat detection enabled: {}", *beat_detection_enabled);
    }

    // Handle scene switching
    const DIGIT_KEYS: [KeyCode; 10] = [
        KeyCode::Key1, KeyCode::Key2, KeyCode::Key3, KeyCode::Key4, KeyCode::Key5,
        KeyCode::Key6, KeyCode::Key7, KeyCode::Key8, KeyCode::Key9, KeyCode::Key0,
    ];
    
    for (i, &key) in DIGIT_KEYS.iter().enumerate() {
        if is_key_pressed(key) {
            scene_manager.switch_scene(i);
        }
    }

    let mut is_fullscreen = globals::IS_FULLSCREEN.lock().unwrap();

    if is_key_pressed(KeyCode::F11) {
        *is_fullscreen = !*is_fullscreen;
        set_fullscreen(*is_fullscreen);
        if !*is_fullscreen {
            request_new_screen_size(854., 480.);
        }
    }
}
