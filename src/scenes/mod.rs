pub mod basic_scene;
pub mod colour_scene;
pub mod melt_scene;
pub mod slow_melt_scene;
pub mod glitch_melt_scene;
pub mod cloud_scene;

use crate::scene::Scene;

pub async fn get_scenes() -> Vec<Box<dyn Scene>> {
    // Scene list
    vec![
        Box::new(basic_scene::BasicScene::new().await),
        Box::new(colour_scene::ColourScene::new().await),
        Box::new(melt_scene::MeltScene::new().await),
        Box::new(slow_melt_scene::SlowMeltScene::new().await),
        Box::new(glitch_melt_scene::GlitchMeltScene::new().await),
        Box::new(cloud_scene::CloudScene::new().await),
    ]
}