import numpy as np

from src.audio_processing import AudioProcessor
from src.gui import GUI
from src.waveform import WaveformScene
from src.reaction_diffusion import ReactionDiffusion, ReactionDiffusionScene

# Constants
dA = 0.07
dB = 0.13
feed = 0.055
kill = 0.062
grid_size = (240, 180)
screen_size = (960, 720)
image_path = 'resources/wyr_image.png'  # Update this path to your image file

# Main loop
def main():
    audio_processor = AudioProcessor()
    reaction_diffusion = ReactionDiffusion(grid_size, dA, dB, feed, kill, image_path)
    waveform_scene = WaveformScene("Waveform")
    reaction_diffusion_scene = ReactionDiffusionScene(reaction_diffusion, "Reaction-Diffusion")
    scenes = [waveform_scene, reaction_diffusion_scene]
    gui = GUI(screen_size, scenes)

    try:
        running = True
        while running:
            running = gui.handle_events(audio_processor, reaction_diffusion, image_path)
            data = audio_processor.read_data()
            gui.draw(data)
            gui.clock.tick(60)
    finally:
        gui.close()
        audio_processor.close()

if __name__ == "__main__":
    main()
