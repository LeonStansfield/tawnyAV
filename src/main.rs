use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets};

#[macroquad::main("Shadertoy")]
async fn main() {
    let image = load_texture("resources/wyr_image.png").await.unwrap();

    let mut fragment_shader = DEFAULT_FRAGMENT_SHADER.to_string();
    let mut vertex_shader = DEFAULT_VERTEX_SHADER.to_string();

    let pipeline_params = PipelineParams {
        depth_write: true,
        depth_test: Comparison::LessOrEqual,
        ..Default::default()
    };

    let mut _material = load_material(
        ShaderSource::Glsl {
            vertex: &vertex_shader,
            fragment: &fragment_shader,
        },
        MaterialParams {
            pipeline_params,
            ..Default::default()
        },
    )
    .unwrap();
    let mut error: Option<String> = None;

    loop {
        clear_background(WHITE);

        draw_texture(&image, 0.0, 0.0, WHITE);

        let mut need_update = false;

        widgets::Window::new(hash!(), vec2(screen_width() - 490., 20.), vec2(470., 650.))
            .label("Shader")
            .ui(&mut *root_ui(), |ui| {
                widgets::TreeNode::new(hash!(), "Fragment shader")
                    .init_unfolded()
                    .ui(ui, |ui| {
                        if ui.editbox(hash!(), vec2(440., 200.), &mut fragment_shader) {
                            need_update = true;
                        };
                    });
                ui.tree_node(hash!(), "Vertex shader", |ui| {
                    if ui.editbox(hash!(), vec2(440., 300.), &mut vertex_shader) {
                        need_update = true;
                    };
                });

                if let Some(ref error) = error {
                    widgets::Label::new(error).multiline(14.0).ui(ui);
                }
            });

        if need_update {
            match load_material(
                ShaderSource::Glsl {
                    vertex: &vertex_shader,
                    fragment: &fragment_shader,
                },
                MaterialParams {
                    pipeline_params,
                    ..Default::default()
                },
            ) {
                Ok(new_material) => {
                    _material = new_material;
                    error = None;
                }
                Err(err) => {
                    error = Some(format!("{:#?}", err));
                }
            }
        }

        next_frame().await
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