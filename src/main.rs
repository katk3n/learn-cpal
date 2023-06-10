use crate::spectrum::Spectrum;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use ringbuf::{Consumer, Producer, RingBuffer};

pub mod spectrum;

const NUM_SAMPLES: usize = 1024;

fn main() -> anyhow::Result<()> {
    let host = cpal::default_host();
    let input_device = host
        .default_input_device()
        .expect("failed to find input device");
    let mut supported_config_range = input_device
        .supported_input_configs()
        .expect("error while querying configs");
    let supported_config = supported_config_range
        .next()
        .expect("no supported config?!")
        .with_max_sample_rate();
    let config = supported_config.into();

    let ring_buffer = RingBuffer::<f32>::new(NUM_SAMPLES * 2);
    let (mut prod, mut cons) = ring_buffer.split();
    for _ in 0..NUM_SAMPLES {
        prod.push(0.0).unwrap();
    }

    let stream = input_device
        .build_input_stream(
            &config,
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                for d in data {
                    prod.push(*d).unwrap();
                }
            },
            move |_err| {},
            None,
        )
        .unwrap();

    stream.play().unwrap();

    loop {
        if cons.len() < NUM_SAMPLES {
            continue;
        }

        let mut samples: [f32; NUM_SAMPLES] = [0.0; NUM_SAMPLES];
        for i in 0..NUM_SAMPLES {
            let sample = cons.pop().unwrap();
            samples[i] = sample;
        }
        let spectrum = Spectrum::analyze(&samples, 27.0, 2000.0);
        println!(
            "freq: {}, amp: {}",
            spectrum.max_frequency, spectrum.max_amplitude
        );
    }

    Ok(())
}
