use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui};
use crate::globals::{BEAT_DETECTION_ENABLED, SENSITIVITY};

pub struct UI {
    pub visible: bool,
}

impl UI {
    pub fn new() -> Self {
        Self { visible: true }
    }

    pub fn toggle_visibility(&mut self) {
        self.visible = !self.visible;
    }

    pub fn draw(&self) {
        if self.visible {
            root_ui().window(
                hash!(),
                vec2(10.0, 10.0),
                vec2(300.0, 250.0),
                |ui| {
                    ui.separator();

                    // Beat detection toggle
                    let mut beat_detection_enabled = *BEAT_DETECTION_ENABLED.lock().unwrap();
                    ui.label(None, "Settings:");
                    ui.checkbox(hash!(), "Enable Beat Detection", &mut beat_detection_enabled);
                    *BEAT_DETECTION_ENABLED.lock().unwrap() = beat_detection_enabled;

                    ui.separator();

                    // Sensitivity slider
                    let mut sensitivity = *SENSITIVITY.lock().unwrap();
                    ui.slider(hash!(), "Sensitivity", 0.1..10.0, &mut sensitivity);
                    *SENSITIVITY.lock().unwrap() = sensitivity;

                    ui.separator();

                    // Display program instructions
                    ui.label(None, "Shortcuts:");
                    ui.label(None, "H: Toggle UI");
                    ui.label(None, "Up Arrow: Increase sensitivity");
                    ui.label(None, "Down Arrow: Decrease sensitivity");
                    ui.label(None, "Space: Toggle beat detection");
                    ui.label(None, "1-9: Change scene");
                },
            );
        }
    }
}