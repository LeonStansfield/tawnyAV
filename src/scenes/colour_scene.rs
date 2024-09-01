use macroquad::prelude::*;
use crate::scene::Scene;

pub struct ColourScene {
    image: Texture2D,
    material: Material,
    time: f32,
    render_target: RenderTarget,
    render_width: f32,
    render_height: f32,
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

        // Define the render target resolution (1080p)
        let render_width = 1920.0;
        let render_height = 1080.0;

        // Create the render target with 1080p resolution
        let render_target = render_target(render_width as u32, render_height as u32);

        Self { 
            image, 
            material, 
            time: 0.0,
            render_target,
            render_width,
            render_height,
        }
    }

    pub fn reset(&mut self) {
        self.time = 0.0;
    }
}

impl Scene for ColourScene {
    fn update(&mut self, _audio_data: &[f32]) {
        // Update scene based on audio data
        self.time += get_frame_time();

        // Reset time when it reaches a certain value
        if self.time > 10.0 {
            self.reset();
        }
    }

    fn draw(&mut self) {
        // Set camera to render target with a 1080p viewport
        set_camera(&Camera2D {
            zoom: vec2(2.0 / self.render_width, 2.0 / self.render_height),
            target: vec2(self.render_width / 2.0, self.render_height / 2.0),
            render_target: Some(self.render_target.clone()),
            ..Default::default()
        });

        // Clear the render target with a background color
        clear_background(LIGHTGRAY);

        // Use the custom material (shader)
        gl_use_material(&self.material);

        let texture_size = vec2(self.image.width() as f32, self.image.height() as f32);
        self.material.set_uniform("TextureSize", texture_size);
        self.material.set_uniform("Time", self.time);

        // Draw the texture onto the render target
        draw_texture_ex(
            &self.image,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(self.render_width, self.render_height)),
                ..Default::default()
            },
        );

        // Use the default material
        gl_use_default_material();

        // Reset camera to draw to the screen
        set_default_camera();
        clear_background(WHITE);

        // Draw the render target texture to the screen, stretched to fit the screen dimensions
        draw_texture_ex(
            &self.render_target.texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );
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
