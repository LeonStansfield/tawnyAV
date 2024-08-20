import pygame
from pygame.locals import *
import pygame_gui
import numpy as np
import time

from src.audio_processing import AudioProcessor
from src.waveform import WaveformScene
from src.reaction_diffusion import ReactionDiffusion, ReactionDiffusionScene
from src.cellular_automata import CellularAutomata, CellularAutomataScene

# Constants
dA = 0.07
dB = 0.13
feed = 0.055
kill = 0.062
grid_size = (240, 180)
screen_size = (960, 720)
image_path = 'resources/wyr_image.png'  # Update this path to your image file

def create_gui_elements(manager):
    threshold_label = pygame_gui.elements.UILabel(
        relative_rect=pygame.Rect((40, 10), (200, 20)),
        text='Threshold Multiplier',
        manager=manager
    )
    threshold_slider = pygame_gui.elements.UIHorizontalSlider(
        relative_rect=pygame.Rect((40, 40), (200, 20)),
        start_value=2.0,
        value_range=(0.5, 5.0),
        manager=manager
    )
    cooldown_label = pygame_gui.elements.UILabel(
        relative_rect=pygame.Rect((300, 10), (200, 20)),
        text='Cooldown Time',
        manager=manager
    )
    cooldown_slider = pygame_gui.elements.UIHorizontalSlider(
        relative_rect=pygame.Rect((300, 40), (200, 20)),
        start_value=0.3,
        value_range=(0.1, 1.0),
        manager=manager
    )
    return threshold_slider, cooldown_slider

def handle_events(manager, scene_dropdown, scenes, current_scene, is_fullscreen, show_ui):
    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            return False, current_scene, is_fullscreen, show_ui
        elif event.type == pygame.KEYDOWN:
            if event.key == pygame.K_F11:
                is_fullscreen = not is_fullscreen
                pygame.display.set_mode((0, 0), pygame.FULLSCREEN if is_fullscreen else 0)
            elif event.key == pygame.K_h:
                show_ui = not show_ui

        manager.process_events(event)

        if event.type == pygame_gui.UI_DROP_DOWN_MENU_CHANGED and event.ui_element == scene_dropdown:
            selected_scene_name = event.text
            current_scene = next(scene for scene in scenes if scene.name == selected_scene_name)

    return True, current_scene, is_fullscreen, show_ui

def main():
    pygame.init()
    screen = pygame.display.set_mode(screen_size)
    pygame.display.set_caption("Audio Visualizer")
    clock = pygame.time.Clock()
    manager = pygame_gui.UIManager(screen_size)
    threshold_slider, cooldown_slider = create_gui_elements(manager)

    # Create scenes
    waveform_scene = WaveformScene("Waveform")
    reaction_diffusion_scene = ReactionDiffusionScene(ReactionDiffusion(grid_size, dA, dB, feed, kill, image_path), "Reaction-Diffusion")
    cellular_automata_scene = CellularAutomataScene(CellularAutomata(grid_size, image_path), "Cellular Automata")
    scenes = [waveform_scene, reaction_diffusion_scene, cellular_automata_scene]

    # Create scene_dropdown
    scene_dropdown = pygame_gui.elements.UIDropDownMenu(
        options_list=[scene.name for scene in scenes],
        starting_option=scenes[0].name,
        relative_rect=pygame.Rect((40, 80), (150, 20)),
        manager=manager
    )

    current_scene = scenes[0]
    is_fullscreen = False
    show_ui = True
    last_update_time = time.time()

    audio_processor = AudioProcessor()

    try:
        running = True
        while running:
            running, current_scene, is_fullscreen, show_ui = handle_events(
                manager, scene_dropdown, scenes, current_scene, is_fullscreen, show_ui
            )

            data = audio_processor.read_data()
            current_time = time.time()
            if audio_processor.detect_beat(data, threshold_slider.get_current_value(), cooldown_slider.get_current_value()) or (current_time - last_update_time > 16):
                for scene in scenes:
                    scene.handle_beat()
                last_update_time = current_time

            screen.fill((0, 0, 0))
            current_scene.draw(screen, data)

            if show_ui:
                manager.update(clock.tick(60) / 1000.0)
                manager.draw_ui(screen)
            pygame.display.flip()

            clock.tick(30)
    finally:
        pygame.quit()
        audio_processor.close()

if __name__ == "__main__":
    main()