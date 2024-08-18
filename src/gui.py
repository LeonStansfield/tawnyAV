import pygame
import numpy as np

class GUI:
    def __init__(self, screen_size):
        pygame.init()
        self.original_screen_size = screen_size
        self.screen_size = screen_size
        self.screen = pygame.display.set_mode(self.screen_size)
        pygame.display.set_caption("Audio Visualizer with Reaction-Diffusion")
        self.clock = pygame.time.Clock()
        self.is_fullscreen = False
        self.show_waveform = True
        self.selected_slider = None
        self.sliders = {
            "THRESHOLD_MULTIPLIER": {"value": 2.0, "min": 0.5, "max": 5.0, "rect": pygame.Rect(40, 40, 200, 20)},
            "COOLDOWN_TIME": {"value": 0.3, "min": 0.1, "max": 1.0, "rect": pygame.Rect(300, 40, 200, 20)},
        }
        self.button_rect = pygame.Rect(40, 80, 150, 20)

    def handle_events(self, audio_processor, reaction_diffusion, image_path):
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                return False
            elif event.type == pygame.KEYDOWN:
                if event.key == pygame.K_F11:
                    self.toggle_fullscreen()
            self.handle_sliders(event)
            self.handle_button(event)
        
        data = audio_processor.read_data()
        if audio_processor.detect_beat(data, self.sliders["THRESHOLD_MULTIPLIER"]["value"], self.sliders["COOLDOWN_TIME"]["value"]):
            pre_simulation_steps = np.random.randint(0, 100)
            reaction_diffusion.A, reaction_diffusion.B = reaction_diffusion.initialize_grids(image_path)
            for _ in range(pre_simulation_steps):
                reaction_diffusion.update()
        
        return True

    def toggle_fullscreen(self):
        self.is_fullscreen = not self.is_fullscreen
        if self.is_fullscreen:
            self.screen = pygame.display.set_mode((0, 0), pygame.FULLSCREEN)
            self.screen_size = self.screen.get_size()
        else:
            self.screen = pygame.display.set_mode(self.original_screen_size)
            self.screen_size = self.original_screen_size

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
            reaction_diffusion.draw(self.screen, self.screen_size)
        
        for name, slider_info in self.sliders.items():
            self.draw_slider(name, slider_info)
        
        self.draw_button(self.button_rect, "Toggle View")
        pygame.display.flip()

    def draw_waveform(self, data):
        samples = np.frombuffer(data, dtype=np.int16)
        samples = samples / 32768.0
        waveform_height = self.screen_size[1] // 2
        samples = samples * waveform_height
        x_scale = self.screen_size[0] / len(samples)
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