import threading
import numpy as np
from PIL import Image
import pygame
from moviepy.editor import VideoFileClip
import random
from ..palette import Palette
from ..scene import Scene

class VideoReactionDiffusionScene(Scene):
    def __init__(self, video_path, video_reaction_diffusion, name):
        self.name = name
        self.video_path = video_path
        self.clip = VideoFileClip(video_path)
        self.current_frame = None
        self.frame_surface = None
        self.frame_rate = self.clip.fps
        self.current_time = 0
        self.video_reaction_diffusion = video_reaction_diffusion
        self.use_reaction_diffusion = False

    def draw(self, screen, data):
        if self.use_reaction_diffusion:
            self.video_reaction_diffusion.update()
            self.video_reaction_diffusion.draw(screen, screen.get_size())
        else:
            if self.current_time < self.clip.duration:
                self.current_frame = self.clip.get_frame(self.current_time)
                self.frame_surface = pygame.surfarray.make_surface(np.rot90(self.current_frame, 1))
                # Flip the frame horizontally
                self.frame_surface = pygame.transform.flip(self.frame_surface, True, False)
                
                # Get the screen size
                screen_width, screen_height = screen.get_size()
                
                # Calculate the new width to maintain the aspect ratio
                aspect_ratio = self.clip.size[0] / self.clip.size[1]
                new_width = int(screen_height * aspect_ratio)
                
                # Scale the frame to fit the window height while maintaining aspect ratio
                self.frame_surface = pygame.transform.scale(self.frame_surface, (new_width, screen_height))
                
                # Center the video horizontally
                x_position = (screen_width - new_width) // 2
                
                screen.blit(self.frame_surface, (x_position, 0))
                self.current_time += 0.75 / self.frame_rate  # Play at 0.75 speed
            else:
                self.current_time = 0

    def handle_beat(self):
        if self.use_reaction_diffusion:
            self.video_reaction_diffusion.restart()
        else:
            self.current_time = random.uniform(1, self.clip.duration - 1)
            self.sample_frame_for_reaction_diffusion()
        
        # Add random pre-simulation steps
        pre_simulation_steps = np.random.randint(0, 100)
        for _ in range(pre_simulation_steps):
            self.video_reaction_diffusion.update()
        
        self.use_reaction_diffusion = not self.use_reaction_diffusion

    def sample_frame_for_reaction_diffusion(self):
        if self.current_frame is not None:
            img = Image.fromarray(self.current_frame)
            img = img.convert('L')
            img = img.resize((self.video_reaction_diffusion.grid_size[1], self.video_reaction_diffusion.grid_size[0]))  # Resize to match grid size
            img_array = np.array(img, dtype=np.float32) / 255.0
            self.video_reaction_diffusion.initialize_grids_from_image(img_array)


class VideoReactionDiffusion:
    def __init__(self, grid_size, dA, dB, feed, kill):
        self.grid_size = grid_size
        self.dA = dA
        self.dB = dB
        self.feed = feed
        self.kill = kill
        self.A, self.B = self.initialize_grids()
        self.palette = self.define_palette()

    def initialize_grids(self):
        A = np.ones(self.grid_size, dtype=np.float32)
        B = np.zeros(self.grid_size, dtype=np.float32)
        return A, B

    def initialize_grids_from_image(self, img_array):
        self.A = np.ones(self.grid_size, dtype=np.float32)
        self.B = np.zeros(self.grid_size, dtype=np.float32)

        img_array /= 2.0
        
        # Define thresholds
        threshold1 = 0.2
        threshold2 = 0.4
        threshold3 = 0.6
        threshold4 = 0.8
        
        # Map the image array to bands of 0, 0.25, 0.5, 0.75, and 1.0
        self.B[img_array < threshold1] = 0.0
        self.B[(img_array >= threshold1) & (img_array < threshold2)] = 0.35
        self.B[(img_array >= threshold2) & (img_array < threshold3)] = 0.5
        self.B[(img_array >= threshold3) & (img_array < threshold4)] = 0.65
        self.B[img_array >= threshold4] = 0.8

        for _ in range(np.random.randint(5, 500)):
            x, y = np.random.randint(0, self.grid_size[0]), np.random.randint(0, self.grid_size[1])
            size = np.random.randint(3, 5)
            self.B[max(0, x-size):min(self.grid_size[0], x+size), max(0, y-size):min(self.grid_size[1], y+size)] = 1.0

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

    def restart(self):
        self.A, self.B = self.initialize_grids()

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
        
        surface = pygame.surfarray.make_surface(color_grid.swapaxes(0, 1))  # Fix the axis swap here
        
        scale_factor = screen_size[1] / self.grid_size[1]
        new_width = int(self.grid_size[0] * scale_factor)
        
        scaled_surface = pygame.transform.smoothscale(surface, (new_width, screen_size[1]))
        
        x_offset = (screen_size[0] - new_width) // 2
        
        screen.fill((0, 0, 0))
        screen.blit(scaled_surface, (x_offset, 0))
