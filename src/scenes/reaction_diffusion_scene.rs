use crate::scene::Scene;
use ::rand::{thread_rng, Rng};
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
        let image = load_texture("resources/wyr_logo.png").await.unwrap(); // TODO: Set to a global constant

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

        // Define the render targets
        let render_width = 1920.0; // TODO: Set to a global constant
        let render_height = 1080.0;
        let render_target_a = render_target(render_width as u32, render_height as u32);
        let render_target_b = render_target(render_width as u32, render_height as u32);

        // Draw noise and texture to render targets
        Self::draw_noise_and_texture(&image, &render_target_a, &render_target_b, render_width, render_height);

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

    fn draw_noise_and_texture(
        image: &Texture2D,
        render_target_a: &RenderTarget,
        render_target_b: &RenderTarget,
        render_width: f32,
        render_height: f32,
    ) {
        set_camera(&Camera2D {
            zoom: vec2(2.0 / render_width, 2.0 / render_height),
            target: vec2(render_width / 2.0, render_height / 2.0),
            render_target: Some(render_target_a.clone()),
            ..Default::default()
        });
    
        // Add random noise with larger squares
        let mut rng = thread_rng();
        let num_squares = rng.gen_range(50..500);
        for _ in 0..num_squares {
            let x = rng.gen_range(0.0..render_width);
            let y = rng.gen_range(0.0..render_height);
            let size = rng.gen_range(1.0..10.0); // Random size for the squares
            let noise = if rng.gen_bool(0.5) {
                1.0 // Bright white
            } else {
                0.0 // Black
            };
            draw_rectangle(
                x,
                y,
                size,
                size,
                Color::new(noise, noise, noise, 1.0),
            );
        }
    
        set_camera(&Camera2D {
            zoom: vec2(2.0 / render_width, 2.0 / render_height),
            target: vec2(render_width / 2.0, render_height / 2.0),
            render_target: Some(render_target_b.clone()),
            ..Default::default()
        });
        clear_background(BLACK);
    
        // Add random noise with larger squares
        for _ in 0..num_squares {
            let x = rng.gen_range(0.0..render_width);
            let y = rng.gen_range(0.0..render_height);
            let size = rng.gen_range(1.0..10.0); // Random size for the squares
            let noise = if rng.gen_bool(0.5) {
                1.0 // Bright white
            } else {
                0.0 // Black
            };
            draw_rectangle(
                x,
                y,
                size,
                size,
                Color::new(noise, noise, noise, 1.0),
            );
        }
    
        // Draw image
        draw_texture_ex(
            image,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(render_width, render_height)),
                ..Default::default()
            },
        );
    
        set_default_camera();
    }

    pub fn reset(&mut self) {
        // Reset the render targets
        let render_width = 1920.0; // TODO: Set to a global constant
        let render_height = 1080.0;
        self.render_target_a = render_target(render_width as u32, render_height as u32);
        self.render_target_b = render_target(render_width as u32, render_height as u32);
    
        // Draw noise and texture to render targets
        Self::draw_noise_and_texture(&self.image, &self.render_target_a, &self.render_target_b, self.render_width, self.render_height);
    
        // Reset time and current target
        self.time = 0.0;
        self.current_target = true;
    }
}

impl Scene for ReactionDiffusionScene {
    fn update(&mut self, _audio_data: &[f32]) {
        // Update time
        self.time += get_frame_time();

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

        // Set camera to current render target
        set_camera(&Camera2D {
            zoom: vec2(2.0 / self.render_width, 2.0 / self.render_height),
            target: vec2(self.render_width / 2.0, self.render_height / 2.0),
            render_target: Some(next_render_target.clone()),
            ..Default::default()
        });

        clear_background(BLACK);

        // Use the custom material
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

        // Draw the final render target texture to the screen
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

// TODO: Move shaders to a separate file
const REACTION_DIFFUSION_FRAGMENT_SHADER: &'static str = "#version 100
precision lowp float;

varying vec2 uv;

uniform sampler2D Texture;
uniform vec2 TextureSize;
uniform float Time;

// Parameters for reaction-diffusion
const float dA = 1.0;
const float dB = 0.6;
const float feed = 0.0367;
const float kill = 0.0649;

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
