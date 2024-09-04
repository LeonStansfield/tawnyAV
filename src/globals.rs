use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    pub static ref SCREEN_WIDTH: Mutex<f32> = Mutex::new(854.0);
    pub static ref SCREEN_HEIGHT: Mutex<f32> = Mutex::new(480.0);
    pub static ref RENDER_WIDTH: Mutex<f32> = Mutex::new(1920.0);
    pub static ref RENDER_HEIGHT: Mutex<f32> = Mutex::new(1080.0);
    pub static ref BEAT_DETECTED: Mutex<bool> = Mutex::new(false);
    pub static ref LOGO_FILEPATH: &'static str = "resources/wyr_logo.png";
}