// Copyright © 2025 Leon Stansfield

use macroquad::prelude::*;
use crate::scene::Scene;
use crate::globals;

pub struct BasicImageScene {
    scene_name: String,
    image: Texture2D,
}

impl BasicImageScene {
    pub async fn new(scene_name: String) -> Self {
        let image_path = globals::IMAGE_FILEPATH.lock().unwrap();
        let image = load_texture(image_path.as_str()).await.unwrap();

        Self {
            scene_name,
            image,
        }
    }
}

impl Scene for BasicImageScene {
    fn update(&mut self) {
        // No dynamic updates needed for this basic scene
    }

    fn draw(&mut self) {
        clear_background(BLACK);

        // Draw the image to cover the entire screen
        draw_texture_ex(
            &self.image,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );
    }

    fn get_name(&self) -> &str {
        &self.scene_name
    }
}