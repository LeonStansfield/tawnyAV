import pygame
from pygame.locals import *
from OpenGL.GL import *
from OpenGL.GL.shaders import compileProgram, compileShader
import numpy as np
import time
from .scene import Scene

def load_shader_source(file_path):
    with open(file_path, 'r') as shader_file:
        return shader_file.read()

class ShaderTestScene(Scene):
    def __init__(self, shader_test, name):
        self.shader_test = shader_test
        self.name = name

    def draw(self, screen, data):
        self.shader_test.draw(screen, screen.get_size())
    
    def handle_beat(self):
        pass

class ShaderTest:
    def __init__(self):
        pygame.init()
        self.screen = pygame.display.set_mode((800, 600), DOUBLEBUF | OPENGL)
        pygame.display.set_caption('Pygame Shaders')

        vertex_shader_source = load_shader_source('shaders/shader_test/vertex.glsl')
        fragment_shader_source = load_shader_source('shaders/shader_test/fragment.glsl')

        self.shader_program = compileProgram(
            compileShader(vertex_shader_source, GL_VERTEX_SHADER),
            compileShader(fragment_shader_source, GL_FRAGMENT_SHADER)
        )

        vertices = np.array([
            [-1, -1],
            [1, -1],
            [1,  1],
            [-1, -1],
            [1,  1],
            [-1,  1],
        ], dtype=np.float32)

        self.vbo = glGenBuffers(1)
        glBindBuffer(GL_ARRAY_BUFFER, self.vbo)
        glBufferData(GL_ARRAY_BUFFER, vertices.nbytes, vertices, GL_STATIC_DRAW)

        position = glGetAttribLocation(self.shader_program, 'position')
        glEnableVertexAttribArray(position)
        glVertexAttribPointer(position, 2, GL_FLOAT, GL_FALSE, 0, None)

        glUseProgram(self.shader_program)
        self.start_time = time.time()

    def update(self):
        pass

    def draw(self, screen, screen_size):
        glUseProgram(self.shader_program)  # Ensure the shader program is in use

        elapsed_time = time.time() - self.start_time

        resolution = glGetUniformLocation(self.shader_program, 'resolution')
        glUniform2f(resolution, screen_size[0], screen_size[1])

        time_uniform = glGetUniformLocation(self.shader_program, 'time')
        glUniform1f(time_uniform, elapsed_time)

        glClear(GL_COLOR_BUFFER_BIT)
        glDrawArrays(GL_TRIANGLES, 0, 6)
        pygame.display.flip()
        pygame.time.wait(10)
