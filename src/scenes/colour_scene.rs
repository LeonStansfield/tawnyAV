use macroquad::prelude::*;
use crate::scene::Scene;

pub struct ColourScene {
    image: Texture2D,
    material: Material,
    time: f32,
}

impl ColourScene {
    pub async fn new() -> Self {
        let image = load_texture("resources/wyr_logo.png").await.unwrap();

        let pipeline_params = PipelineParams {
            depth_write: true,
            depth_test: Comparison::LessOrEqual,
            ..Default::default()
        };

        let material = load_material(
            ShaderSource::Glsl {
                vertex: DEFAULT_VERTEX_SHADER,
                fragment: COLORFUL_FRAGMENT_SHADER,
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

        Self { image, material, time: 0.0 }
    }
}

impl Scene for ColourScene {
    fn update(&mut self, _audio_data: &[f32]) {
        // Update scene based on audio data
        self.time += get_frame_time();
    }

    fn draw(&self) {
        gl_use_material(&self.material);

        let texture_size = vec2(self.image.width() as f32, self.image.height() as f32);
        self.material.set_uniform("TextureSize", texture_size);
        self.material.set_uniform("Time", self.time);

        draw_texture_ex(
            &self.image,
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

const COLORFUL_FRAGMENT_SHADER: &'static str = "#version 100
precision lowp float;

varying vec2 uv;

uniform sampler2D Texture;
uniform vec2 TextureSize;
uniform float Time;

void main() {
    vec4 textureColor = texture2D(Texture, uv);
    float r = 0.5 + 0.5 * sin(Time + 0.0);
    float g = 0.5 + 0.5 * sin(Time + 2.0);
    float b = 0.5 + 0.5 * sin(Time + 4.0);
    vec4 backgroundColor = vec4(r, g, b, 1.0);
    gl_FragColor = mix(backgroundColor, textureColor, textureColor.a);
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