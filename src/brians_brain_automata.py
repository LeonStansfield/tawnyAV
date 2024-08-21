import numpy as np
from scipy.signal import convolve2d
from scipy.ndimage import gaussian_filter
from PIL import Image
import pygame
from .scene import Scene

class BriansBrainAutomata:
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
        
        # Randomly initialize the grid with on and off cells
        random_grid = np.random.rand(*self.grid_size) < 0.2
        grid[random_grid] = 1
        
        return grid

    def update(self):
        kernel = np.array([[1, 1, 1], [1, 0, 1], [1, 1, 1]])
        neighbor_count = convolve2d(self.grid == 1, kernel, mode='same', boundary='wrap')
        
        new_grid = np.zeros_like(self.grid)
        new_grid[(self.grid == 1) & (neighbor_count == 2)] = 2  # On cell turns into dying cell (level 1)
        for i in range(2, 11):
            new_grid[(self.grid == i)] = (i + 1) % 11  # Dying cells transition through levels
        new_grid[(self.grid == 0) & (neighbor_count == 2)] = 1  # Off cell with exactly two on neighbors turns into on cell
        
        self.grid = new_grid

    def define_palette(self):
        return [
            (13, 27, 36),       # Off
            (217, 67, 80),      # On
            (229, 88, 131),     # Dying level 1
            (242, 109, 182),    # Dying level 2
            (191, 87, 186),     # Dying level 3
            (141, 65, 191),     # Dying level 4
            (81, 68, 153),      # Dying level 5
            (22, 71, 115),      # Dying level 6
            (48, 114, 153),     # Dying level 7
            (75, 158, 191),     # Dying level 8
            (37, 79, 95)        # Dying level 9
        ]

    def map_to_palette(self, value):
        return self.palette[value]

    def draw(self, screen, screen_size):
        color_grid = np.array([self.map_to_palette(value) for value in self.grid.flatten()]).reshape(self.grid_size[0], self.grid_size[1], 3)
        
        # Apply Gaussian blur
        color_grid = gaussian_filter(color_grid, sigma=0.6)
        
        surface = pygame.surfarray.make_surface(color_grid)
        
        scale_factor = screen_size[1] / self.grid_size[1]
        new_width = int(self.grid_size[0] * scale_factor)
        
        scaled_surface = pygame.transform.scale(surface, (new_width, screen_size[1]))
        
        x_offset = (screen_size[0] - new_width) // 2
        
        screen.fill((0, 0, 0))
        screen.blit(scaled_surface, (x_offset, 0))

class BriansBrainAutomataScene(Scene):
    def __init__(self, automata, name):
        self.name = name
        self.automata = automata

    def draw(self, screen, data):
        self.automata.update()
        self.automata.draw(screen, screen.get_size())
    
    def handle_beat(self):
        self.automata.grid = self.automata.initialize_grids(self.automata.image_path)
