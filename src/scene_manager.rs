use crate::scene::Scene;

pub struct SceneManager {
    scenes: Vec<Box<dyn Scene>>,
    current_scene: usize,
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

    pub fn update(&mut self, audio_data: &[f32]) {
        self.scenes[self.current_scene].update(audio_data);
    }

    pub fn draw(&mut self) {
        self.scenes[self.current_scene].draw();
    }
}