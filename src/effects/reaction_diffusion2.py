import numpy as np
from PIL import Image
import pygame
from scipy.ndimage import gaussian_filter
from ..palette import Palette
from ..scene import Scene

class ReactionDiffusionScene2(Scene):
    def __init__(self, reaction_diffusion, name):
        self.name = name
        self.reaction_diffusion = reaction_diffusion

    def draw(self, screen, data):
        self.reaction_diffusion.update()
        self.reaction_diffusion.draw(screen, screen.get_size())
    
    def handle_beat(self):
        pre_simulation_steps = np.random.randint(0, 5)
        self.reaction_diffusion.u, self.reaction_diffusion.v = self.reaction_diffusion.initialize_grids(self.reaction_diffusion.image_path)
        for _ in range(pre_simulation_steps):
            self.reaction_diffusion.update()


class ReactionDiffusion2:
    def __init__(self, grid_size, dU, dV, epsilon, gamma, I, image_path, time_step=0.7):
        self.grid_size = grid_size
        self.dU = dU
        self.dV = dV
        self.epsilon = epsilon
        self.gamma = gamma
        self.I = I
        self.image_path = image_path
        self.u, self.v = self.initialize_grids(self.image_path)
        self.palette = self.define_palette()
        self.time_step = time_step

    def initialize_grids(self, image_path):
        img = Image.open(image_path).convert('L')
        img_array = np.array(img, dtype=np.float32) / 255.0
        
        u = np.ones(self.grid_size, dtype=np.float32)
        v = np.zeros(self.grid_size, dtype=np.float32)
        
        v[img_array < 1.0] = 1.0
        
        noise = np.random.rand(*self.grid_size) < 0.3
        v[noise] = 1.0
        
        return u, v

    def laplacian(self, Z):
        return (
            -4 * Z
            + np.roll(Z, 1, axis=0) + np.roll(Z, -1, axis=0)
            + np.roll(Z, 1, axis=1) + np.roll(Z, -1, axis=1)
        )

    def update(self):
        lapU = self.laplacian(self.u)
        lapV = self.laplacian(self.v)
        
        self.u = np.clip(self.u + self.time_step * (self.dU * lapU + self.u - self.u**3 - self.v + self.I), 0, 1)
        self.v = np.clip(self.v + self.time_step * (self.dV * lapV + self.epsilon * (self.u - self.gamma * self.v)), 0, 1)

    def define_palette(self):
        palette = Palette('resources/palette.txt')
        return palette.get_colors()

    def draw(self, screen, screen_size):
        # Normalize the values to cover the full range of the palette
        C = (self.u - self.v) * 255
        C = np.clip(C, 0, 255).astype(np.uint8)
        
        # Normalize C to the range of the palette indices
        C_normalized = (C / 255.0) * (len(self.palette) - 1)
        C_normalized = np.clip(C_normalized, 0, len(self.palette) - 1).astype(np.uint8)
        
        # Create a NumPy array for the palette
        palette_array = np.array(self.palette, dtype=np.uint8)
        
        # Map the normalized values to the palette
        color_grid = palette_array[C_normalized]
        
        surface = pygame.surfarray.make_surface(color_grid)
        
        scale_factor = screen_size[1] / self.grid_size[1]
        new_width = int(self.grid_size[0] * scale_factor)
        
        scaled_surface = pygame.transform.smoothscale(surface, (new_width, screen_size[1]))
        
        x_offset = (screen_size[0] - new_width) // 2
        
        screen.fill((0, 0, 0))
        screen.blit(scaled_surface, (x_offset, 0))
