use macroquad::prelude::*;
use macroquad::rand::gen_range;
use crate::scene::Scene;
use crate::globals;

pub struct CloudScene {
    image: Texture2D,
    material: Material,
    time: f32,
    render_target: RenderTarget,
    render_width: f32,
    render_height: f32,
}

impl CloudScene {
    pub async fn new() -> Self {
        let image = load_texture(*globals::LOGO_FILEPATH).await.unwrap();

        let pipeline_params = PipelineParams {
            depth_write: true,
            depth_test: Comparison::LessOrEqual,
            ..Default::default()
        };

        let material = load_material(
            ShaderSource::Glsl {
                vertex: DEFAULT_VERTEX_SHADER,
                fragment: CLOUD_SCENE_FRAGMENT_SHADER,
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

        let render_width = *globals::RENDER_WIDTH.lock().unwrap();
        let render_height = *globals::RENDER_HEIGHT.lock().unwrap();
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
}

impl Scene for CloudScene {
    fn update(&mut self) {
        self.time += get_frame_time();

        if *globals::BEAT_DETECTED.lock().unwrap() == true {
            self.time = gen_range(0.0, 50.0);

            let mut beat_detected = globals::BEAT_DETECTED.lock().unwrap();
            *beat_detected = false;
        }
    }

    fn draw(&mut self) {
        set_camera(&Camera2D {
            zoom: vec2(2.0 / self.render_width, 2.0 / self.render_height),
            target: vec2(self.render_width / 2.0, self.render_height / 2.0),
            render_target: Some(self.render_target.clone()),
            ..Default::default()
        });

        clear_background(BLACK);

        gl_use_material(&self.material);

        let texture_size = vec2(self.render_width, self.render_height);
        self.material.set_uniform("TextureSize", texture_size);
        self.material.set_uniform("Time", self.time);

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

        set_default_camera();
        clear_background(WHITE);

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

const CLOUD_SCENE_FRAGMENT_SHADER: &'static str = "#version 120
precision mediump float;

varying vec2 uv;

uniform vec2 TextureSize;
uniform float Time;
uniform sampler2D Texture;

const float cloudscale = 1.1;
const float speed = 0.03;
const float clouddark = 0.5;
const float cloudlight = 0.3;
const float cloudcover = 0.2;
const float cloudalpha = 8.0;
const float skytint = 0.5;
const vec3 skycolour1 = vec3(0.2, 0.4, 0.6);
const vec3 skycolour2 = vec3(0.4, 0.7, 1.0);

// Simplex noise function or a simpler noise function for this example
float noise(vec2 p) {
    return fract(sin(dot(p, vec2(2.9898, 3.233))) * 1.0);
}

void main() {
    vec4 fragColor;
    vec2 p = uv * TextureSize / TextureSize;
    vec2 uv = p * cloudscale - Time * speed;
    uv = mod(uv, 1.0);  // Wrap the UV coordinates
    
    float cloudPattern = noise(uv * 10.0); // Adjust scale of noise
    
    vec3 skycolour = mix(skycolour2, skycolour1, p.y);
    vec3 cloudcolour = vec3(1.1, 1.1, 0.9) * clamp(clouddark + cloudlight * cloudPattern, 0.0, 1.0);
    
    float cloudAlpha = cloudcover + cloudalpha * cloudPattern;
    vec3 result = mix(skycolour, clamp(skytint * skycolour + cloudcolour, 0.0, 1.0), clamp(cloudAlpha, 0.0, 1.0));
    
    fragColor = vec4(result, 1.0);
    gl_FragColor = fragColor;
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
    gl_Position = Projection * Model * vec4(position, 1.0);
    uv = texcoord;
}
";