import numpy as np

from src.audio_processing import AudioProcessor
from src.render import Render
from src.shader_test import ShaderTest, ShaderTestScene
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
    shader_test = ShaderTest()
    waveform_scene = WaveformScene("Waveform")
    reaction_diffusion_scene = ReactionDiffusionScene(reaction_diffusion, "Reaction-Diffusion")
    shader_test_scene = ShaderTestScene(shader_test, "shader test")
    scenes = [waveform_scene, reaction_diffusion_scene, shader_test_scene]
    render = Render(screen_size, scenes)

    try:
        running = True
        while running:
            running = render.handle_events(audio_processor)
            data = audio_processor.read_data()
            render.draw(data)
            render.clock.tick(60)
    finally:
        render.close()
        audio_processor.close()

if __name__ == "__main__":
    main()
