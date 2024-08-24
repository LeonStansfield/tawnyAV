from moviepy.editor import VideoFileClip
import pygame
import random
from ..scene import Scene

class VideoScene(Scene):
    def __init__(self, video_path, name):
        self.name = name
        self.video_path = video_path
        self.clip = VideoFileClip(video_path)
        self.current_frame = None
        self.frame_surface = None
        self.frame_rate = self.clip.fps
        self.current_time = 0

    def draw(self, screen, data):
        if self.current_time < self.clip.duration:
            self.current_frame = self.clip.get_frame(self.current_time)
            self.frame_surface = pygame.surfarray.make_surface(self.current_frame.swapaxes(0, 1))
            
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
            self.current_time += 0.75 / self.frame_rate  # Play at 0.5 speed
        else:
            self.current_time = 0

    def handle_beat(self):
        # Skip to a random part of the video
        self.current_time = random.uniform(1, self.clip.duration - 1)
    