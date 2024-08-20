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

class ShaderTest:
    def __init__(self, vertex_shader_path, fragment_shader_path):
        self.vertex_shader_source = load_shader_source(vertex_shader_path)
        self.fragment_shader_source = load_shader_source(fragment_shader_path)
        self.shader_program = self.compile_shaders()
        self.vbo = self.setup_vertex_data()
        self.start_time = time.time()

    def compile_shaders(self):
        return compileProgram(
            compileShader(self.vertex_shader_source, GL_VERTEX_SHADER),
            compileShader(self.fragment_shader_source, GL_FRAGMENT_SHADER)
        )

    def setup_vertex_data(self):
        vertices = np.array([
            [-1, -1],
            [1, -1],
            [1,  1],
            [-1, -1],
            [1,  1],
            [-1,  1],
        ], dtype=np.float32)

        vbo = glGenBuffers(1)
        glBindBuffer(GL_ARRAY_BUFFER, vbo)
        glBufferData(GL_ARRAY_BUFFER, vertices.nbytes, vertices, GL_STATIC_DRAW)

        position = glGetAttribLocation(self.shader_program, 'position')
        glEnableVertexAttribArray(position)
        glVertexAttribPointer(position, 2, GL_FLOAT, GL_FALSE, 0, None)

        return vbo

    def draw(self, screen, screen_size):
        glUseProgram(self.shader_program)

        elapsed_time = time.time() - self.start_time

        resolution = glGetUniformLocation(self.shader_program, 'resolution')
        glUniform2f(resolution, *screen_size)

        time_uniform = glGetUniformLocation(self.shader_program, 'time')
        glUniform1f(time_uniform, elapsed_time)

        glClear(GL_COLOR_BUFFER_BIT)
        glDrawArrays(GL_TRIANGLES, 0, 6)

class ShaderTestScene(Scene):
    def __init__(self, shader_test, name):
        self.shader_test = shader_test
        self.name = name

    def draw(self, screen, data):
        self.shader_test.draw(screen, screen.get_size())
    
    def handle_beat(self):
        pass