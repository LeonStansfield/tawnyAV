import numpy as np
import pygame
from .scene import Scene

class WaveformScene(Scene):
    def __init__(self, name):
        self.name = name

    def draw(self, screen, data):
        samples = np.frombuffer(data, dtype=np.int16)
        samples = samples / 32768.0
        waveform_height = screen.get_height() // 2
        samples = samples * waveform_height
        x_scale = screen.get_width() / len(samples)
        points = [(x * x_scale, waveform_height + samples[x]) for x in range(len(samples))]
        pygame.draw.lines(screen, (255, 255, 255), False, points)