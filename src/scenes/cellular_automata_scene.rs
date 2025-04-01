use macroquad::prelude::*;
use crate::scene::Scene;
use crate::globals;

pub struct CellularAutomataScene {
    scene_name: String,
    grid: Vec<Vec<bool>>,
    rows: usize,
    cols: usize,
    cell_size: f32,
}

impl CellularAutomataScene {
    pub fn new(scene_name: String) -> Self {
        let cell_size = 4.0;

        // Calculate rows and columns based on RENDER_WIDTH and RENDER_HEIGHT
        let render_width = *globals::RENDER_WIDTH.lock().unwrap();
        let render_height = *globals::RENDER_HEIGHT.lock().unwrap();
        let rows = (render_height / cell_size) as usize;
        let cols = (render_width / cell_size) as usize;

        let grid = Self::randomize_grid(rows, cols);

        Self {
            scene_name,
            grid,
            rows,
            cols,
            cell_size,
        }
    }

    fn randomize_grid(rows: usize, cols: usize) -> Vec<Vec<bool>> {
        (0..rows)
            .map(|_| (0..cols).map(|_| rand::gen_range(0, 2) == 1).collect())
            .collect()
    }

    fn update_grid(&mut self) {
        let mut new_grid = self.grid.clone();

        for row in 0..self.rows {
            for col in 0..self.cols {
                let live_neighbors = self.count_live_neighbors(row, col);

                if self.grid[row][col] {
                    // Cell dies unless it has 2 or 3 live neighbors
                    new_grid[row][col] = live_neighbors == 2 || live_neighbors == 3;
                } else {
                    // Dead cell becomes alive if it has exactly 3 live neighbors
                    new_grid[row][col] = live_neighbors == 3;
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

                if self.grid[neighbor_row][neighbor_col] {
                    count += 1;
                }
            }
        }

        count
    }
}

impl Scene for CellularAutomataScene {
    fn update(&mut self) {
        // Check for beat detection and randomize grid if a beat is detected
        if *globals::BEAT_DETECTED.lock().unwrap() {
            self.grid = Self::randomize_grid(self.rows, self.cols);
            let mut beat_detected = globals::BEAT_DETECTED.lock().unwrap();
            *beat_detected = false;
        }

        // Update the grid for the next generation
        self.update_grid();
    }

    fn draw(&mut self) {
        clear_background(BLACK);

        // Scale the grid to fit the screen dimensions
        let scale_x = screen_width() / (self.cols as f32 * self.cell_size);
        let scale_y = screen_height() / (self.rows as f32 * self.cell_size);

        for row in 0..self.rows {
            for col in 0..self.cols {
                if self.grid[row][col] {
                    draw_rectangle(
                        col as f32 * self.cell_size * scale_x,
                        row as f32 * self.cell_size * scale_y,
                        self.cell_size * scale_x,
                        self.cell_size * scale_y,
                        WHITE,
                    );
                }
            }
        }
    }

    fn get_name(&self) -> &str {
        &self.scene_name
    }
}