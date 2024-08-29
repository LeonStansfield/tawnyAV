use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets};
use std::sync::Arc;

#[macroquad::main("Shadertoy")]
async fn main() {
    // Initialize the CPAL host
    let host = cpal::default_host();

    // Select the default input device (microphone)
    let device = host
        .default_input_device()
        .expect("Failed to find input device");

    println!("Selected input device: {}", device.name().unwrap());

    // Get the default input format
    let config = device.default_input_config().expect("Failed to get default input config");

    // Threshold for detecting peaks
    let peak_threshold: f32 = 0.7; // Adjust this value based on your needs

    // Create and run the input stream
    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => run_stream::<f32>(&device, &config.into(), peak_threshold),
        cpal::SampleFormat::I16 => run_stream::<i16>(&device, &config.into(), peak_threshold),
        cpal::SampleFormat::U16 => run_stream::<u16>(&device, &config.into(), peak_threshold),
        _ => panic!("Unsupported sample format"),
    };

    let stream = stream.expect("Failed to run the audio stream");

    // Play the stream
    stream.play().expect("Failed to play stream");

    // Store the stream to keep it running
    let _stream = Arc::new(stream);

    // Macroquad shader toy setup
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

fn run_stream<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    peak_threshold: f32,
) -> Result<cpal::Stream, cpal::BuildStreamError>
where
    T: cpal::Sample + cpal::SizedSample + dasp::Sample + dasp::sample::ToSample<f32>,
{
    let err_fn = |err| eprintln!("An error occurred on stream: {}", err);

    device.build_input_stream(
        config,
        move |data: &[T], _: &cpal::InputCallbackInfo| {
            for &sample in data.iter() {
                let normalized_sample = sample.to_sample::<f32>();
                // println!("Sample value: {}", normalized_sample);

                if normalized_sample > peak_threshold {
                    println!("Peak detected: {}", normalized_sample);
                }
            }
        },
        err_fn,
        None, // Latency hint
    )
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