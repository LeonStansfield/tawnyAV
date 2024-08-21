import numpy as np
from PIL import Image
import pygame
from scipy.ndimage import gaussian_filter
from .scene import Scene

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
    def __init__(self, grid_size, dU, dV, epsilon, gamma, I, image_path):
        self.grid_size = grid_size
        self.dU = dU
        self.dV = dV
        self.epsilon = epsilon
        self.gamma = gamma
        self.I = I
        self.image_path = image_path
        self.u, self.v = self.initialize_grids(self.image_path)
        self.palette = self.define_palette()

    def initialize_grids(self, image_path):
        img = Image.open(image_path).convert('L')
        img_array = np.array(img, dtype=np.float32) / 255.0
        
        u = np.ones(self.grid_size, dtype=np.float32)
        v = np.zeros(self.grid_size, dtype=np.float32)
        
        v[img_array < 1.0] = 1.0
        
        noise = np.random.rand(*self.grid_size) < 0.02
        v[noise] = 0.5
        
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
        
        self.u = np.clip(self.u + self.dU * lapU + self.u - self.u**3 - self.v + self.I, 0, 1)
        self.v = np.clip(self.v + self.dV * lapV + self.epsilon * (self.u - self.gamma * self.v), 0, 1)

    def define_palette(self):
        return [(255, 255, 255), (252, 243, 244), (250, 231, 233), (247, 219, 222), (245, 208, 211), (243, 196, 200), (240, 184, 189), (238, 172, 178), (236, 161, 167), (233, 149, 156), (231, 137, 145), (228, 125, 134), (226, 114, 123), (224, 102, 112), (221, 90, 101), (219, 78, 90), (217, 67, 80), (218, 69, 86), (220, 72, 92), (221, 74, 99), (223, 77, 105), (224, 80, 111), (226, 82, 118), (227, 85, 124), (229, 88, 131), (231, 90, 137), (232, 93, 143), (234, 95, 150), (235, 98, 156), (237, 101, 162), (238, 103, 169), (240, 106, 175), (242, 109, 182), (235, 106, 182), (229, 103, 183), (223, 100, 183), (216, 98, 184), (210, 95, 184), (204, 92, 185), (197, 89, 185), (191, 87, 186), (185, 84, 187), (178, 81, 187), (172, 78, 188), (166, 76, 188), (159, 73, 189), (153, 70, 189), (147, 67, 190), (141, 65, 191), (133, 65, 186), (126, 65, 181), (118, 66, 176), (111, 66, 172), (103, 66, 167), (96, 67, 162), (88, 67, 157), (81, 68, 153), (74, 68, 148), (66, 68, 143), (59, 69, 138), (51, 69, 134), (44, 69, 129), (36, 70, 124), (29, 70, 119), (22, 71, 115), (25, 76, 119), (28, 81, 124), (31, 87, 129), (35, 92, 134), (38, 98, 138), (41, 103, 143), (45, 109, 148), (48, 114, 153), (51, 119, 157), (55, 125, 162), (58, 130, 167), (61, 136, 172), (65, 141, 176), (68, 147, 181), (71, 152, 186), (75, 158, 191), (70, 148, 179), (65, 138, 167), (60, 128, 155), (56, 118, 143), (51, 108, 131), (46, 98, 119), (42, 88, 107), (37, 79, 95), (32, 69, 83), (28, 59, 71), (23, 49, 59), (18, 39, 47), (14, 29, 35), (9, 19, 23), (4, 9, 11), (0, 0, 0)]

    def draw(self, screen, screen_size):
        C = (self.u - self.v) * 255
        C = np.clip(C, 0, 255).astype(np.uint8)
        
        # Create a NumPy array for the palette
        palette_array = np.array(self.palette, dtype=np.uint8)
        
        # Ensure C values are within the range of the palette indices
        C = np.clip(C, 0, len(self.palette) - 1)
        
        # Map the values to the palette
        color_grid = palette_array[C]
        
        surface = pygame.surfarray.make_surface(color_grid)
        
        scale_factor = screen_size[1] / self.grid_size[1]
        new_width = int(self.grid_size[0] * scale_factor)
        
        scaled_surface = pygame.transform.smoothscale(surface, (new_width, screen_size[1]))
        
        x_offset = (screen_size[0] - new_width) // 2
        
        screen.fill((0, 0, 0))
        screen.blit(scaled_surface, (x_offset, 0))
        