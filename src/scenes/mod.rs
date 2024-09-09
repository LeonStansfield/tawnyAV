pub mod shader_scene;

use crate::scene::Scene;

pub async fn get_scenes() -> Vec<Box<dyn Scene>> {
    // Scene list
    vec![
        Box::new(shader_scene::ShaderScene::new("resources/shaders/basic.glsl").await),
        Box::new(shader_scene::ShaderScene::new("resources/shaders/colours.glsl").await),
        Box::new(shader_scene::ShaderScene::new("resources/shaders/melt.glsl").await),
        Box::new(shader_scene::ShaderScene::new("resources/shaders/slow_melt.glsl").await),
        Box::new(shader_scene::ShaderScene::new("resources/shaders/glitch_melt.glsl").await),
        Box::new(shader_scene::ShaderScene::new("resources/shaders/cloud.glsl").await),
        Box::new(shader_scene::ShaderScene::new("resources/shaders/parallax_noise.glsl").await),
    ]
}