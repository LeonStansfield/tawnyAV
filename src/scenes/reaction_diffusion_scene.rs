use crate::scene::Scene;
use macroquad::prelude::*;

pub struct ReactionDiffusionScene {
    image: Texture2D,
    material: Material,
    render_target_a: RenderTarget,
    render_target_b: RenderTarget,
    render_width: f32,
    render_height: f32,
    time: f32,
    current_target: bool,
}

impl ReactionDiffusionScene {
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
                fragment: REACTION_DIFFUSION_FRAGMENT_SHADER,
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

        let render_width = 1920.0;
        let render_height = 1080.0;
        // Create two render targets for the reaction-diffusion process
        let render_target_a = render_target(render_width as u32, render_height as u32);
        let render_target_b = render_target(render_width as u32, render_height as u32);

        // Render the initial image to render_target_a
        set_camera(&Camera2D {
            zoom: vec2(2.0 / render_width, 2.0 / render_height),
            target: vec2(render_width / 2.0, render_height / 2.0),
            render_target: Some(render_target_a.clone()),
            ..Default::default()
        });

        clear_background(BLACK);
        draw_texture_ex(
            &image,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(render_width, render_height)),
                ..Default::default()
            },
        );
        set_default_camera();

        Self {
            image,
            material,
            render_target_a,
            render_target_b,
            render_width,
            render_height,
            time: 0.0,
            current_target: true,
        }
    }

    pub fn reset(&mut self) {
        // Reset the render targets
        let render_width = 1920.0;
        let render_height = 1080.0;
        self.render_target_a = render_target(render_width as u32, render_height as u32);
        self.render_target_b = render_target(render_width as u32, render_height as u32);

        // Clear the render target A
        set_camera(&Camera2D {
            zoom: vec2(2.0 / self.render_width, 2.0 / self.render_height),
            target: vec2(self.render_width / 2.0, self.render_height / 2.0),
            render_target: Some(self.render_target_a.clone()),
            ..Default::default()
        });
        clear_background(BLACK);

        // Render the initial image to render_target_a
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
        set_default_camera();

        // Reset time and current target
        self.time = 0.0;
        self.current_target = true;
    }
}

impl Scene for ReactionDiffusionScene {
    fn update(&mut self, _audio_data: &[f32]) {
        // Update time
        self.time += get_frame_time();

        // If time is greater than 10 seconds, reset the scene
        if self.time > 3.0 {
            self.reset();
        }
    }

    fn draw(&mut self) {
        let (current_render_target, next_render_target) = if self.current_target {
            (&self.render_target_a, &self.render_target_b)
        } else {
            (&self.render_target_b, &self.render_target_a)
        };

        // Set camera to current render target with a 1080p viewport
        set_camera(&Camera2D {
            zoom: vec2(2.0 / self.render_width, 2.0 / self.render_height),
            target: vec2(self.render_width / 2.0, self.render_height / 2.0),
            render_target: Some(next_render_target.clone()),
            ..Default::default()
        });

        // Clear the render target
        clear_background(BLACK);

        // Use the custom material (shader)
        gl_use_material(&self.material);

        let texture_size = vec2(self.render_width, self.render_height);
        self.material.set_uniform("TextureSize", texture_size);
        self.material.set_uniform("Time", self.time);

        // Draw the texture onto the next render target
        draw_texture_ex(
            &current_render_target.texture,
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

        // Draw the final render target texture to the screen, stretched to fit the screen dimensions
        draw_texture_ex(
            &next_render_target.texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );

        // Swap the targets for the next frame
        self.current_target = !self.current_target;
    }
}

const REACTION_DIFFUSION_FRAGMENT_SHADER: &'static str = "#version 100
precision lowp float;

varying vec2 uv;

uniform sampler2D Texture;
uniform vec2 TextureSize;
uniform float Time;

// Parameters for reaction-diffusion
const float dA = 1.0;
const float dB = 0.5;
const float feed = 0.055;
const float kill = 0.062;

vec2 laplacian(vec2 p) {
    vec2 sum = vec2(0.0);
    sum += texture2D(Texture, uv + vec2(-1.0, 0.0) / TextureSize).xy * 0.2;
    sum += texture2D(Texture, uv + vec2(1.0, 0.0) / TextureSize).xy * 0.2;
    sum += texture2D(Texture, uv + vec2(0.0, -1.0) / TextureSize).xy * 0.2;
    sum += texture2D(Texture, uv + vec2(0.0, 1.0) / TextureSize).xy * 0.2;
    sum -= texture2D(Texture, uv).xy * 1.0;
    return sum;
}

void main() {
    vec4 state = texture2D(Texture, uv);
    vec2 lap = laplacian(state.xy);
    float a = state.x + (dA * lap.x - state.x * state.y * state.y + feed * (1.0 - state.x)) * 0.5;
    float b = state.y + (dB * lap.y + state.x * state.y * state.y - (kill + feed) * state.y) * 0.5;
    a = clamp(a, 0.0, 1.0);
    b = clamp(b, 0.0, 1.0);
    gl_FragColor = vec4(a, b, b - a, 1.0);
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
