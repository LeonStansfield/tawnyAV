import numpy as np
import pygame
import pyaudio
import scipy.fftpack
import time
from PIL import Image
from collections import deque

# Constants
CHUNK = 1024
FORMAT = pyaudio.paInt16
CHANNELS = 1
RATE = 44100
LOW_FREQ = 20
HIGH_FREQ = 150
ROLLING_WINDOW = 50
dA = 0.07
dB = 0.13
feed = 0.055
kill = 0.062
grid_size = (200, 200)
screen_size = (800, 800)
pre_simulation_steps = np.random.randint(0, 100)
image_path = 'resources/wyr_image.png'  # Update this path to your image file

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

    def draw(self, screen):
        C = (self.A - self.B) * 255
        C = np.clip(C, 0, 255).astype(np.uint8)
        color_grid = np.zeros((self.grid_size[0], self.grid_size[1], 3), dtype=np.uint8)
        for i in range(self.grid_size[0]):
            for j in range(self.grid_size[1]):
                color_grid[i, j] = self.map_to_palette(C[i, j] / 255.0)
        surface = pygame.surfarray.make_surface(color_grid)
        surface = pygame.transform.scale(surface, screen_size)
        screen.blit(surface, (0, 0))

class AudioProcessor:
    def __init__(self):
        self.p = pyaudio.PyAudio()
        self.stream = self.p.open(format=FORMAT, channels=CHANNELS, rate=RATE, input=True, frames_per_buffer=CHUNK)
        self.energy_queue = deque(maxlen=ROLLING_WINDOW)
        self.last_beat_time = 0

    def read_data(self):
        return self.stream.read(CHUNK, exception_on_overflow=False)

    def detect_beat(self, data, threshold_multiplier, cooldown_time):
        samples = np.frombuffer(data, dtype=np.int16)
        fft_spectrum = np.abs(scipy.fftpack.fft(samples))[:CHUNK // 2]
        freqs = np.fft.fftfreq(len(samples), 1.0 / RATE)[:CHUNK // 2]
        low_freq_spectrum = fft_spectrum[(freqs >= LOW_FREQ) & (freqs <= HIGH_FREQ)]
        energy = np.sum(low_freq_spectrum)
        self.energy_queue.append(energy)
        
        if len(self.energy_queue) == ROLLING_WINDOW:
            avg_energy = np.mean(self.energy_queue)
            if energy > threshold_multiplier * avg_energy:
                current_time = time.time()
                if current_time - self.last_beat_time > cooldown_time:
                    self.last_beat_time = current_time
                    return True
        return False

    def close(self):
        self.stream.stop_stream()
        self.stream.close()
        self.p.terminate()

class GUI:
    def __init__(self, screen_size):
        pygame.init()
        self.screen = pygame.display.set_mode(screen_size)
        pygame.display.set_caption("Audio Visualizer with Reaction-Diffusion")
        self.clock = pygame.time.Clock()
        self.is_fullscreen = False
        self.show_waveform = True
        self.selected_slider = None
        self.sliders = {
            "THRESHOLD_MULTIPLIER": {"value": 2.0, "min": 0.5, "max": 5.0, "rect": pygame.Rect(300, 750, 200, 20)},
            "COOLDOWN_TIME": {"value": 0.3, "min": 0.1, "max": 1.0, "rect": pygame.Rect(550, 750, 200, 20)},
        }
        self.button_rect = pygame.Rect((screen_size[0] - 150) // 2, 660, 150, 40)

    def handle_events(self, audio_processor, reaction_diffusion):
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                return False
            elif event.type == pygame.KEYDOWN:
                if event.key == pygame.K_F11:
                    self.is_fullscreen = not self.is_fullscreen
                    if self.is_fullscreen:
                        self.screen = pygame.display.set_mode((0, 0), pygame.FULLSCREEN)
                    else:
                        self.screen = pygame.display.set_mode(screen_size)
            self.handle_sliders(event)
            self.handle_button(event)
        
        data = audio_processor.read_data()
        if audio_processor.detect_beat(data, self.sliders["THRESHOLD_MULTIPLIER"]["value"], self.sliders["COOLDOWN_TIME"]["value"]):
            pre_simulation_steps = np.random.randint(0, 100)
            reaction_diffusion.A, reaction_diffusion.B = reaction_diffusion.initialize_grids(image_path)
            for _ in range(pre_simulation_steps):
                reaction_diffusion.update()
        
        return True

    def handle_sliders(self, event):
        if event.type == pygame.MOUSEBUTTONDOWN:
            for name, slider in self.sliders.items():
                if slider["rect"].collidepoint(event.pos):
                    self.selected_slider = name
                    break
        elif event.type == pygame.MOUSEBUTTONUP:
            self.selected_slider = None
        elif event.type == pygame.MOUSEMOTION and self.selected_slider:
            slider = self.sliders[self.selected_slider]
            x, y, width, height = slider["rect"]
            relative_x = max(0, min(event.pos[0] - x, width))
            slider["value"] = slider["min"] + (slider["max"] - slider["min"]) * (relative_x / width)

    def handle_button(self, event):
        if event.type == pygame.MOUSEBUTTONDOWN and self.button_rect.collidepoint(event.pos):
            self.show_waveform = not self.show_waveform

    def draw(self, reaction_diffusion, data):
        self.screen.fill((0, 0, 0))
        if self.show_waveform:
            self.draw_waveform(data)
        else:
            reaction_diffusion.update()
            reaction_diffusion.draw(self.screen)
        
        for name, slider_info in self.sliders.items():
            self.draw_slider(name, slider_info)
        
        self.draw_button(self.button_rect, "Toggle View")
        pygame.display.flip()

    def draw_waveform(self, data):
        samples = np.frombuffer(data, dtype=np.int16)
        samples = samples / 32768.0
        waveform_height = screen_size[0] // 2
        samples = samples * waveform_height
        x_scale = screen_size[1] / len(samples)
        points = [(x * x_scale, waveform_height + samples[x]) for x in range(len(samples))]
        pygame.draw.lines(self.screen, (255, 255, 255), False, points)

    def draw_slider(self, name, slider_info):
        pygame.draw.rect(self.screen, (255, 255, 255), slider_info["rect"], 2)
        x, y, width, height = slider_info["rect"]
        handle_x = x + int((slider_info["value"] - slider_info["min"]) / (slider_info["max"] - slider_info["min"]) * width)
        pygame.draw.circle(self.screen, (255, 255, 255), (handle_x, y + height // 2), height // 2)

        font = pygame.font.Font(None, 24)
        text = font.render(f"{name}: {slider_info['value']:.2f}", True, (255, 255, 255))
        self.screen.blit(text, (x, y - 25))

    def draw_button(self, rect, text):
        pygame.draw.rect(self.screen, (255, 255, 255), rect, 2)
        font = pygame.font.Font(None, 36)
        text_surface = font.render(text, True, (255, 255, 255))
        text_rect = text_surface.get_rect(center=rect.center)
        self.screen.blit(text_surface, text_rect)

    def close(self):
        pygame.quit()

# Main loop
def main():
    audio_processor = AudioProcessor()
    reaction_diffusion = ReactionDiffusion(grid_size, dA, dB, feed, kill, image_path)
    gui = GUI(screen_size)

    try:
        running = True
        while running:
            running = gui.handle_events(audio_processor, reaction_diffusion)
            data = audio_processor.read_data()
            gui.draw(reaction_diffusion, data)
            gui.clock.tick(60)
    finally:
        gui.close()
        audio_processor.close()

if __name__ == "__main__":
    main()
