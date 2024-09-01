use macroquad::prelude::*;

pub trait Scene {
    fn update(&mut self, audio_data: &[f32]);
    fn draw(&mut self);
}