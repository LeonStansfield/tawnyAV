use macroquad::prelude::*;
use macroquad::rand::gen_range;
use crate::scene::Scene;
use crate::globals;

pub struct GlitchMeltScene {
    image: Texture2D,
    material: Material,
    time: f32,
    seed: f32,
    render_target: RenderTarget,
    render_width: f32,
    render_height: f32,
}

impl GlitchMeltScene {
    pub async fn new() -> Self {
        // Load the logo image
        let image = load_texture(*globals::LOGO_FILEPATH).await.unwrap();

        // Define the pipeline parameters
        let pipeline_params = PipelineParams {
            depth_write: true,
            depth_test: Comparison::LessOrEqual,
            ..Default::default()
        };
        
        // Define the custom material
        let material = load_material(
            ShaderSource::Glsl {
                vertex: DEFAULT_VERTEX_SHADER,
                fragment: GLITCH_MELT_SCENE,
            },
            MaterialParams {
                pipeline_params,
                uniforms: vec![
                    UniformDesc::new("TextureSize", UniformType::Float2),
                    UniformDesc::new("Time", UniformType::Float1),
                    UniformDesc::new("Seed", UniformType::Float1),
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

impl Scene for GlitchMeltScene {
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

            self.seed = gen_range(0.0, 100.0);
            self.material.set_uniform("Seed", self.time);

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

const GLITCH_MELT_SCENE: &'static str = "#version 100
precision lowp float;

varying vec2 uv;

uniform vec2 TextureSize;
uniform float Time;
uniform sampler2D Texture;

float cosRange(float amt, float range, float minimum) {
    return (((1.0 + cos(amt * 3.14159 / 180.0)) * 0.5) * range) + minimum;
}

void main() {
    vec2 uv = gl_FragCoord.xy / TextureSize;
    vec2 p = (2.0 * gl_FragCoord.xy - TextureSize) / max(TextureSize.x, TextureSize.y);
    
    float ct = cosRange(Time * 5.0, 3.0, 1.1);
    float xBoost = cosRange(Time * 0.2, 5.0, 5.0);
    float yBoost = cosRange(Time * 0.1, 10.0, 5.0);
    float fScale = cosRange(Time * 15.5, 1.25, 0.5);

    for (int i = 1; i < 40; i++) {
        float _i = float(i);
        vec2 newp = p;
        newp.x += 0.2 / _i * sin(_i * p.y + Time * cos(ct) * 0.5 / 20.0 + 0.005 * _i) * fScale + xBoost; 
        newp.y += 0.2 / _i * sin(_i * p.x + Time * ct * 0.3 / 40.0 + 0.03 * float(i + 15)) * fScale + yBoost;
        p = newp;
    }

    vec3 col = vec3(
        0.5 * sin(3.0 * p.x) + 0.5,
        0.5 * sin(3.0 * p.y) + 0.5,
        sin(p.x + p.y)
    );
    col *= 0.975;

    // Add border
    float extrusion = (col.x + col.y + col.z) / 4.0;
    extrusion *= 1.5;
    
    vec4 textureColor = texture2D(Texture, uv);
    gl_FragColor = vec4(col, extrusion) + textureColor * 0.5;
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