use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::Arc;

pub fn initialize_audio() -> Arc<cpal::Stream> {
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
    Arc::new(stream)
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