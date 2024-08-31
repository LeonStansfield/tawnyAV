use macroquad::prelude::*;
use crate::scene::Scene;

pub struct InvertedScene {
    image: Texture2D,
    material: Material,
}

impl InvertedScene {
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
                fragment: INVERTED_FRAGMENT_SHADER,
            },
            MaterialParams {
                pipeline_params,
                ..Default::default()
            },
        )
        .unwrap();

        Self { image, material }
    }
}

impl Scene for InvertedScene {
    fn update(&mut self, _audio_data: &[f32]) {
        // Update scene based on audio data
    }

    fn draw(&self) {
        gl_use_material(&self.material);

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

const INVERTED_FRAGMENT_SHADER: &'static str = "#version 100
precision lowp float;

varying vec2 uv;

uniform sampler2D Texture;

void main() {
    vec4 color = texture2D(Texture, uv);
    gl_FragColor = vec4(1.0 - color.rgb, color.a);
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