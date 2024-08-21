import threading
import numpy as np
from PIL import Image
import pygame
from scipy.signal import convolve2d
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
        
        # Set pixels from the image to 1.0 in v
        v[img_array < 1.0] = 1.0
        
        # Add random noise
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
        return [(0, 0, 0), (3, 8, 10), (7, 16, 20), (11, 24, 30), (15, 33, 40), (19, 41, 50), (23, 49, 60), (27, 58, 70), (31, 66, 80), (35, 74, 90), (39, 83, 100), (43, 91, 110), (47, 99, 120), (51, 108, 130), (55, 116, 140), (59, 124, 150), (63, 133, 160), (67, 141, 170), (71, 149, 180), (75, 158, 191), (72, 153, 187), (69, 148, 183), (66, 144, 179), (63, 139, 175), (61, 135, 171), (58, 130, 167), (55, 125, 163), (52, 121, 159), (49, 116, 155), (47, 112, 151), (44, 107, 147), (41, 103, 143), (38, 98, 139), (35, 93, 135), (33, 89, 131), (30, 84, 127), (27, 80, 123), (24, 75, 119), (22, 71, 115), (28, 70, 119), (34, 70, 123), (40, 70, 127), (47, 69, 131), (53, 69, 135), (59, 69, 139), (65, 68, 143), (72, 68, 147), (78, 68, 151), (84, 67, 155), (90, 67, 159), (97, 67, 163), (103, 66, 167), (109, 66, 171), (115, 66, 175), (122, 65, 179), (128, 65, 183), (134, 65, 187), (141, 65, 191), (146, 67, 190), (151, 69, 190), (156, 71, 189), (162, 74, 189), (167, 76, 188), (172, 78, 188), (178, 81, 187), (183, 83, 187), (188, 85, 186), (194, 88, 186), (199, 90, 185), (204, 92, 185), (210, 95, 184), (215, 97, 184), (220, 99, 183), (226, 102, 183), (231, 104, 182), (236, 106, 182), (242, 109, 182), (240, 106, 176), (239, 104, 171), (238, 102, 165), (236, 100, 160), (235, 97, 155), (234, 95, 149), (232, 93, 144), (231, 91, 139), (230, 89, 133), (228, 86, 128), (227, 84, 122), (226, 82, 117), (224, 80, 112), (223, 78, 106), (222, 75, 101), (220, 73, 96), (219, 71, 90), (218, 69, 85), (217, 67, 80)]

    def map_to_palette(self, value):
        index = int(value * (len(self.palette) - 1))
        return self.palette[index]

    def draw(self, screen, screen_size):
        def process_chunk(start_row, end_row, color_grid, C):
            for i in range(start_row, end_row):
                for j in range(self.grid_size[1]):
                    color_grid[i, j] = self.map_to_palette(C[i, j] / 255.0)

        C = (self.u - self.v) * 255
        C = np.clip(C, 0, 255).astype(np.uint8)
        
        # Apply Gaussian blur
        C = gaussian_filter(C, sigma=1)
        
        color_grid = np.zeros((self.grid_size[0], self.grid_size[1], 3), dtype=np.uint8)
        
        # Number of threads to use
        num_threads = 4
        chunk_size = self.grid_size[0] // num_threads
        threads = []

        for i in range(num_threads):
            start_row = i * chunk_size
            end_row = (i + 1) * chunk_size if i < num_threads - 1 else self.grid_size[0]
            thread = threading.Thread(target=process_chunk, args=(start_row, end_row, color_grid, C))
            threads.append(thread)
            thread.start()
        
        # Wait for all threads to complete
        for thread in threads:
            thread.join()

        surface = pygame.surfarray.make_surface(color_grid)
        
        # Calculate scaling factor to maintain aspect ratio
        scale_factor = screen_size[1] / self.grid_size[1]
        new_width = int(self.grid_size[0] * scale_factor)
        
        # Scale the surface
        scaled_surface = pygame.transform.scale(surface, (new_width, screen_size[1]))
        
        # Calculate position to center the surface
        x_offset = (screen_size[0] - new_width) // 2
        
        # Fill the screen with black
        screen.fill((0, 0, 0))
        
        # Blit the scaled surface onto the screen
        screen.blit(scaled_surface, (x_offset, 0))
