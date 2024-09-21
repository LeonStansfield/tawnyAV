use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    pub static ref SCREEN_WIDTH: Mutex<f32> = Mutex::new(854.0);
    pub static ref SCREEN_HEIGHT: Mutex<f32> = Mutex::new(480.0);
    pub static ref RENDER_WIDTH: Mutex<f32> = Mutex::new(1920.0);
    pub static ref RENDER_HEIGHT: Mutex<f32> = Mutex::new(1080.0);
    pub static ref IS_FULLSCREEN: Mutex<bool> = Mutex::new(true);
    pub static ref BEAT_DETECTION_ENABLED: Mutex<bool> = Mutex::new(true);
    pub static ref BEAT_DETECTED: Mutex<bool> = Mutex::new(false);
    pub static ref SENSITIVITY: Mutex<f32> = Mutex::new(1.5);
    pub static ref IMAGE_FILEPATH: &'static str = "resources/images/wyr_logo.png";
    pub static ref VIDEO_FILEPATH: &'static str = "resources/videos/PXL_20240606_123856647.mp4";
}