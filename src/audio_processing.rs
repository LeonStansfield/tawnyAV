use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::collections::VecDeque;

use crate::globals;

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

    // Create and run the input stream
    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => run_stream::<f32>(&device, &config.into()),
        cpal::SampleFormat::I16 => run_stream::<i16>(&device, &config.into()),
        cpal::SampleFormat::U16 => run_stream::<u16>(&device, &config.into()),
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
) -> Result<cpal::Stream, cpal::BuildStreamError>
where
    T: cpal::Sample + cpal::SizedSample + dasp::Sample + dasp::sample::ToSample<f32>,
{
    let peak_threshold_multiplier: f32 = 1.5; // Multiplier for dynamic threshold
    let cooldown_time = Duration::from_millis(300); // 300 ms cooldown between peaks
    let rolling_window_size = 50; // Size of the rolling window for average energy

    let err_fn = |err| eprintln!("An error occurred on stream: {}", err);

    let energy_queue = Arc::new(Mutex::new(VecDeque::with_capacity(rolling_window_size)));
    let last_beat_time = Arc::new(Mutex::new(Instant::now() - cooldown_time));

    let energy_queue_clone = Arc::clone(&energy_queue);
    let last_beat_time_clone = Arc::clone(&last_beat_time);

    device.build_input_stream(
        config,
        move |data: &[T], _: &cpal::InputCallbackInfo| {
            let mut energy_queue = energy_queue_clone.lock().unwrap();
            let mut last_beat_time = last_beat_time_clone.lock().unwrap();

            let mut energy: f32 = 0.0;
            for &sample in data.iter() {
                let normalized_sample = sample.to_sample::<f32>();
                energy += normalized_sample.abs();
            }
            
            energy /= data.len() as f32; // Normalize energy by the number of samples

            if energy_queue.len() == rolling_window_size {
                energy_queue.pop_front(); // Remove the oldest energy value
            }
            energy_queue.push_back(energy);

            let avg_energy: f32 = energy_queue.iter().sum::<f32>() / energy_queue.len() as f32;
            let dynamic_threshold = avg_energy * peak_threshold_multiplier;

            let current_time = Instant::now();
            if energy > dynamic_threshold && current_time.duration_since(*last_beat_time) > cooldown_time {
                let mut beat_detected = globals::BEAT_DETECTED.lock().unwrap();
                *beat_detected = true;
                *last_beat_time = current_time;
            }
        },
        err_fn,
        None, // Latency hint
    )
}

