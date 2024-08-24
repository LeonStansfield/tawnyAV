import threading
import numpy as np
from scipy.signal import convolve2d
from scipy.ndimage import gaussian_filter
from PIL import Image
import pygame
from concurrent.futures import ThreadPoolExecutor
from ..scene import Scene

class gameOfLife:
    def __init__(self, grid_size, image_path):
        self.grid_size = grid_size
        self.image_path = image_path
        self.grid = self.initialize_grids(self.image_path)
        self.palette = self.define_palette()

    def initialize_grids(self, image_path):
        img = Image.open(image_path).convert('L')
        img_array = np.array(img, dtype=np.float32) / 255.0
        
        grid = np.zeros(self.grid_size, dtype=np.int32)
        grid[img_array < 1.0] = 1
        
        # Use vectorized operations to randomly fill the grid
        random_fill = np.random.rand(*self.grid_size) < 0.5
        grid[random_fill] = 1
        
        return grid

    def update_chunk(self, new_grid, start_row, end_row):
        sub_grid = self.grid[start_row:end_row, :]
        total_neighbors = convolve2d(sub_grid, np.ones((3, 3)), mode='same', boundary='wrap') - sub_grid
        
        birth = (sub_grid == 0) & (total_neighbors == 3)
        survive = (sub_grid == 1) & ((total_neighbors == 2) | (total_neighbors == 3))
        
        new_grid[start_row:end_row, :] = birth | survive

    def update(self):
        new_grid = np.copy(self.grid)
        num_threads = 4
        chunk_size = self.grid_size[0] // num_threads

        with ThreadPoolExecutor(max_workers=num_threads) as executor:
            futures = []
            for i in range(num_threads):
                start_row = i * chunk_size
                end_row = (i + 1) * chunk_size if i != num_threads - 1 else self.grid_size[0]
                futures.append(executor.submit(self.update_chunk, new_grid, start_row, end_row))
            
            for future in futures:
                future.result()

        self.grid = new_grid

    def define_palette(self):
        return [(0, 0, 0), (171, 55, 112)]

    def map_to_palette(self, value):
        return self.palette[value]

    def draw(self, screen, screen_size):
        color_grid = np.zeros((self.grid_size[0], self.grid_size[1], 3), dtype=np.uint8)
        for i in range(self.grid_size[0]):
            for j in range(self.grid_size[1]):
                color_grid[i, j] = self.map_to_palette(self.grid[i, j])
        
        # Apply Gaussian blur
        color_grid = gaussian_filter(color_grid, sigma=0.6)

        surface = pygame.surfarray.make_surface(color_grid)
        
        scale_factor = screen_size[1] / self.grid_size[1]
        new_width = int(self.grid_size[0] * scale_factor)
        
        scaled_surface = pygame.transform.scale(surface, (new_width, screen_size[1]))
        
        x_offset = (screen_size[0] - new_width) // 2
        
        screen.fill((0, 0, 0))
        screen.blit(scaled_surface, (x_offset, 0))

class gameOfLifeScene(Scene):
    def __init__(self, cellular_automata, name):
        self.name = name
        self.cellular_automata = cellular_automata

    def draw(self, screen, data):
        self.cellular_automata.update()
        self.cellular_automata.draw(screen, screen.get_size())
    
    def handle_beat(self):
        self.cellular_automata.grid = self.cellular_automata.initialize_grids(self.cellular_automata.image_path)
