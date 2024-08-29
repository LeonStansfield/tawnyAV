use macroquad::prelude::*;
mod audio_processing;
mod scene;
mod scenes;
mod scene_manager;

use scene_manager::SceneManager;

#[macroquad::main("Shadertoy")]
async fn main() {
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
        if is_key_pressed(KeyCode::Key1) {
            scene_manager.switch_scene(0);
        }
        if is_key_pressed(KeyCode::Key2) {
            scene_manager.switch_scene(1);
        }

        next_frame().await
    }
}