pub mod scene;

use crate::scene::Scene;

pub async fn get_scenes() -> Vec<Box<dyn Scene>> {
    // Scene list
    vec![
        Box::new(scene::SceneParams::new("resources/shaders/basic.glsl").await),
        Box::new(scene::SceneParams::new("resources/shaders/colours.glsl").await),
        Box::new(scene::SceneParams::new("resources/shaders/melt.glsl").await),
        Box::new(scene::SceneParams::new("resources/shaders/slow_melt.glsl").await),
        Box::new(scene::SceneParams::new("resources/shaders/glitch_melt.glsl").await),
        Box::new(scene::SceneParams::new("resources/shaders/cloud.glsl").await),
    ]
}