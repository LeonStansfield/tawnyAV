import numpy as np
import pygame
import pyaudio
import scipy.fftpack
import time
from PIL import Image
from collections import deque

import audio_processing.py

# Constants for beat detection
CHUNK = 1024
FORMAT = pyaudio.paInt16
CHANNELS = 1
RATE = 44100
LOW_FREQ = 20
HIGH_FREQ = 150
ROLLING_WINDOW = 50  # Hardcoded rolling window size

# Reaction-Diffusion parameters
dA = 0.07
dB = 0.13
feed = 0.055
kill = 0.062
grid_size = (200, 200)  # Grid size
screen_size = (800, 800)
pre_simulation_steps = np.random.randint(0, 100)  # Number of pre-simulation steps

# Initialize Pygame
pygame.init()
screen = pygame.display.set_mode(screen_size)
pygame.display.set_caption("Audio Visualizer with Reaction-Diffusion")
clock = pygame.time.Clock()
is_fullscreen = False

# PyAudio initialization
p = pyaudio.PyAudio()
stream = p.open(format=FORMAT, channels=CHANNELS, rate=RATE, input=True, frames_per_buffer=CHUNK)

# Rolling energy queue
energy_queue = deque(maxlen=ROLLING_WINDOW)
last_beat_time = 0

# Sliders for parameters
slider_width = 200
slider_height = 20
slider_spacing = 50
slider_y = screen_size[1] - slider_height - 50
slider_x_start = (screen_size[0] - (2 * slider_width + slider_spacing)) // 2

sliders = {
    "THRESHOLD_MULTIPLIER": {"value": 2.0, "min": 0.5, "max": 5.0, "rect": pygame.Rect(slider_x_start, slider_y, slider_width, slider_height)},
    "COOLDOWN_TIME": {"value": 0.3, "min": 0.1, "max": 1.0, "rect": pygame.Rect(slider_x_start + slider_width + slider_spacing, slider_y, slider_width, slider_height)},
}

selected_slider = None
show_waveform = True

# Toggle button
button_width = 150
button_height = 40
button_rect = pygame.Rect((screen_size[0] - button_width) // 2, slider_y - button_height - 40, button_width, button_height)

def initialize_grids(image_path):
    """Initialize the reaction-diffusion grids with image data."""
    # Load the image and convert to grayscale
    img = Image.open(image_path).convert('L')
    img_array = np.array(img, dtype=np.float32) / 255.0  # Normalize to [0, 1]
    
    # Initialize A and B grids
    A = np.ones(grid_size, dtype=np.float32)
    B = np.zeros(grid_size, dtype=np.float32)
    
    # Set B where image is not white (0.0)
    B[img_array < 1.0] = 1.0
    
    # Add random spots with varying sizes
    for _ in range(np.random.randint(5, 500)):
        x, y = np.random.randint(0, grid_size[0]), np.random.randint(0, grid_size[1])
        size = np.random.randint(3, 5)  # Random size between 3 and 15
        B[max(0, x-size):min(grid_size[0], x+size), max(0, y-size):min(grid_size[1], y+size)] = 1.0
    
    return A, B

def laplacian(Z):
    """Compute the Laplacian of matrix Z with a simplified 4-point stencil."""
    return (
        -4 * Z
        + np.roll(Z, 1, axis=0) + np.roll(Z, -1, axis=0)
        + np.roll(Z, 1, axis=1) + np.roll(Z, -1, axis=1)
    )

def update_reaction_diffusion(A, B, dA, dB, feed, kill):
    """Update the reaction-diffusion grid with simplified calculations."""
    lapA = laplacian(A)
    lapB = laplacian(B)
    reaction = A * B**2
    
    # Update A and B with clamping to avoid invalid values
    A = np.clip(A + dA * lapA - reaction + feed * (1 - A), 0, 1)
    B = np.clip(B + dB * lapB + reaction - (kill + feed) * B, 0, 1)
    
    return A, B

# Define a color palette
palette = [(255, 255, 255), (252, 243, 244), (250, 231, 233), (247, 219, 222), (245, 208, 211), (243, 196, 200), (240, 184, 189), (238, 172, 178), (236, 161, 167), (233, 149, 156), (231, 137, 145), (228, 125, 134), (226, 114, 123), (224, 102, 112), (221, 90, 101), (219, 78, 90), (217, 67, 80), (218, 69, 86), (220, 72, 92), (221, 74, 99), (223, 77, 105), (224, 80, 111), (226, 82, 118), (227, 85, 124), (229, 88, 131), (231, 90, 137), (232, 93, 143), (234, 95, 150), (235, 98, 156), (237, 101, 162), (238, 103, 169), (240, 106, 175), (242, 109, 182), (235, 106, 182), (229, 103, 183), (223, 100, 183), (216, 98, 184), (210, 95, 184), (204, 92, 185), (197, 89, 185), (191, 87, 186), (185, 84, 187), (178, 81, 187), (172, 78, 188), (166, 76, 188), (159, 73, 189), (153, 70, 189), (147, 67, 190), (141, 65, 191), (133, 65, 186), (126, 65, 181), (118, 66, 176), (111, 66, 172), (103, 66, 167), (96, 67, 162), (88, 67, 157), (81, 68, 153), (74, 68, 148), (66, 68, 143), (59, 69, 138), (51, 69, 134), (44, 69, 129), (36, 70, 124), (29, 70, 119), (22, 71, 115), (25, 76, 119), (28, 81, 124), (31, 87, 129), (35, 92, 134), (38, 98, 138), (41, 103, 143), (45, 109, 148), (48, 114, 153), (51, 119, 157), (55, 125, 162), (58, 130, 167), (61, 136, 172), (65, 141, 176), (68, 147, 181), (71, 152, 186), (75, 158, 191), (70, 148, 179), (65, 138, 167), (60, 128, 155), (56, 118, 143), (51, 108, 131), (46, 98, 119), (42, 88, 107), (37, 79, 95), (32, 69, 83), (28, 59, 71), (23, 49, 59), (18, 39, 47), (14, 29, 35), (9, 19, 23), (4, 9, 11), (0, 0, 0)]

def map_to_palette(value, palette):
    """Map a value to the nearest color in the palette."""
    index = int(value * (len(palette) - 1))
    return palette[index]

def draw_reaction_diffusion(screen, A, B):
    """Draw the reaction-diffusion grid to the screen using a color palette."""
    C = (A - B) * 255
    C = np.clip(C, 0, 255).astype(np.uint8)
    color_grid = np.zeros((grid_size[0], grid_size[1], 3), dtype=np.uint8)
    for i in range(grid_size[0]):
        for j in range(grid_size[1]):
            color_grid[i, j] = map_to_palette(C[i, j] / 255.0, palette)
    surface = pygame.surfarray.make_surface(color_grid)
    surface = pygame.transform.scale(surface, screen_size)
    screen.blit(surface, (0, 0))

def draw_waveform(screen, data):
    samples = np.frombuffer(data, dtype=np.int16)
    samples = samples / 32768.0
    waveform_height = screen_size[0] // 2
    samples = samples * waveform_height
    x_scale = screen_size[1] / len(samples)
    points = [(x * x_scale, waveform_height + samples[x]) for x in range(len(samples))]
    pygame.draw.lines(screen, (255, 255, 255), False, points)

def draw_slider(screen, name, slider_info):
    x, y, width, height = slider_info["rect"]
    pygame.draw.rect(screen, (255, 255, 255), (x, y, width, height), 2)
    # Position of the slider handle
    handle_x = x + int((slider_info["value"] - slider_info["min"]) / (slider_info["max"] - slider_info["min"]) * width)
    pygame.draw.rect(screen, (0, 255, 0), (handle_x - 5, y - 5, 10, height + 10))
    # Draw the slider name
    font = pygame.font.SysFont(None, 24)
    text = font.render(f"{name}: {slider_info['value']:.2f}", True, (255, 255, 255))
    screen.blit(text, (x, y - 30))

def draw_button(screen, rect, text):
    pygame.draw.rect(screen, (255, 255, 255), rect, 2)
    font = pygame.font.SysFont(None, 24)
    text_surface = font.render(text, True, (255, 255, 255))
    text_rect = text_surface.get_rect(center=rect.center)
    screen.blit(text_surface, text_rect)

def handle_sliders(event):
    global selected_slider
    if event.type == pygame.MOUSEBUTTONDOWN:
        for name, slider in sliders.items():
            if slider["rect"].collidepoint(event.pos):
                selected_slider = name
                break
    elif event.type == pygame.MOUSEBUTTONUP:
        selected_slider = None
    elif event.type == pygame.MOUSEMOTION and selected_slider:
        slider = sliders[selected_slider]
        x, y, width, height = slider["rect"]
        # Update slider value based on mouse position
        relative_x = max(0, min(event.pos[0] - x, width))
        slider["value"] = slider["min"] + (slider["max"] - slider["min"]) * (relative_x / width)

def handle_button(event):
    global show_waveform
    if event.type == pygame.MOUSEBUTTONDOWN and button_rect.collidepoint(event.pos):
        show_waveform = not show_waveform

def detect_beat(data):
    samples = np.frombuffer(data, dtype=np.int16)
    fft_spectrum = np.abs(scipy.fftpack.fft(samples))[:CHUNK // 2]
    freqs = np.fft.fftfreq(len(samples), 1.0 / RATE)[:CHUNK // 2]
    low_freq_spectrum = fft_spectrum[(freqs >= LOW_FREQ) & (freqs <= HIGH_FREQ)]
    energy = np.sum(low_freq_spectrum)
    energy_queue.append(energy)
    if len(energy_queue) == ROLLING_WINDOW:
        avg_energy = np.mean(energy_queue)
        if energy > sliders["THRESHOLD_MULTIPLIER"]["value"] * avg_energy:
            return True
    return False

# Initialize the reaction-diffusion grids with image data
image_path = 'wyr_image.png'  # Update this path to your image file
A, B = initialize_grids(image_path)

# Pre-simulate the reaction-diffusion process
for _ in range(pre_simulation_steps):
    A, B = update_reaction_diffusion(A, B, dA, dB, feed, kill)

# Main loop
running = True

while running:
    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            running = False
        elif event.type == pygame.KEYDOWN:
            if event.key == pygame.K_F11:
                is_fullscreen = not is_fullscreen
                if is_fullscreen:
                    screen = pygame.display.set_mode((0,0),pygame.FULLSCREEN)
                else:
                    screen = pygame.display.set_mode(screen_size)
        handle_sliders(event)
        handle_button(event)
    
    data = stream.read(CHUNK, exception_on_overflow=False)
    if detect_beat(data):
        current_time = time.time()
        if current_time - last_beat_time > sliders["COOLDOWN_TIME"]["value"]:
            pre_simulation_steps = np.random.randint(0, 100)
            # Reinitialize the grids with a new variation from the image
            A, B = initialize_grids(image_path)
            if not show_waveform:
                for _ in range(pre_simulation_steps):
                    A, B = update_reaction_diffusion(A, B, dA, dB, feed, kill)
            last_beat_time = current_time
    
    # Update the reaction-diffusion grid
    if not show_waveform:
        A, B = update_reaction_diffusion(A, B, dA, dB, feed, kill)
    
    # Draw the reaction-diffusion grid or waveform based on the toggle state
    screen.fill((0, 0, 0))
    if show_waveform:
        draw_waveform(screen, data)
    else:
        draw_reaction_diffusion(screen, A, B)

    for name, slider_info in sliders.items():
        draw_slider(screen, name, slider_info)
    
    draw_button(screen, button_rect, "Toggle View")

    pygame.display.flip()
    clock.tick(30)  # Lower frame rate to reduce CPU usage

# Cleanup
stream.stop_stream()
stream.close()
p.terminate()
pygame.quit()