use macroquad::prelude::*;
use crate::scene::Scene;

pub struct GameOfLifeScene {
    image: Texture2D,
    frame_buffer: RenderTarget,
    material: Material,
    time: f32,
}

impl GameOfLifeScene {
    pub async fn new() -> Self {
        let image = load_texture("resources/wyr_logo.png").await.unwrap();
        let frame_buffer = render_target(image.width() as u32, image.height() as u32);

        let pipeline_params = PipelineParams {
            depth_write: true,
            depth_test: Comparison::LessOrEqual,
            ..Default::default()
        };

        let material = load_material(
            ShaderSource::Glsl {
                vertex: DEFAULT_VERTEX_SHADER,
                fragment: GAME_OF_LIFE_FRAGMENT_SHADER,
            },
            MaterialParams {
                pipeline_params,
                uniforms: vec![
                    UniformDesc::new("TextureSize", UniformType::Float2),
                    UniformDesc::new("Time", UniformType::Float1),
                ],
                ..Default::default()
            },
        )
        .unwrap();

        Self {
            image,
            frame_buffer,
            material,
            time: 0.0,
        }
    }
}

impl Scene for GameOfLifeScene {
    fn update(&mut self, _audio_data: &[f32]) {
        // Update scene based on audio data
        self.time += get_frame_time();

        
    }

    fn draw(&self) {
        gl_use_material(&self.material);

        // Determine which texture to use as input for the shader
        let input_texture = if self.time <= 0.1 {
            &self.image // Use the initial image for the first frame
        } else {
            &self.frame_buffer.texture // Use the frame buffer for subsequent frames
        };

        let frame_buffer_copy = &self.frame_buffer;
        let texture_size = vec2(self.image.width() as f32, self.image.height() as f32);
        self.material.set_uniform("TextureSize", texture_size);
        self.material.set_uniform("Time", self.time);

        // Set the camera to render to the frame buffer
        set_camera(&Camera2D {
            render_target: Some(frame_buffer_copy.clone()),
            ..Default::default()
        });

        // Clear the frame buffer and draw the Game of Life
        clear_background(BLANK);

        draw_texture_ex(
            input_texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(texture_size),
                ..Default::default()
            },
        );

        // Reset the camera to render to the screen
        set_default_camera();

        // Draw the frame buffer to the screen
        draw_texture_ex(
            &self.frame_buffer.texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );

        gl_use_default_material();
    }
}

const GAME_OF_LIFE_FRAGMENT_SHADER: &'static str = "#version 100
precision lowp float;

varying vec2 uv;

uniform sampler2D Texture;
uniform vec2 TextureSize;
uniform float Time;

void main() {
    vec2 texel = 1.0 / TextureSize;

    vec4 color = texture2D(Texture, uv);
    float alive = color.r < 0.5 ? 1.0 : 0.0;

    float neighbors = 0.0;
    for (int x = -1; x <= 1; x++) {
        for (int y = -1; y <= 1; y++) {
            if (x == 0 && y == 0) continue;
            vec2 offset = vec2(float(x), float(y)) * texel;
            vec4 neighbor_color = texture2D(Texture, uv + offset);
            neighbors += neighbor_color.r < 0.5 ? 1.0 : 0.0;
        }
    }

    if (alive == 1.0) {
        if (neighbors < 2.0 || neighbors > 3.0) {
            alive = 0.0;
        }
    } else {
        if (neighbors == 3.0) {
            alive = 1.0;
        }
    }

    gl_FragColor = vec4(vec3(1.0 - alive), 1.0);
}
";

const DEFAULT_VERTEX_SHADER: &'static str = "#version 100
precision lowp float;

attribute vec3 position;
attribute vec2 texcoord;

varying vec2 uv;

uniform mat4 Model;
uniform mat4 Projection;

void main() {
    gl_Position = Projection * Model * vec4(position, 1);
    uv = texcoord;
}
";
