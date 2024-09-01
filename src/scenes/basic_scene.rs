use macroquad::prelude::*;
use crate::scene::Scene;

pub struct BasicScene {
    image: Texture2D,
    material: Material,
    render_target: RenderTarget,
    render_width: f32,
    render_height: f32,
}

impl BasicScene {
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
                fragment: DEFAULT_FRAGMENT_SHADER,
            },
            MaterialParams {
                pipeline_params,
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
            render_target,
            render_width,
            render_height
        }
    }
}

impl Scene for BasicScene {
    fn update(&mut self, _audio_data: &[f32]) {
        // Update scene based on audio data
    }

    fn draw(&mut self) {
        // Set camera to render target with a 1080p viewport
        set_camera(&Camera2D {
            zoom: vec2(2.0 / self.render_width, 2.0 / self.render_height),
            target: vec2(self.render_width / 2.0, self.render_height / 2.0),
            render_target: Some(self.render_target.clone()),
            ..Default::default()
        });

        clear_background(LIGHTGRAY);

        // Draw the scene onto the render target
        gl_use_material(&self.material);

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

        gl_use_default_material();

        // Reset camera to draw to the screen
        set_default_camera();
        clear_background(BLACK);

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

const DEFAULT_FRAGMENT_SHADER: &'static str = "#version 100
precision lowp float;

varying vec2 uv;

uniform sampler2D Texture;

void main() {
    gl_FragColor = texture2D(Texture, uv);
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
