use rand::Rng;
use std::f32::consts::PI;

use core::time::Duration;
use rodio::{source::Source, OutputStream};

struct WavetableOscillator {
    sample_rate: u32,
    wave_table: Vec<f32>,
    index: f32,
    index_increment: f32,
}

impl WavetableOscillator {
    fn new(sample_rate: u32, wave_table: Vec<f32>) -> WavetableOscillator {
        return WavetableOscillator {
            sample_rate,
            wave_table,
            index: 0.0,
            index_increment: 0.0,
        };
    }

    fn set_frequency(&mut self, frequency: f32) {
        self.index_increment = frequency * self.wave_table.len() as f32 / self.sample_rate as f32;
    }

    fn get_sample(&mut self) -> f32 {
        let sample = self.lerp();
        self.index += self.index_increment;
        self.index %= self.wave_table.len() as f32;
        return sample;
    }

    fn lerp(&self) -> f32 {
        let truncated_index = self.index as usize;
        let next_index = (truncated_index + 1) % self.wave_table.len();

        let next_index_weight = self.index - truncated_index as f32;
        let truncated_index_weight = 1.0 - next_index_weight;

        return truncated_index_weight * self.wave_table[truncated_index]
            + next_index_weight * self.wave_table[next_index];
    }
}

impl Iterator for WavetableOscillator {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        return Some(self.get_sample());
    }
}

impl Source for WavetableOscillator {
    fn channels(&self) -> u16 {
        return 1;
    }

    fn sample_rate(&self) -> u32 {
        return self.sample_rate;
    }

    fn current_frame_len(&self) -> Option<usize> {
        return None;
    }

    fn total_duration(&self) -> Option<Duration> {
        return None;
    }
}

fn main() {
    let wave_table_size = 64;
    let mut sine_wave_table: Vec<f32> = Vec::with_capacity(wave_table_size);
    let mut cos_wave_table: Vec<f32> = Vec::with_capacity(wave_table_size);
    let mut square_wave_table: Vec<f32> = Vec::with_capacity(wave_table_size);
    let mut triangle_wave_table: Vec<f32> = Vec::with_capacity(wave_table_size);
    let mut sawtooth_wave_table: Vec<f32> = Vec::with_capacity(wave_table_size);
    let mut noise_wave_table: Vec<f32> = Vec::with_capacity(wave_table_size);

    for n in 0..wave_table_size {
        // Insert values for wave tables
        let percentage = n as f32 / wave_table_size as f32;
        sine_wave_table.push((2.0 * PI * n as f32 / wave_table_size as f32).sin());
        cos_wave_table.push((2.0 * PI * n as f32 / wave_table_size as f32).cos());
        noise_wave_table.push(rand::thread_rng().gen_range(-1.0..1.0));
        sawtooth_wave_table.push(2.0 * percentage - 1.0);

        if n < wave_table_size / 2 {
            square_wave_table.push(-1.0);
            triangle_wave_table.push(2.0 * (percentage * 2.0) - 1.0);
        } else {
            square_wave_table.push(1.0);
            triangle_wave_table.push(1.0 - (2.0 * (percentage * 2.0) - 1.0));
        }
    }

    let mut wave_table: Vec<f32> = Vec::new();
    // wave_table.append(&mut noise_wave_table);
    wave_table.append(&mut square_wave_table);
    // wave_table.append(&mut sine_wave_table);
    // wave_table.append(&mut cos_wave_table);
    // wave_table.append(&mut triangle_wave_table);
    // wave_table.append(&mut sawtooth_wave_table);

    // TODO: make CLI to switch wave tables

    let mut oscillator = WavetableOscillator::new(44_100, wave_table);
    oscillator.set_frequency(440.0);

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let _result = stream_handle.play_raw(oscillator.convert_samples());
    std::thread::sleep(std::time::Duration::from_secs(2));
}
