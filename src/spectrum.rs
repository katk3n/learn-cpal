use spectrum_analyzer::scaling::*;
use spectrum_analyzer::windows::hann_window;
use spectrum_analyzer::{samples_fft_to_spectrum, FrequencyLimit};

const SAMPLING_RATE: u32 = 48000;

pub struct Spectrum {
    pub frequencies: Vec<(f32, f32)>,
    pub max_frequency: f32,
    pub max_amplitude: f32,
}

impl Default for Spectrum {
    fn default() -> Self {
        Self {
            frequencies: vec![],
            max_frequency: 0.0,
            max_amplitude: 0.0,
        }
    }
}

impl Spectrum {
    pub fn analyze(samples: &[f32], min_frequency: f32, max_frequency: f32) -> Self {
        let hann_window = hann_window(samples);
        let spectrum = samples_fft_to_spectrum(
            &hann_window,
            SAMPLING_RATE,
            FrequencyLimit::Range(min_frequency, max_frequency),
            Some(&divide_by_N_sqrt),
        )
        .unwrap();

        let frequencies = spectrum
            .data()
            .iter()
            .map(|freq| (freq.0.val(), freq.1.val()))
            .collect();

        let (max_fr, max_amp) = spectrum.max();

        Self {
            frequencies,
            max_frequency: max_fr.val(),
            max_amplitude: max_amp.val(),
        }
    }
}
