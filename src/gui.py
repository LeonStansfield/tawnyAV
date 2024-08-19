import pygame
import pygame_gui
import numpy as np
import time

class GUI:
    def __init__(self, screen_size, scenes):
        pygame.init()
        self.original_screen_size = screen_size
        self.screen_size = screen_size
        self.last_update_time = time.time()
        self.screen = pygame.display.set_mode(self.screen_size)
        pygame.display.set_caption("Audio Visualizer with Reaction-Diffusion")
        self.clock = pygame.time.Clock()
        self.is_fullscreen = False
        self.show_ui = True  # Flag to track UI visibility
        self.show_waveform = True  # Initialize show_waveform attribute
        self.scenes = scenes
        self.current_scene = scenes[0]  # Initialize with the first scene

        # Initialize pygame_gui
        self.manager = pygame_gui.UIManager(self.screen_size)

        # Create GUI elements
        self.threshold_label = pygame_gui.elements.UILabel(
            relative_rect=pygame.Rect((40, 10), (200, 20)),
            text='Threshold Multiplier',
            manager=self.manager
        )
        self.threshold_slider = pygame_gui.elements.UIHorizontalSlider(
            relative_rect=pygame.Rect((40, 40), (200, 20)),
            start_value=2.0,
            value_range=(0.5, 5.0),
            manager=self.manager
        )
        self.cooldown_label = pygame_gui.elements.UILabel(
            relative_rect=pygame.Rect((300, 10), (200, 20)),
            text='Cooldown Time',
            manager=self.manager
        )
        self.cooldown_slider = pygame_gui.elements.UIHorizontalSlider(
            relative_rect=pygame.Rect((300, 40), (200, 20)),
            start_value=0.3,
            value_range=(0.1, 1.0),
            manager=self.manager
        )
        self.scene_dropdown = pygame_gui.elements.UIDropDownMenu(
            options_list=[scene.name for scene in self.scenes],
            starting_option=self.scenes[0].name,
            relative_rect=pygame.Rect((40, 80), (150, 20)),
            manager=self.manager
        )

    def handle_events(self, audio_processor, reaction_diffusion, image_path):
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                return False
            elif event.type == pygame.KEYDOWN:
                if event.key == pygame.K_F11:
                    self.toggle_fullscreen()
                elif event.key == pygame.K_h:
                    self.toggle_ui()
            
            self.manager.process_events(event)

            # Handle dropdown selection
            if event.type == pygame_gui.UI_DROP_DOWN_MENU_CHANGED:
                if event.ui_element == self.scene_dropdown:
                    selected_scene_name = event.text
                    self.current_scene = next(scene for scene in self.scenes if scene.name == selected_scene_name)
        
        data = audio_processor.read_data()
        current_time = time.time()
        if audio_processor.detect_beat(data, self.threshold_slider.get_current_value(), self.cooldown_slider.get_current_value()) or (current_time - self.last_update_time > 16):
            pre_simulation_steps = np.random.randint(0, 100)
            reaction_diffusion.A, reaction_diffusion.B = reaction_diffusion.initialize_grids(image_path)
            for _ in range(pre_simulation_steps):
                reaction_diffusion.update()
            self.last_update_time = current_time
        
        return True

    def toggle_fullscreen(self):
        self.is_fullscreen = not self.is_fullscreen
        if self.is_fullscreen:
            self.screen = pygame.display.set_mode((0, 0), pygame.FULLSCREEN)
            self.screen_size = self.screen.get_size()
        else:
            self.screen = pygame.display.set_mode(self.original_screen_size)
            self.screen_size = self.original_screen_size

    def toggle_waveform(self):
        self.show_waveform = not self.show_waveform

    def toggle_ui(self):
        self.show_ui = not self.show_ui

    def draw(self, data):
        self.screen.fill((0, 0, 0))
        self.current_scene.draw(self.screen, data)
        
        if self.show_ui:
            self.manager.update(self.clock.tick(60) / 1000.0)
            self.manager.draw_ui(self.screen)
        pygame.display.flip()

    def close(self):
        pygame.quit()