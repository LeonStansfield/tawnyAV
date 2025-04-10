// Copyright © 2025 Leon Stansfield

use crate::scene::Scene;
use crate::scenes;

pub struct SceneManager {
    pub scenes: Vec<Box<dyn Scene>>,
    pub current_scene: usize,
}

impl SceneManager {
    pub fn new(scenes: Vec<Box<dyn Scene>>) -> Self {
        Self {
            scenes,
            current_scene: 0,
        }
    }

    pub fn switch_scene(&mut self, index: usize) {
        if index < self.scenes.len() {
            self.current_scene = index;
        }
    }

    pub fn update(&mut self) {
        self.scenes[self.current_scene].update();
    }

    pub fn draw(&mut self) {
        self.scenes[self.current_scene].draw();
    }

    pub async fn reload_scenes(&mut self) {
        print!("Reloading scenes...");
        self.scenes = scenes::get_scenes().await;
        self.current_scene = 0; // Reset to the first scene
    }
}