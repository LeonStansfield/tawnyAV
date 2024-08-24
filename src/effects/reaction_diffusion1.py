import threading
import numpy as np
from PIL import Image
import pygame
from ..palette import Palette
from ..scene import Scene

class ReactionDiffusionScene1(Scene):
    def __init__(self, reaction_diffusion, name):
        self.name = name
        self.reaction_diffusion = reaction_diffusion

    def draw(self, screen, data):
        self.reaction_diffusion.update()
        self.reaction_diffusion.draw(screen, screen.get_size())
    
    def handle_beat(self):
        pre_simulation_steps = np.random.randint(0, 100)
        self.reaction_diffusion.A, self.reaction_diffusion.B = self.reaction_diffusion.initialize_grids(self.reaction_diffusion.image_path)
        for _ in range(pre_simulation_steps):
            self.reaction_diffusion.update()

class ReactionDiffusion1:
    def __init__(self, grid_size, dA, dB, feed, kill, image_path):
        self.grid_size = grid_size
        self.dA = dA
        self.dB = dB
        self.feed = feed
        self.kill = kill
        self.image_path = image_path
        self.A, self.B = self.initialize_grids(self.image_path)
        self.palette = self.define_palette()

    def initialize_grids(self, image_path):
        img = Image.open(image_path).convert('L')
        img_array = np.array(img, dtype=np.float32) / 255.0
        
        A = np.ones(self.grid_size, dtype=np.float32)
        B = np.zeros(self.grid_size, dtype=np.float32)
        
        B[img_array < 1.0] = 1.0
        for _ in range(np.random.randint(5, 500)):
            x, y = np.random.randint(0, self.grid_size[0]), np.random.randint(0, self.grid_size[1])
            size = np.random.randint(3, 5)
            B[max(0, x-size):min(self.grid_size[0], x+size), max(0, y-size):min(self.grid_size[1], y+size)] = 1.0
        
        return A, B

    def laplacian(self, Z):
        return (
            -4 * Z
            + np.roll(Z, 1, axis=0) + np.roll(Z, -1, axis=0)
            + np.roll(Z, 1, axis=1) + np.roll(Z, -1, axis=1)
        )

    def update(self):
        lapA = self.laplacian(self.A)
        lapB = self.laplacian(self.B)
        reaction = self.A * self.B**2
        
        self.A = np.clip(self.A + self.dA * lapA - reaction + self.feed * (1 - self.A), 0, 1)
        self.B = np.clip(self.B + self.dB * lapB + reaction - (self.kill + self.feed) * self.B, 0, 1)

    def define_palette(self):
        palette = Palette('resources/palette.txt')
        return palette.get_colors()
    
    def draw(self, screen, screen_size):
        C = (self.A - self.B) * (len(self.palette) - 1)  # Scale to the range of palette indices
        C = np.clip(C, 0, len(self.palette) - 1).astype(np.uint8)
        
        # Create a NumPy array for the palette
        palette_array = np.array(self.palette, dtype=np.uint8)
        
        # Map the values to the palette
        color_grid = palette_array[C]
        
        surface = pygame.surfarray.make_surface(color_grid)
        
        scale_factor = screen_size[1] / self.grid_size[1]
        new_width = int(self.grid_size[0] * scale_factor)
        
        scaled_surface = pygame.transform.smoothscale(surface, (new_width, screen_size[1]))
        
        x_offset = (screen_size[0] - new_width) // 2
        
        screen.fill((0, 0, 0))
        screen.blit(scaled_surface, (x_offset, 0))
