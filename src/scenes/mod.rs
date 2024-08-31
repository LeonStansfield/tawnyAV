pub mod basic_scene;
pub mod inverted_scene;

use crate::scene::Scene;

pub async fn get_scenes() -> Vec<Box<dyn Scene>> {
    vec![
        Box::new(basic_scene::BasicScene::new().await),
        Box::new(inverted_scene::InvertedScene::new().await),
        // Add more scenes here
    ]
}