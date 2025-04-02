pub mod shader_scene;
pub mod image_scene;
pub mod cellular_automata_scene;

use crate::scene::Scene;
use std::fs;
use std::io::{self, BufRead};

pub async fn get_scenes() -> Vec<Box<dyn Scene>> {
    let mut scenes: Vec<Box<dyn Scene>> = Vec::new();
    let shader_dir = "resources/shaders";
    let cellular_automata_dir = "resources/cellular_automata";

    // Load shader scenes
    if let Ok(entries) = fs::read_dir(shader_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("glsl") {
                if let Some(shader_path) = path.to_str() {
                    if let Some(file_stem) = path.file_stem().and_then(|s| s.to_str()) {
                        scenes.push(Box::new(shader_scene::ShaderScene::new(file_stem.to_string(), shader_path).await) as Box<dyn Scene>);
                    }
                }
            }
        }
    }

    // Load cellular automata scenes
    if let Ok(entries) = fs::read_dir(cellular_automata_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("txt") {
                if let Some(file_path) = path.to_str() {
                    if let Ok(file) = fs::File::open(file_path) {
                        let reader = io::BufReader::new(file);
                        let mut rules = String::new();
                        let mut alive_color = String::new();
                        let mut dead_color = String::new();
                        let mut spawn_chance = 0.5;

                        for line in reader.lines().flatten() {
                            if line.starts_with("B") {
                                rules = line;
                            } else if line.starts_with("A:") {
                                alive_color = line[2..].to_string();
                            } else if line.starts_with("D:") {
                                dead_color = line[2..].to_string();
                            } else if line.starts_with("SC:") {
                                spawn_chance = line[3..].parse::<f32>().unwrap_or(0.5).clamp(0.0, 1.0);
                            }
                        }

                        if !rules.is_empty() && !alive_color.is_empty() && !dead_color.is_empty() {
                            if let Some(file_stem) = path.file_stem().and_then(|s| s.to_str()) {
                                scenes.push(Box::new(cellular_automata_scene::CellularAutomataScene::new(
                                    file_stem.to_string(),
                                    rules,
                                    &alive_color,
                                    &dead_color,
                                    spawn_chance,
                                ).await) as Box<dyn Scene>);
                            }
                        }
                    }
                }
            }
        }
    }

    // Add a basic image scene
    scenes.push(Box::new(image_scene::BasicImageScene::new("Image Scene".to_string()).await) as Box<dyn Scene>);

    scenes
}