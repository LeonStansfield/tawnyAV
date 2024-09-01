pub mod basic_scene;
pub mod colour_scene;
pub mod reaction_diffusion_scene;

use crate::scene::Scene;

pub async fn get_scenes() -> Vec<Box<dyn Scene>> {
    // Scene list
    vec![
        Box::new(basic_scene::BasicScene::new().await),
        Box::new(colour_scene::ColourScene::new().await),
        Box::new(reaction_diffusion_scene::ReactionDiffusionScene::new().await),
    ]
}