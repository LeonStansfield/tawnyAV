import pygame
from pygame.locals import *
from OpenGL.GL import *
from OpenGL.GL.shaders import compileProgram, compileShader
import numpy as np
import time

def load_shader_source(file_path):
    with open(file_path, 'r') as shader_file:
        return shader_file.read()

# Load shaders from files
vertex_shader_source = load_shader_source('shaders/shader_test/vertex.glsl')
fragment_shader_source = load_shader_source('shaders/shader_test/fragment.glsl')

# Initialize Pygame and OpenGL
pygame.init()
screen = pygame.display.set_mode((800, 600), DOUBLEBUF | OPENGL)
pygame.display.set_caption('Pygame Shaders')

# Compile shaders and program
shader_program = compileProgram(
    compileShader(vertex_shader_source, GL_VERTEX_SHADER),
    compileShader(fragment_shader_source, GL_FRAGMENT_SHADER)
)

# Set up vertex data (two triangles to cover the screen)
vertices = np.array([
    [-1, -1],
    [1, -1],
    [1,  1],
    [-1, -1],
    [1,  1],
    [-1,  1],
], dtype=np.float32)

# Create a Vertex Buffer Object and upload the vertex data
vbo = glGenBuffers(1)
glBindBuffer(GL_ARRAY_BUFFER, vbo)
glBufferData(GL_ARRAY_BUFFER, vertices.nbytes, vertices, GL_STATIC_DRAW)

# Specify the layout of the vertex data
position = glGetAttribLocation(shader_program, 'position')
glEnableVertexAttribArray(position)
glVertexAttribPointer(position, 2, GL_FLOAT, GL_FALSE, 0, None)

# Use the shader program
glUseProgram(shader_program)

# Main loop
running = True
start_time = time.time()
while running:
    for event in pygame.event.get():
        if event.type == QUIT or (event.type == KEYDOWN and event.key == K_ESCAPE):
            running = False

    # Calculate the elapsed time
    elapsed_time = time.time() - start_time

    # Set uniform variables for resolution and time
    resolution = glGetUniformLocation(shader_program, 'resolution')
    glUniform2f(resolution, 800, 600)

    time_uniform = glGetUniformLocation(shader_program, 'time')
    glUniform1f(time_uniform, elapsed_time)

    # Clear the screen
    glClear(GL_COLOR_BUFFER_BIT)

    # Draw the triangles
    glDrawArrays(GL_TRIANGLES, 0, 6)

    # Swap the front and back buffer
    pygame.display.flip()
    pygame.time.wait(10)

# Cleanup
pygame.quit()
