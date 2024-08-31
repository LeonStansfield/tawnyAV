pub mod basic_scene;
pub mod colour_scene;
pub mod game_of_life_scene;

use crate::scene::Scene;

pub async fn get_scenes() -> Vec<Box<dyn Scene>> {
    // Scene list
    vec![
        Box::new(basic_scene::BasicScene::new().await),
        Box::new(colour_scene::ColourScene::new().await),
        Box::new(game_of_life_scene::GameOfLifeScene::new().await),
    ]
}