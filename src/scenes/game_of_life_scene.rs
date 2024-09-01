use crate::scene::Scene;
use macroquad::prelude::*;

pub struct GameOfLifeScene {
    material: Material,
    image: Texture2D,
    time: f32,
    render_target: RenderTarget,
    render_width: f32,
    render_height: f32,
}

impl GameOfLifeScene {
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
                fragment: CONWAY_FRAGMENT_SHADER,
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
        let render_width = 1920.0;
        let render_height = 1080.0;
        let render_target = render_target(render_width as u32, render_height as u32);

        // Draw the initial state to the render target
        set_camera(&Camera2D {
            zoom: vec2(2.0 / render_width, 2.0 / render_height),
            target: vec2(render_width / 2.0, render_height / 2.0),
            render_target: Some(render_target.clone()),
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
            material,
            image,
            time: 0.0,
            render_target,
            render_width,
            render_height,
        }
    }

    pub fn reset(&mut self) {
        self.time = 0.0;

        // Draw the initial state to the render target
        set_camera(&Camera2D {
            zoom: vec2(2.0 / self.render_width, 2.0 / self.render_height),
            target: vec2(self.render_width / 2.0, self.render_height / 2.0),
            render_target: Some(self.render_target.clone()),
            ..Default::default()
        });

        clear_background(BLACK);

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
    }
}

impl Scene for GameOfLifeScene {
    fn update(&mut self, _audio_data: &[f32]) {
        self.time += get_frame_time();

        // Reset the simulation after 3 seconds, using the original image
        if self.time > 3.0 {
            self.reset();
        }
    }

    fn draw(&mut self) {
        // Set the camera to render to the render target
        set_camera(&Camera2D {
            zoom: vec2(2.0 / self.render_width, 2.0 / self.render_height),
            target: vec2(self.render_width / 2.0, self.render_height / 2.0),
            render_target: Some(self.render_target.clone()),
            ..Default::default()
        });

        clear_background(BLACK);

        // Use the custom material (shader)
        gl_use_material(&self.material);

        let texture_size = vec2(self.render_target.texture.width() as f32, self.render_target.texture.height() as f32);
        self.material.set_uniform("TextureSize", texture_size);
        self.material.set_uniform("Time", self.time);

        // Draw the render target's texture back onto itself
        draw_texture_ex(
            &self.render_target.texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(self.render_width, self.render_height)),
                ..Default::default()
            },
        );

        // Use the default material (standard rendering)
        gl_use_default_material();

        // Reset the camera and draw the final output to the screen
        set_default_camera();
        clear_background(WHITE);

        // Draw the render target texture (now containing the updated state) to the screen
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

// Shaders remain the same
const CONWAY_FRAGMENT_SHADER: &'static str = "#version 100
precision lowp float;

varying vec2 uv;

uniform sampler2D Texture;
uniform vec2 TextureSize;
uniform float Time;

float get_cell_state(vec2 coords) {
    vec4 color = texture2D(Texture, coords / TextureSize);
    return color.r > 0.5 ? 1.0 : 0.0; // Use red channel to determine the cell state
}

float get_neighbor_count(vec2 coords) {
    float count = 0.0;
    vec2 offset[8];
    offset[0] = vec2(-1.0, -1.0);
    offset[1] = vec2(0.0, -1.0);
    offset[2] = vec2(1.0, -1.0);
    offset[3] = vec2(-1.0, 0.0);
    offset[4] = vec2(1.0, 0.0);
    offset[5] = vec2(-1.0, 1.0);
    offset[6] = vec2(0.0, 1.0);
    offset[7] = vec2(1.0, 1.0);
    
    for (int i = 0; i < 8; i++) {
        count += get_cell_state(coords + offset[i]);
    }
    
    return count;
}

void main() {
    vec2 pixel_coords = uv * TextureSize;
    
    float current_state = get_cell_state(pixel_coords);
    float neighbors = get_neighbor_count(pixel_coords);
    
    float new_state = current_state;
    
    if (current_state == 1.0) {
        if (neighbors < 2.0 || neighbors > 3.0) {
            new_state = 0.0; // Cell dies
        }
    } else {
        if (neighbors == 3.0) {
            new_state = 1.0; // Cell becomes alive
        }
    }
    
    vec4 color = vec4(new_state, new_state, new_state, 1.0);
    gl_FragColor = color;
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
