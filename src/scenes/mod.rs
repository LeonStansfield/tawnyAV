pub mod image_shader_scene;

use crate::scene::Scene;
use std::fs;

pub async fn get_scenes() -> Vec<Box<dyn Scene>> {
    let mut scenes: Vec<Box<dyn Scene>> = Vec::new();
    let shader_dir = "resources/shaders";
    
    if let Ok(entries) = fs::read_dir(shader_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("glsl") {
                if let Some(shader_path) = path.to_str() {
                    if let Some(file_stem) = path.file_stem().and_then(|s| s.to_str()) {
                        scenes.push(Box::new(image_shader_scene::ShaderScene::new(file_stem.to_string(), shader_path).await) as Box<dyn Scene>);
                    }
                }
            }
        }
    }
    
    scenes
}
