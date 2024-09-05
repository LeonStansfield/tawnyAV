use macroquad::prelude::*;
use std::time::{Duration, Instant};

mod audio_processing;
mod scene;
mod scenes;
mod scene_manager;
mod input;
pub mod globals;

use scene_manager::SceneManager;

#[macroquad::main("TawnyAV")]
async fn main() {
    // Initialize window
    request_new_screen_size(*globals::SCREEN_WIDTH.lock().unwrap(), *globals::SCREEN_HEIGHT.lock().unwrap());
    let frame_duration = Duration::from_secs_f32(1.0 / 30.0);

    // Initialize audio processing
    let _stream = audio_processing::initialize_audio();

    // Create scenes
    let scenes = scenes::get_scenes().await;
    let mut scene_manager = SceneManager::new(scenes);

    loop {
        let frame_start = Instant::now();

        input::handle_input(&mut scene_manager).await;

        clear_background(WHITE);

        // Update and draw the current scene
        scene_manager.update();
        scene_manager.draw();

        next_frame().await;

        // Calculate frame time and sleep if necessary to maintain 30fps
        let frame_time = frame_start.elapsed();
        if frame_time < frame_duration {
            let sleep_duration = frame_duration - frame_time;
            std::thread::sleep(sleep_duration);
        }
    }
}