use crate::globals::{BEAT_DETECTION_ENABLED, IMAGE_FILEPATH, SENSITIVITY};
use crate::scene_manager::SceneManager;
use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets::{self, ComboBox}};
use std::path::Path;

pub struct UI {
    pub visible: bool,
    pub image_filepath: String,
    last_known_global_filepath: String,
    show_help: bool,
    image_path_error: Option<String>,
}

impl UI {
    pub fn new() -> Self {
        let initial_filepath = IMAGE_FILEPATH.lock().unwrap().clone();
        Self {
            visible: true,
            image_filepath: initial_filepath.clone(),
            last_known_global_filepath: initial_filepath,
            show_help: false,
            image_path_error: None,
        }
    }

    pub fn toggle_visibility(&mut self) {
        self.visible = !self.visible;
    }

    pub async fn draw(&mut self, scene_manager: &mut SceneManager) {
        if !self.visible {
            return;
        }

        let mut reload_scenes = false;
        let window_width = 350.0;
        let window_height = 300.0;

        widgets::Window::new(hash!(), vec2(10.0, 10.0), vec2(window_width, window_height))
            .label("Settings")
            .titlebar(true)
            .movable(true)
            .ui(&mut *root_ui(), |ui| {
                ui.label(None, "Scene Selection");
                ui.separator();
                if !scene_manager.scenes.is_empty() {
                    let scene_labels: Vec<String> = (0..scene_manager.scenes.len())
                        .map(|i| format!("Scene {}", i + 1))
                        .collect();
                    let scene_label_slices: Vec<&str> = scene_labels.iter().map(|s| s.as_str()).collect();
                    let current_label = scene_labels.get(scene_manager.current_scene).map_or("Select Scene", |s| s.as_str());
                    ComboBox::new(hash!("scene_select"), &scene_label_slices)
                        .label(current_label)
                        .ui(ui, &mut scene_manager.current_scene);
                } else {
                    ui.label(None, "No scenes loaded.");
                }
                ui.separator();
                ui.label(None, "Toggle Beat Detection:");
                let mut beat_detection_enabled = *BEAT_DETECTION_ENABLED.lock().unwrap();
                ui.checkbox(hash!("beat_detect_check"), "", &mut beat_detection_enabled);
                *BEAT_DETECTION_ENABLED.lock().unwrap() = beat_detection_enabled;

                if beat_detection_enabled {
                    let mut sensitivity = *SENSITIVITY.lock().unwrap();
                    ui.label(None, "Processing Sensitivity:");
                    ui.slider(hash!("sensitivity_slider"), &format!("{:.2}", sensitivity), 0.1..5.0, &mut sensitivity);
                    *SENSITIVITY.lock().unwrap() = sensitivity;
                }
                ui.separator();
                ui.label(None, "Image Path:");
                ui.input_text(hash!("image_path_input"), "", &mut self.image_filepath);

                if let Some(err) = &self.image_path_error {
                    ui.label(None, err);
                } else {
                    let current_global_filepath = IMAGE_FILEPATH.lock().unwrap().clone();
                    if self.image_filepath != current_global_filepath && self.image_filepath != self.last_known_global_filepath {
                        ui.label(None, "Path changed! Press 'Reload Scene' to apply.");
                    }
                    if self.image_filepath == current_global_filepath {
                        self.last_known_global_filepath = current_global_filepath;
                    }
                }
                
                ui.separator();

                if ui.button(None, "Reload Scene") {
                    let trimmed_path = self.image_filepath.trim();
                    if !trimmed_path.is_empty() {
                        let path_obj = Path::new(trimmed_path);
                        if path_obj.exists() && path_obj.is_file() {
                            *IMAGE_FILEPATH.lock().unwrap() = trimmed_path.to_string();
                            self.last_known_global_filepath = trimmed_path.to_string();
                            self.image_path_error = None;
                            reload_scenes = true;
                        } else {
                            self.image_path_error = Some(format!("Error: File not found or not accessible at '{}'", trimmed_path));
                        }
                    } else {
                        self.image_path_error = Some("Error: Image path cannot be empty.".to_string());
                    }
                }
                ui.separator();
                ui.separator();
                ui.separator();
                ui.separator();

                let help_button_text = if self.show_help { "Hide Help" } else { "Show Help" };
                if ui.button(None, help_button_text) {
                    self.show_help = !self.show_help;
                }
            });

        if self.show_help {
            widgets::Window::new(hash!("help_window"), vec2(370.0, 10.0), vec2(300.0, 200.0))
                .label("Help")
                .titlebar(true)
                .movable(true)
                .ui(&mut *root_ui(), |ui| {
                    ui.separator();
                    ui.label(None, "--- Help ---");
                    ui.label(None, "Shortcuts:");
                    ui.label(None, "H: Show/Hide UI");
                    ui.label(None, "Up Arrow: Increase sensitivity");
                    ui.label(None, "Down Arrow: Decrease sensitivity");
                    ui.label(None, "Space: Toggle beat detection");
                    ui.label(None, "F11: Toggle fullscreen");
                    ui.label(None, "1-9: Change scene");
                });
        }

        if reload_scenes {
            scene_manager.reload_scenes().await;
        }
    }
}
