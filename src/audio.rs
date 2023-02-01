pub use std::time::Duration;
pub use rodio::Source;

/// A structure which holds a wavetable oscillator.
/// 
/// A wavetable oscillator is used to store the shape of a wave in a table or
/// an array which can later be played at a specific frequency. There are
/// several advantages to storing a wave this way, most notably:
/// 
/// - Efficiency: It is more efficient to use a lookup table to store certain
/// shapes of waves such as sine waves than to call the sin() function.
/// - Timbre: It is easy to change the shape of the wave to something more
/// complex such as a square, sawtooth or triangle wave.
/// 
/// This implementation of a wavetable oscillator also allows you to play
/// multiple frequencies of the wave at the same time.
/// 
/// # Examples
/// 
/// ```rust
/// use std::time::Duration;
/// use rodio::{OutputStream, OutputStreamHandle, Sink};
/// use musictools::audio::{WavetableOscillator, Source};
/// 
/// let mut oscillator = WavetableOscillator::new(128, 44100);
/// oscillator.add_frequency(440.0);
/// oscillator.add_frequency(659.3);
/// let (_stream, stream_handle) = OutputStream::try_default().unwrap();
/// let sink = Sink::try_new(&stream_handle).unwrap();
/// sink.append(oscillator);
/// std::thread::sleep(Duration::from_millis(1000));
/// ```
pub struct WavetableOscillator {
    wave_table: Vec<f32>,
    table_size: usize,
    sample_rate: u32,
    table_deltas: Vec<f32>,
    table_indexes: Vec<f32>
}

impl WavetableOscillator {
    /// Creates and returns a new wavetable oscillator which can be used as a
    /// [`rodio::Source`].
    /// 
    /// # Parameters
    /// 
    /// - `table_size`: Determines the size of the array that holds the wave,
    /// and ultimately the resolution of the waveform.
    /// - `sample_rate`: The sample rate in hertz that will be used while
    /// reproducing the waveform through the user's speakers. This is
    /// typically 44100 Hz.
    pub fn new(table_size: usize, sample_rate: u32) -> WavetableOscillator {
        let wave_table = generate_wave_table(table_size);
        return WavetableOscillator {
            wave_table,
            table_size,
            sample_rate,
            table_deltas: Vec::new(),
            table_indexes: Vec::new()
        };
    }

    /// Adds a new frequency to the list of frequencies that will be used to
    /// play the waveform.
    /// 
    /// # Parameters
    /// 
    /// - `frequency`: The frequency in hertz that will be added to the list.
    pub fn add_frequency(&mut self, frequency: f32) {
        let table_delta = frequency * self.table_size as f32 / self.sample_rate as f32;
        self.table_deltas.push(table_delta);
        self.table_indexes.push(0.0);
    }

    /// Clears the list of frequencies that will be used to play the waveform.
    pub fn clear_frequencies(&mut self) {
        self.table_deltas.clear();
        self.table_indexes.clear();
    }

    fn get_next_sample(&mut self) -> f32 {
        let mut sample = 0.0;
        for index in 0..self.table_deltas.len() {
            let current_index = self.table_indexes[index] as usize;
            let next_index = (current_index + 1) % self.table_size;
            let lerp_frac = self.table_indexes[index] - current_index as f32;
            let current_value = self.wave_table[current_index];
            let next_value = self.wave_table[next_index];
            let lerp_value = current_value + lerp_frac * (next_value - current_value);
            sample += lerp_value / self.table_deltas.len() as f32;
            self.table_indexes[index] += self.table_deltas[index];
            self.table_indexes[index] %= self.table_size as f32;
        }
        return sample;
    }
}

fn generate_wave_table(table_size: usize) -> Vec<f32> {
    let mut wave_table: Vec<f32> = Vec::with_capacity(table_size);
    for i in 0..table_size {
        let time_value = 2.0 * std::f32::consts::PI * i as f32 / table_size as f32;
        let wave_value = time_value.sin();
        wave_table.push(wave_value);
    }
    return wave_table;
}

impl Iterator for WavetableOscillator {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        return Some(self.get_next_sample());
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