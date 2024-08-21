import pygame
from pygame.locals import *
import pygame_gui
import numpy as np
import time

from src.audio_processing import AudioProcessor
from src.waveform import WaveformScene
from src.reaction_diffusion1 import ReactionDiffusion1, ReactionDiffusionScene1
from src.reaction_diffusion2 import ReactionDiffusion2, ReactionDiffusionScene2
from src.game_of_life import gameOfLife, gameOfLifeScene
from src.brians_brain_automata import BriansBrainAutomata, BriansBrainAutomataScene

# Constants
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
        start_value=1.0,
        value_range=(0.1, 3.5),
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

def handle_events(manager, scene_dropdown, scenes, current_scene, is_fullscreen, show_ui, threshold_slider):
    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            return False, current_scene, is_fullscreen, show_ui
        elif event.type == pygame.KEYDOWN:
            if event.key == pygame.K_F11:
                is_fullscreen = not is_fullscreen
                pygame.display.set_mode((0, 0), pygame.FULLSCREEN if is_fullscreen else 0)
            elif event.key == pygame.K_h:
                show_ui = not show_ui
            elif pygame.K_1 <= event.key <= pygame.K_9:
                index = event.key - pygame.K_1
                if index < len(scenes):
                    current_scene = scenes[index]
            elif event.key == pygame.K_UP:
                new_value = min(threshold_slider.get_current_value() + 0.1, threshold_slider.value_range[1])
                threshold_slider.set_current_value(new_value)
            elif event.key == pygame.K_DOWN:
                new_value = max(threshold_slider.get_current_value() - 0.1, threshold_slider.value_range[0])
                threshold_slider.set_current_value(new_value)

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

    # Initialize font
    pygame.font.init()
    font = pygame.font.SysFont('Arial', 18)

    # Create scenes
    waveform_scene = WaveformScene("Waveform")

    reaction_diffusion_scene1 = ReactionDiffusionScene1(ReactionDiffusion1(grid_size, 0.07, 0.13, 0.055, 0.062, image_path), "Reaction-Diffusion")
    reaction_diffusion_scene2 = ReactionDiffusionScene2(ReactionDiffusion2(grid_size, 0.16, 0.4, 0.02, 0.05, 0.4, image_path), "Reaction-Diffusion 2")
    game_of_life_scene = gameOfLifeScene(gameOfLife(grid_size, image_path), "Game of Life")
    brians_brain_scene = BriansBrainAutomataScene(BriansBrainAutomata(grid_size, image_path), "Brian's Brain")
    scenes = [waveform_scene, reaction_diffusion_scene1, reaction_diffusion_scene2, game_of_life_scene, brians_brain_scene]

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
                manager, scene_dropdown, scenes, current_scene, is_fullscreen, show_ui, threshold_slider
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

            # Calculate and draw FPS
            fps = clock.get_fps()
            fps_text = font.render(f"FPS: {fps:.2f}", True, pygame.Color('white'))
            screen.blit(fps_text, (10, 10))

            pygame.display.flip()

            clock.tick(30)
    finally:
        pygame.quit()
        audio_processor.close()

if __name__ == "__main__":
    main()
