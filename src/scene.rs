// Copyright © 2025 Leon Stansfield

pub trait Scene {
    fn update(&mut self);
    fn draw(&mut self);
    fn get_name(&self) -> &str;
}