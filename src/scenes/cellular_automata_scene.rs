use macroquad::prelude::*;
use crate::scene::Scene;
use crate::globals;

pub struct CellularAutomataScene {
    scene_name: String,
    birth_rule: Vec<usize>,
    survival_rule: Vec<usize>,
    max_life: u8,
    image: Image,
    threshold: u8,
    grid: Vec<Vec<u8>>,
    rows: usize,
    cols: usize,
    cell_size: f32,
    alive_color: Color,
    dead_color: Color,
    spawn_chance: f32,
}

impl CellularAutomataScene {
    pub async fn new(
        scene_name: String,
        ruleset: String,
        alive_color: &str,
        dead_color: &str,
        spawn_chance: f32,
    ) -> Self {
        let (birth_rule, survival_rule, max_life) = Self::parse_ruleset(&ruleset);
    
        let image_path = globals::IMAGE_FILEPATH.lock().unwrap();
        let image = load_image(image_path.as_str()).await.unwrap();
        let cell_size = 4.0;
    
        let render_width = *globals::RENDER_WIDTH.lock().unwrap();
        let render_height = *globals::RENDER_HEIGHT.lock().unwrap();
        let rows = (render_height / cell_size) as usize;
        let cols = (render_width / cell_size) as usize;
    
        let grid = Self::randomize_grid(rows, cols, spawn_chance);
        let threshold = 128; // Default threshold for pixel brightness
        let grid = Self::remove_image_pixels_from_grid(grid.clone(), &image, threshold);
    
        let alive_color = Self::parse_color(alive_color);
        let dead_color = Self::parse_color(dead_color);
    
        Self {
            scene_name,
            birth_rule,
            survival_rule,
            max_life,
            image,
            threshold,
            grid,
            rows,
            cols,
            cell_size,
            alive_color,
            dead_color,
            spawn_chance,
        }
    }

    fn parse_ruleset(ruleset: &str) -> (Vec<usize>, Vec<usize>, u8) {
        let mut birth_rule = Vec::new();
        let mut survival_rule = Vec::new();
        let mut max_life = 1; 

        let parts: Vec<&str> = ruleset.split('/').collect();

        for part in parts {
            if let Some(nums) = part.strip_prefix('B') {
                birth_rule = nums.chars().filter_map(|c| c.to_digit(10).map(|n| n as usize)).collect();
            } else if let Some(nums) = part.strip_prefix('S') {
                survival_rule = nums.chars().filter_map(|c| c.to_digit(10).map(|n| n as usize)).collect();
            } else if let Some(num_str) = part.strip_prefix('L') {
                max_life = num_str.parse::<u8>().unwrap_or(1);
                if max_life == 0 { max_life = 1; }
            }
        }

        (birth_rule, survival_rule, max_life)
    }

    // Modified randomize_grid to return u8 (0 or 1)
    fn randomize_grid(rows: usize, cols: usize, spawn_chance: f32) -> Vec<Vec<u8>> {
        (0..rows)
            .map(|_| (0..cols).map(|_| if rand::gen_range(0.0, 1.0) < spawn_chance { 1 } else { 0 }).collect()) // 0 or 1 based on spawn chance
            .collect()
    }

    // Modified remove_image_pixels_from_grid for u8 grid
    fn remove_image_pixels_from_grid(mut grid: Vec<Vec<u8>>, image: &Image, threshold: u8) -> Vec<Vec<u8>> {
        let image_width = image.width as usize;
        let image_height = image.height as usize;
        let pixels = &image.bytes;

        for row in 0..grid.len() {
            for col in 0..grid[row].len() {
                let image_x = (col as f32 / grid[row].len() as f32 * image_width as f32) as usize;
                let image_y = (row as f32 / grid.len() as f32 * image_height as f32) as usize;

                let pixel_index = (image_y.clamp(0, image_height -1) * image_width + image_x.clamp(0, image_width - 1)) * 4;

                // Check if pixel_index is within bounds before accessing
                if pixel_index + 3 < pixels.len() { // Check for RGBA
                    let r = pixels[pixel_index];
                    let g = pixels[pixel_index + 1];
                    let b = pixels[pixel_index + 2];
                    let brightness = (r as u16 + g as u16 + b as u16) / 3;

                    if brightness > threshold as u16 {
                        grid[row][col] = 0;
                    }
                } else {
                    println!("Warning: Calculated pixel index out of bounds at ({}, {}) -> ({}, {}) -> index {}", row, col, image_y, image_x, pixel_index);
                }
            }
        }

        grid
    }

    fn update_grid(&mut self) {
        let mut new_grid = self.grid.clone();
    
        for row in 0..self.rows {
            for col in 0..self.cols {
                let live_neighbors = self.count_live_neighbors(row, col);
                let current_life = self.grid[row][col];
    
                if current_life == 0 {
                    // Birth Rule
                    // Dead cell becomes alive (starts at max_life) if neighbor count matches birth rule
                    if self.birth_rule.contains(&live_neighbors) {
                        new_grid[row][col] = self.max_life;
                    } else {
                        new_grid[row][col] = 0; // Stays dead
                    }
                } else {
                    // Survival & Aging Rule
                    if self.survival_rule.contains(&live_neighbors) {
                        // Survive: Maintain current life level
                        new_grid[row][col] = current_life;
                    } else {
                        // Age: Decrease life level, eventually die
                        new_grid[row][col] = current_life.saturating_sub(1);
                    }
                }
            }
        }
    
        self.grid = new_grid;
    }
    
    fn count_live_neighbors(&self, row: usize, col: usize) -> usize {
        let mut count = 0;
    
        for dr in -1..=1 {
            for dc in -1..=1 {
                if dr == 0 && dc == 0 {
                    continue;
                }
    
                let neighbor_row = (row as isize + dr).rem_euclid(self.rows as isize) as usize;
                let neighbor_col = (col as isize + dc).rem_euclid(self.cols as isize) as usize;
    
                if self.grid[neighbor_row][neighbor_col] > 0 {
                    count += 1;
                }
            }
        }
    
        count
    }

    fn parse_color(color_str: &str) -> Color {
        let components: Vec<&str> = color_str.split(',').collect();
        if components.len() == 3 {
            let r = components[0].strip_prefix('R').unwrap_or("0").parse::<f32>().unwrap_or(0.0) / 255.0;
            let g = components[1].strip_prefix('G').unwrap_or("0").parse::<f32>().unwrap_or(0.0) / 255.0;
            let b = components[2].strip_prefix('B').unwrap_or("0").parse::<f32>().unwrap_or(0.0) / 255.0;
            Color::new(r, g, b, 1.0)
        } else {
            print!("Invalid color format: '{}'. Ensure colours are in the format R256,G256,B256\n", color_str);
            Color::new(0.0, 0.0, 0.0, 1.0)
        }
    }

    fn interpolate_color(start: Color, end: Color, t: f32) -> Color {
        let r = start.r + (end.r - start.r) * t;
        let g = start.g + (end.g - start.g) * t;
        let b = start.b + (end.b - start.b) * t;
        let a = start.a + (end.a - start.a) * t;
        Color::new(r, g, b, a)
    }
}

impl Scene for CellularAutomataScene {
    fn update(&mut self) {

        if *globals::BEAT_DETECTED.lock().unwrap() {
            self.grid = Self::randomize_grid(self.rows, self.cols, self.spawn_chance);
            let mut beat_detected = globals::BEAT_DETECTED.lock().unwrap();
            *beat_detected = false;
        }

        self.grid = Self::remove_image_pixels_from_grid(self.grid.clone(), &self.image, self.threshold);

        self.update_grid();
    }

    fn draw(&mut self) {
        clear_background(self.dead_color);
    
        let scale_x = screen_width() / (self.cols as f32 * self.cell_size);
        let scale_y = screen_height() / (self.rows as f32 * self.cell_size);
    
        for row in 0..self.rows {
            for col in 0..self.cols {
                let life_level = self.grid[row][col];
                if life_level > 0 {
                    let color = if self.max_life == 1 {
                        self.alive_color
                    } else {
                        let t = (life_level as f32 - 1.0) / (self.max_life as f32 - 1.0).max(1.0);
                        Self::interpolate_color(self.dead_color, self.alive_color, t)
                    };
    
                    draw_rectangle(
                        col as f32 * self.cell_size * scale_x,
                        row as f32 * self.cell_size * scale_y,
                        self.cell_size * scale_x,
                        self.cell_size * scale_y,
                        color,
                    );
                }
            }
        }
    }

    fn get_name(&self) -> &str {
        &self.scene_name
    }
}