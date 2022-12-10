/// Code followed from WolfSound simple wavetable synthesis in Rust article:
/// https://thewolfsound.com/sound-synthesis/wavetable-synth-in-rust/
use rand::Rng;
use std::f32::consts::PI;

use core::time::Duration;
use rodio::{source::Source, OutputStream, Sink};

/// A `WavetableOscillator` iterates over a given wave table at a speed dictated
/// by the frequency of the tone it should output, and the given sample rate.
struct WavetableOscillator {
    sample_rate: u32,
    wave_table: Vec<f32>,
    index: f32,
    index_increment: f32,
    amplitude: f32,
}

impl WavetableOscillator {
    fn new(sample_rate: u32, wave_table: Vec<f32>) -> WavetableOscillator {
        return WavetableOscillator {
            sample_rate,
            wave_table,
            index: 0.0,
            index_increment: 0.0,
            amplitude: 0.8,
        };
    }

    /// Index increment is calculated as follows:
    /// ```
    ///   f * L / SR
    /// ```
    /// where `f` = frequency of osc, `L` is length of wavetable, and `SR` is sample rate
    fn set_frequency(&mut self, frequency: f32) {
        self.index_increment = frequency * self.wave_table.len() as f32 / self.sample_rate as f32;
    }

    /// Sets amplitude of oscillator.
    ///
    /// `amplitude` should be a unipolar value between [0.0, 1.0]
    fn set_amplitude(&mut self, amplitude: f32) {
        self.amplitude = amplitude;
    }

    /// Generates a sample and increments the lookup index in the wave table.
    ///
    /// Sample generation consists of linear interpolation of the wave table values
    /// according to the `index` value and incrementing the index. Then, we
    /// perform `fmod`, which is simply the `%` operation in Rust.
    fn get_sample(&mut self) -> f32 {
        let sample = self.lerp();
        self.index += self.index_increment;
        self.index %= self.wave_table.len() as f32;
        return sample;
    }

    /// Returns a linearly interpolated sample (scaled for amplitude) between
    /// the current index and the next index determined by the index increment.
    fn lerp(&self) -> f32 {
        let truncated_index = self.index as usize;
        let next_index = (truncated_index + 1) % self.wave_table.len();

        let next_index_weight = self.index - truncated_index as f32;
        let truncated_index_weight = 1.0 - next_index_weight;

        let interpolated_sample = truncated_index_weight * self.wave_table[truncated_index]
            + next_index_weight * self.wave_table[next_index];

        interpolated_sample * self.amplitude
    }
}

impl Iterator for WavetableOscillator {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        return Some(self.get_sample());
    }
}

/// The `Source` trait declares methods that tells `rodio` what characteristics our
/// output waveform has. In this case, we have a mono channel output at the given
/// sample rate that can last for an uncertain amount of time (hence `None` returned
/// for `total_duration()`).
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

/// Returns a bipolar value for a sawtooth wave given an input in **radians**.
fn sawtooth(x: f32) -> f32 {
    (x + PI) / PI % 2.0 - 1.0
}

fn main() {
    // Declare wave tables
    // A wavetable is an array in memory that contains 1 period of the waveform we
    // want to play out of our oscillator.
    let wave_table_size = 64;

    // We're using the `with_capacity` constructor to specify how many elements
    // should be possible to fit in our vector without reallocation of memory.
    let mut sine_wave_table: Vec<f32> = Vec::with_capacity(wave_table_size);
    let mut cos_wave_table: Vec<f32> = Vec::with_capacity(wave_table_size);
    let mut square_wave_table: Vec<f32> = Vec::with_capacity(wave_table_size);
    let mut triangle_wave_table: Vec<f32> = Vec::with_capacity(wave_table_size);
    let mut sawtooth_wave_table: Vec<f32> = Vec::with_capacity(wave_table_size);
    let mut noise_wave_table: Vec<f32> = Vec::with_capacity(wave_table_size);

    // Insert values for wave tables
    for n in 0..wave_table_size {
        let percentage = n as f32 / wave_table_size as f32;

        // Sine wave table math - generating a sine wave using built in `sin` and radian input
        sine_wave_table.push((2.0 * PI * n as f32 / wave_table_size as f32).sin());

        // Cosine wave table is just the sine wave table, phase offset by 1/4th a period.
        // Since cosine waves start at an amplitude of 1, you'll hear a click when the sound starts.
        // Probably not best to use this.. 😅
        cos_wave_table.push((2.0 * PI * n as f32 / wave_table_size as f32).cos());

        // I thought this would produce noise, but since this is a wave table with (probably) not that
        // many samples, it actually produces a periodic waveform and, subsequently, an audible pitch
        // with an interesting spectra.
        noise_wave_table.push(rand::thread_rng().gen_range(-1.0..1.0));

        // I found two ways to make a sawtooth wave table.
        // The first is similar in syntax to sin() and cos() and takes in a radian input.
        // The second just constructs the sawtooth wave using the wave table position as a percentage.

        // sawtooth_wave_table.push(sawtooth(2.0 * PI * n as f32 / wave_table_size as f32));
        sawtooth_wave_table.push(2.0 * percentage - 1.0);

        // Generates a pulse wave with duty cycle 50% (although this could be easily changed in code)
        square_wave_table.push(match n < wave_table_size / 2 {
            true => -1.0,
            false => 1.0,
        });

        // Generates a triangle wave with math similar to the second sawtooth wave example
        triangle_wave_table.push(match n < wave_table_size / 2 {
            true => 2.0 * (percentage * 2.0) - 1.0,
            false => 1.0 - (2.0 * (percentage * 2.0) - 1.0),
        });
    }

    // Create wave table for oscillator; append as desired
    let mut wave_table: Vec<f32> = Vec::new();
    wave_table.append(&mut sine_wave_table);
    // wave_table.append(&mut noise_wave_table);
    // wave_table.append(&mut square_wave_table);
    // wave_table.append(&mut cos_wave_table);
    // wave_table.append(&mut triangle_wave_table);
    // wave_table.append(&mut sawtooth_wave_table);

    // Initialize wavetable
    let mut oscillator = WavetableOscillator::new(44_100, wave_table);
    oscillator.set_frequency(440.0);
    oscillator.set_amplitude(0.1);

    // Play audio over wavetable
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    sink.append(oscillator.fade_in(Duration::from_secs(1)));
    sink.sleep_until_end();

    // let _result = stream_handle.play_raw(oscillator.convert_samples());
    // std::thread::sleep(std::time::Duration::from_secs(2));
}
