use macroquad::prelude::*;
mod audio_processing;
mod scene;
mod scenes;
mod scene_manager;

use scene_manager::SceneManager;

#[macroquad::main("TawnyAV")]
async fn main() {
    // Initialize window
    let mut is_fullscreen = false;
    request_new_screen_size(854., 480.);

    // Initialize audio processing
    let _stream = audio_processing::initialize_audio();

    // Create scenes
    let scenes = scenes::get_scenes().await;
    let mut scene_manager = SceneManager::new(scenes);

    loop {
        clear_background(WHITE);

        // Update and draw the current scene
        scene_manager.update(&[]); // Pass audio data here
        scene_manager.draw();

        // Handle scene switching (e.g., based on user input)
        const DIGIT_KEYS: [KeyCode; 10] = [
            KeyCode::Key1, KeyCode::Key2, KeyCode::Key3, KeyCode::Key4, KeyCode::Key5,
            KeyCode::Key6, KeyCode::Key7, KeyCode::Key8, KeyCode::Key9, KeyCode::Key0,
        ];
        
        for (i, &key) in DIGIT_KEYS.iter().enumerate() {
            if is_key_pressed(key) {
                scene_manager.switch_scene(i); // Scene indices are 0-based
            }
        }

        if is_key_pressed(KeyCode::F11) {
            is_fullscreen = !is_fullscreen;
            set_fullscreen(is_fullscreen);
            if !is_fullscreen {
                request_new_screen_size(854., 480.);
            }
        }

        next_frame().await
    }
}