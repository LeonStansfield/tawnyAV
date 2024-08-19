import numpy as np
from PIL import Image
import pygame
from .scene import Scene

class ReactionDiffusionScene(Scene):
    def __init__(self, reaction_diffusion, name):
        self.name = name
        self.reaction_diffusion = reaction_diffusion

    def draw(self, screen, data):
        self.reaction_diffusion.update()
        self.reaction_diffusion.draw(screen, screen.get_size())

class ReactionDiffusion:
    def __init__(self, grid_size, dA, dB, feed, kill, image_path):
        self.grid_size = grid_size
        self.dA = dA
        self.dB = dB
        self.feed = feed
        self.kill = kill
        self.A, self.B = self.initialize_grids(image_path)
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
        return [(255, 255, 255), (252, 243, 244), (250, 231, 233), (247, 219, 222), (245, 208, 211), (243, 196, 200), (240, 184, 189), (238, 172, 178), (236, 161, 167), (233, 149, 156), (231, 137, 145), (228, 125, 134), (226, 114, 123), (224, 102, 112), (221, 90, 101), (219, 78, 90), (217, 67, 80), (218, 69, 86), (220, 72, 92), (221, 74, 99), (223, 77, 105), (224, 80, 111), (226, 82, 118), (227, 85, 124), (229, 88, 131), (231, 90, 137), (232, 93, 143), (234, 95, 150), (235, 98, 156), (237, 101, 162), (238, 103, 169), (240, 106, 175), (242, 109, 182), (235, 106, 182), (229, 103, 183), (223, 100, 183), (216, 98, 184), (210, 95, 184), (204, 92, 185), (197, 89, 185), (191, 87, 186), (185, 84, 187), (178, 81, 187), (172, 78, 188), (166, 76, 188), (159, 73, 189), (153, 70, 189), (147, 67, 190), (141, 65, 191), (133, 65, 186), (126, 65, 181), (118, 66, 176), (111, 66, 172), (103, 66, 167), (96, 67, 162), (88, 67, 157), (81, 68, 153), (74, 68, 148), (66, 68, 143), (59, 69, 138), (51, 69, 134), (44, 69, 129), (36, 70, 124), (29, 70, 119), (22, 71, 115), (25, 76, 119), (28, 81, 124), (31, 87, 129), (35, 92, 134), (38, 98, 138), (41, 103, 143), (45, 109, 148), (48, 114, 153), (51, 119, 157), (55, 125, 162), (58, 130, 167), (61, 136, 172), (65, 141, 176), (68, 147, 181), (71, 152, 186), (75, 158, 191), (70, 148, 179), (65, 138, 167), (60, 128, 155), (56, 118, 143), (51, 108, 131), (46, 98, 119), (42, 88, 107), (37, 79, 95), (32, 69, 83), (28, 59, 71), (23, 49, 59), (18, 39, 47), (14, 29, 35), (9, 19, 23), (4, 9, 11), (0, 0, 0)]

    def map_to_palette(self, value):
        index = int(value * (len(self.palette) - 1))
        return self.palette[index]

    def draw(self, screen, screen_size):
        def process_chunk(start_row, end_row, color_grid, C):
            for i in range(start_row, end_row):
                for j in range(self.grid_size[1]):
                    color_grid[i, j] = self.map_to_palette(C[i, j] / 255.0)

        C = (self.A - self.B) * 255
        C = np.clip(C, 0, 255).astype(np.uint8)
        color_grid = np.zeros((self.grid_size[0], self.grid_size[1], 3), dtype=np.uint8)
        
        # Process the entire grid in a single thread
        process_chunk(0, self.grid_size[0], color_grid, C)

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
