import pygame
from pygame.locals import *
import numpy as np
import time

from src.gui import GUIManager

from src.audio_processing import AudioProcessor
from src.effects.waveform import WaveformScene
from src.effects.reaction_diffusion1 import ReactionDiffusion1, ReactionDiffusionScene1
from src.effects.reaction_diffusion2 import ReactionDiffusion2, ReactionDiffusionScene2
from src.effects.game_of_life import gameOfLife, gameOfLifeScene
from src.effects.brians_brain_automata import BriansBrainAutomata, BriansBrainAutomataScene
from src.effects.video import VideoScene

# Constants
grid_size = (320, 180)
screen_size = (960, 540)
image_path = 'resources/wyr_image.png'

def main():
    pygame.init()
    screen = pygame.display.set_mode(screen_size)
    pygame.display.set_caption("TAWNY AV")
    clock = pygame.time.Clock()

    # Initialize font
    pygame.font.init()
    font = pygame.font.SysFont('Arial', 18)

    # Create scenes
    waveform_scene = WaveformScene("Waveform")
    reaction_diffusion_scene1 = ReactionDiffusionScene1(ReactionDiffusion1(grid_size, 0.07, 0.13, 0.055, 0.062, 'resources/wyr_image.png'), "Reaction-Diffusion")
    reaction_diffusion_scene2 = ReactionDiffusionScene2(ReactionDiffusion2(grid_size, 0.16, 0.4, 0.02, 0.05, 0.4, 'resources/wyr_image.png'), "Reaction-Diffusion 2")
    game_of_life_scene = gameOfLifeScene(gameOfLife(grid_size, 'resources/wyr_image.png'), "Game of Life")
    brians_brain_scene = BriansBrainAutomataScene(BriansBrainAutomata(grid_size, 'resources/wyr_image.png'), "Brian's Brain")
    alfred_video_scene = VideoScene('resources/PXL_20240606_123856647.mp4', "Alfred Video Scene")
    friends_video_scene = VideoScene('resources\MVI_9064.MP4', "Friends Video Scene")
    scenes = [waveform_scene, reaction_diffusion_scene1, reaction_diffusion_scene2, game_of_life_scene, brians_brain_scene, alfred_video_scene, friends_video_scene]

    gui_manager = GUIManager(screen_size, scenes)
    audio_processor = AudioProcessor()

    try:
        running = True
        last_update_time = time.time()
        while running:
            running = gui_manager.handle_events(scenes)

            data = audio_processor.read_data()
            current_time = time.time()
            if audio_processor.detect_beat(data, gui_manager.threshold_slider.get_current_value(), gui_manager.cooldown_slider.get_current_value()) or (current_time - last_update_time > 16):
                for scene in scenes:
                    scene.handle_beat()
                last_update_time = current_time

            screen.fill((0, 0, 0))
            gui_manager.current_scene.draw(screen, data)

            if gui_manager.show_ui:
                gui_manager.manager.update(clock.tick(60) / 1000.0)
                gui_manager.manager.draw_ui(screen)

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