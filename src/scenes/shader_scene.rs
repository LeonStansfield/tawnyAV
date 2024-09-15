use macroquad::prelude::*;
use macroquad::rand::gen_range;
use std::fs::File;
use std::io::Read;
use crate::scene::Scene;
use crate::globals;

pub struct ShaderScene {
    image: Texture2D,
    material: Material,
    time: f32,
    seed: f64,
    render_target: RenderTarget,
    render_width: f32,
    render_height: f32,
}

fn load_shader_str(shader_str: &str) -> String {
    let mut file = File::open(shader_str).unwrap();
    let mut shader = String::new();
    file.read_to_string(&mut shader).unwrap();
    shader
}

impl ShaderScene {
    pub async fn new(fragment_shader_filepath: &str) -> Self {
        let image = load_texture(*globals::LOGO_FILEPATH).await.unwrap();

        let pipeline_params = PipelineParams {
            depth_write: true,
            depth_test: Comparison::LessOrEqual,
            ..Default::default()
        };

        let fragment_shader = load_shader_str(fragment_shader_filepath);

        let material = load_material(
            ShaderSource::Glsl {
                vertex: DEFAULT_VERTEX_SHADER,
                fragment: &fragment_shader,
            },
            MaterialParams {
                pipeline_params,
                uniforms: vec![
                    UniformDesc::new("TextureSize", UniformType::Float2),
                     UniformDesc::new("Seed", UniformType::Float2),
                    UniformDesc::new("Time", UniformType::Float1),
                ],
                ..Default::default()
            },
        )
        .unwrap();

        // Define the render target
        let render_width = *globals::RENDER_WIDTH.lock().unwrap();
        let render_height = *globals::RENDER_HEIGHT.lock().unwrap();
        let render_target = render_target(render_width as u32, render_height as u32);

        Self { 
            image,
            material, 
            time: 0.0,
            seed: 0.0,
            render_target,
            render_width,
            render_height,
        }
    }
}

impl Scene for ShaderScene {
    fn update(&mut self) {
        self.time += get_frame_time();

        // Sync with beats, add randomness
        if *globals::BEAT_DETECTED.lock().unwrap() == true {
            let old_time = self.time;
            self.time = gen_range(60.0, 100.0);
            // If old time is within 15.0 of the new time, add a random amount
            if old_time > self.time - 15.0 {
                self.time += gen_range(10.0, 15.0);
            }

            self.material.set_uniform("Time", self.time);

            self.seed = gen_range(0.0, 100.0);
            self.material.set_uniform("Seed", self.seed);

            let mut beat_detected = globals::BEAT_DETECTED.lock().unwrap();
            *beat_detected = false;
        }
    }

    fn draw(&mut self) {
        // Set camera to the render target
        set_camera(&Camera2D {
            zoom: vec2(2.0 / self.render_width, 2.0 / self.render_height),
            target: vec2(self.render_width / 2.0, self.render_height / 2.0),
            render_target: Some(self.render_target.clone()),
            ..Default::default()
        });

        clear_background(BLACK);

        // Use the custom material
        gl_use_material(&self.material);

        // Set the uniforms
        let texture_size = vec2(self.render_width, self.render_height);
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

        // Reset camera to the screen
        set_default_camera();
        clear_background(WHITE);

        // Draw the render target texture to the screen
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