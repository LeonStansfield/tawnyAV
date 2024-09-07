use macroquad::prelude::*;
use macroquad::rand::gen_range;
use crate::scene::Scene;
use crate::globals;

pub struct MeltScene {
    image: Texture2D,
    material: Material,
    time: f32,
    seed: f32,
    render_target: RenderTarget,
    render_width: f32,
    render_height: f32,
}

impl MeltScene {
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
                fragment: MELT_FRAGMENT_SHADER,
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

impl Scene for MeltScene {
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

const MELT_FRAGMENT_SHADER: &'static str = "#version 120
precision lowp float;

varying vec2 uv;

uniform vec2 TextureSize;
uniform float Time;
uniform float Seed;
uniform sampler2D Texture;

// Palette represents points of a gradient of colours to use
const vec3 Palette[13] = vec3[13](
    vec3(1.0, 1.0, 1.0),         // (255, 255, 255)
    vec3(1.0, 0.894, 0.882),     // (255, 228, 225)
    vec3(1.0, 0.941, 0.961),     // (255, 240, 245)
    vec3(1.0, 0.894, 0.882),     // (255, 228, 225)
    vec3(0.863, 0.706, 0.784),   // (220, 180, 200)
    vec3(0.745, 0.471, 0.627),   // (190, 120, 160)
    vec3(0.671, 0.216, 0.439),   // (171, 55, 112)
    vec3(0.686, 0.004, 0.004),   // (175, 1, 1)
    vec3(0.549, 0.039, 0.039),   // (140, 10, 10)
    vec3(0.392, 0.059, 0.059),   // (100, 15, 15)
    vec3(0.235, 0.039, 0.039),   // (60, 10, 10)
    vec3(0.118, 0.020, 0.020),   // (30, 5, 5)
    vec3(0.0, 0.0, 0.0)          // (0, 0, 0)
);

// Function to interpolate between two colors
vec3 interpolate(vec3 color1, vec3 color2, float t) {
    return mix(color1, color2, t);
}

vec3 getPaletteColor(vec3 col) {
    float minDist = distance(col, Palette[0]);
    vec3 closestColor = Palette[0];
    
    // Iterate over the gradient defined by the palette
    for (int i = 0; i < 12; i++) {
        for (float t = 0.0; t <= 1.0; t += 0.01) {
            vec3 interpolatedColor = interpolate(Palette[i], Palette[i + 1], t);
            float dist = distance(col, interpolatedColor);
            if (dist < minDist) {
                minDist = dist;
                closestColor = interpolatedColor;
            }
        }
    }
    
    return closestColor;
}

float cosRange(float amt, float range, float minimum) {
    return (((1.0 + cos(amt * 3.14159 / 180.0)) * 0.5) * range) + minimum;
}

float hash(float x) {
    return fract(sin(x * 12.9898 * 78.233) * 0.005) * Seed;
}

void main() {
    vec2 uv = gl_FragCoord.xy / TextureSize;
    vec2 p = (2.0 * gl_FragCoord.xy - TextureSize) / max(TextureSize.x, TextureSize.y);
    
    float ct = cosRange(Time * (5.0 + hash(1.0)), 3.0, 1.1);
    float xBoost = cosRange(Time * (0.2 + hash(2.0)), 5.0, 5.0);
    float yBoost = cosRange(Time * (0.1 + hash(3.0)), 10.0, 5.0);
    float fScale = cosRange(Time * (15.5 + hash(4.0)), 1.25, 0.5);

    for (int i = 1; i < 40; i++) {
        float _i = float(i);
        vec2 newp = p;
        newp.x += 0.2 / _i * sin(_i * p.y + Time * cos(ct) * 0.5 / 20.0 + 0.005 * _i) * fScale + xBoost; 
        newp.y += 0.2 / _i * sin(_i * p.x + Time * ct * 0.3 / 40.0 + 0.03 * float(i + 15)) * fScale + yBoost;
        p = newp;
    }

    vec3 col = vec3(
        0.5 * sin(3.0 * p.x + hash(5.0)) + 0.5,
        0.5 * sin(3.0 * p.y + hash(6.0)) + 0.5,
        sin(p.x + p.y + hash(7.0))
    );

    col = getPaletteColor(col);

    // Add border
    float extrusion = (col.x + col.y + col.z) / 4.0;
    extrusion *= 1.5;
    
    vec4 textureColor = texture2D(Texture, uv);

    vec4 finalColor = (vec4(col, extrusion) + textureColor) * 0.5;
    vec3 col_adjusted = getPaletteColor(finalColor.rgb);

    gl_FragColor = vec4(col_adjusted, (1 / finalColor.a) * 0.5);
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