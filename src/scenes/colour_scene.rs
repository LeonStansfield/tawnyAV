use macroquad::prelude::*;
use macroquad::rand::gen_range;
use crate::scene::Scene;
use crate::globals;

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
        // Set the scenes properties up
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

        // Define the render target
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

impl Scene for ColourScene {
    fn update(&mut self) {
        // Update time
        self.time += get_frame_time();

        // Update scene based on audio data
        if *globals::BEAT_DETECTED.lock().unwrap() == true {
            // Do thing
            self.time = gen_range(0.0, 10.0);

            let mut beat_detected = globals::BEAT_DETECTED.lock().unwrap();
            *beat_detected = false;
        }
    }

    fn draw(&mut self) {
        // Set camera to render target
        set_camera(&Camera2D {
            zoom: vec2(2.0 / self.render_width, 2.0 / self.render_height),
            target: vec2(self.render_width / 2.0, self.render_height / 2.0),
            render_target: Some(self.render_target.clone()),
            ..Default::default()
        });

        clear_background(LIGHTGRAY);

        // Use the custom material
        gl_use_material(&self.material);

        // Set the uniforms
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

// TODO: Move shaders to a separate file
const COLORFUL_FRAGMENT_SHADER: &'static str = "#version 120
precision lowp float;

varying vec2 uv;

uniform sampler2D Texture;
uniform vec2 TextureSize;
uniform float Time;

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

void main() {
    vec4 textureColor = texture2D(Texture, uv);
    float r = 0.5 + 0.5 * sin(Time + 0.0);
    float g = 0.5 + 0.5 * sin(Time + 2.0);
    float b = 0.5 + 0.5 * sin(Time + 4.0);
    vec3 colour =  vec3(r, g, b);
    colour = getPaletteColor(colour);
    vec4 backgroundColor = vec4(colour, 1.0);

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
