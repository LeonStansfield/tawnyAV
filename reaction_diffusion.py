import numpy as np
import pygame
from PIL import Image

# Reaction-Diffusion parameters
dA = 0.07
dB = 0.13
feed = 0.055
kill = 0.062
grid_size = (200, 200)  # Grid size
screen_size = (800, 800)
pre_simulation_steps = np.random.randint(0, 100)  # Number of pre-simulation steps

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
    for _ in range(np.random.randint(50, 250)):
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

def draw_reaction_diffusion(screen, A, B):
    """Draw the reaction-diffusion grid to the screen."""
    C = (A - B) * 255
    C = np.clip(C, 0, 255).astype(np.uint8)
    surface = pygame.surfarray.make_surface(C)
    surface = pygame.transform.scale(surface, screen_size)
    screen.blit(surface, (0, 0))

# Initialize Pygame
pygame.init()
screen = pygame.display.set_mode(screen_size)
pygame.display.set_caption("Reaction-Diffusion Visualization")
clock = pygame.time.Clock()

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
        elif event.type == pygame.KEYDOWN and event.key == pygame.K_RETURN:
            pre_simulation_steps = np.random.randint(0, 100)
            # Reinitialize the grids with a new variation from the image
            A, B = initialize_grids(image_path)
            for _ in range(pre_simulation_steps):
                A, B = update_reaction_diffusion(A, B, dA, dB, feed, kill)
    
    # Update the reaction-diffusion grid
    A, B = update_reaction_diffusion(A, B, dA, dB, feed, kill)
    
    # Draw the reaction-diffusion grid
    draw_reaction_diffusion(screen, A, B)
    
    pygame.display.flip()
    clock.tick(30)  # Lower frame rate to reduce CPU usage

# Cleanup
pygame.quit()
