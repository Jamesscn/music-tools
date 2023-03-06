use std::time::Duration;
use rodio::{Sink, Source, OutputStream};
use crate::note::Note;
use crate::track::Track;

#[derive(Copy, Clone, Debug)]
struct Channel {
    table_delta: f32,
    table_index: f32,
    note: Note
}

impl Channel {
    pub fn update_table_index(&mut self, table_size: usize) {
        self.table_index += self.table_delta;
        self.table_index %= table_size as f32;
    }

    pub fn get_table_index(&self) -> f32 {
        return self.table_index;
    }

    pub fn get_note_value(&self) -> i16 {
        return self.note.get_value();
    }
}

#[derive(Clone, Debug)]
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
/// use musictools::audio::WavetableOscillator;
/// 
/// let mut oscillator = WavetableOscillator::new(128, 44100);
/// oscillator.add_frequency(440.0);
/// oscillator.add_frequency(659.3);
/// let stream_result = OutputStream::try_default();
/// if stream_result.is_ok() {
///     let (_stream, stream_handle) = stream_result.unwrap();
///     let sink = Sink::try_new(&stream_handle).unwrap();
///     sink.append(oscillator);
///     std::thread::sleep(Duration::from_millis(1000));
/// } else {
///     println!("No sound card detected!");
/// }
/// ```
pub struct WavetableOscillator {
    wave_table: Vec<f32>,
    sample_rate: u32,
    channels: Vec<Channel>
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
        let wave_table = generate_wave_table(table_size, f32::sin, 2.0 * std::f32::consts::PI);
        return WavetableOscillator {
            wave_table,
            sample_rate,
            channels: Vec::new()
        };
    }

    pub fn add_channel(&mut self, note: Note) {
        for channel_index in 0..self.channels.len() {
            if self.channels[channel_index].get_note_value() == note.get_value() {
                return;
            }
        }
        self.channels.push(Channel {
            table_delta: note.get_frequency() * self.wave_table.len() as f32 / self.sample_rate as f32,
            table_index: 0.0,
            note
        });
    }

    pub fn remove_channel(&mut self, note: Note) {
        for channel_index in 0..self.channels.len() {
            if self.channels[channel_index].get_note_value() == note.get_value() {
                self.channels.remove(channel_index);
                return;
            }
        }
    }

    /// Uses the function provided to generate the wave table.
    /// 
    /// # Parameters
    /// 
    /// - `function`: The function used to generate the shape of the wave that
    /// will be played by the oscillator. It must recieve a parameter of type
    /// [`f32`] representing the time value of the wave between 0 and
    /// `time_scale`, and it must return an [`f32`] representing the height of
    /// the wave at that time between -1 and 1.
    /// - `time_scale`: This parameter scales the time variable that is passed
    /// to `function`.
    /// 
    /// # Examples
    /// 
    /// How to use a function from [`Waveforms`] to play triangle waves:
    /// 
    /// ```rust
    /// use musictools::audio::{WavetableOscillator, Waveforms};
    /// 
    /// let mut oscillator = WavetableOscillator::new(128, 44100);
    /// oscillator.set_wave_function(Waveforms::TRIANGLE_WAVE, 1.0);
    /// ```
    /// 
    /// How to create an oscillator for a sine wave:
    /// 
    /// ```rust
    /// use musictools::audio::WavetableOscillator;
    /// 
    /// let mut oscillator = WavetableOscillator::new(128, 44100);
    /// oscillator.set_wave_function(f32::sin, 2.0 * std::f32::consts::PI);
    /// ```
    /// 
    /// How to create an oscillator for a square wave:
    /// 
    /// ```rust
    /// use musictools::audio::WavetableOscillator;
    /// 
    /// fn square_wave(time: f32) -> f32 {
    ///     if time < 0.5 {
    ///         return 0.0;
    ///     }
    ///     return 1.0;
    /// }
    /// 
    /// let mut oscillator = WavetableOscillator::new(128, 44100);
    /// oscillator.set_wave_function(square_wave, 1.0);
    /// ```
    pub fn set_wave_function(&mut self, function: impl Fn(f32) -> f32, time_scale: f32) {
        self.wave_table = generate_wave_table(self.wave_table.len(), function, time_scale);
    }
}

fn generate_wave_table(table_size: usize, function: impl Fn(f32) -> f32, time_scale: f32) -> Vec<f32> {
    let mut wave_table: Vec<f32> = Vec::with_capacity(table_size);
    for i in 0..table_size {
        let time_value = i as f32 / table_size as f32;
        let wave_value = function(time_scale * time_value);
        wave_table.push(wave_value);
    }
    return wave_table;
}

impl Iterator for WavetableOscillator {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        let mut sample = 0.0;
        let table_size = self.wave_table.len();
        let num_channels = self.channels.len();
        for channel_index in 0..num_channels {
            let current_index = self.channels[channel_index].get_table_index() as usize;
            let next_index = (current_index + 1) % table_size;
            let lerp_frac = self.channels[channel_index].get_table_index() - current_index as f32;
            let current_value = self.wave_table[current_index];
            let next_value = self.wave_table[next_index];
            let lerp_value = current_value + lerp_frac * (next_value - current_value);
            sample += lerp_value / num_channels as f32;
            self.channels[channel_index].update_table_index(table_size);
        }
        return Some(sample);
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

fn sine_wave(time: f32) -> f32 {
    return f32::sin(2.0 * std::f32::consts::PI * time);
}

fn square_wave(time: f32) -> f32 {
    if time < 0.5 {
        return -1.0;
    }
    return 1.0;
}

fn triangle_wave(time: f32) -> f32 {
    if time < 0.5 {
        return -4.0 * time + 1.0;
    }
    return 4.0 * time - 3.0;
}

fn sawtooth_wave(time: f32) -> f32 {
    return 2.0 * time - 1.0;
}

/// A structure containing common waveforms.
pub struct Waveforms;

impl Waveforms {
    /// The sine wave function with a period of 1 unit of time.
    pub const SINE_WAVE: &dyn Fn(f32) -> f32 = &sine_wave;
    /// The square wave function with a period of 1 unit of time.
    pub const SQUARE_WAVE: &dyn Fn(f32) -> f32 = &square_wave;
    /// The triangle wave function with a period of 1 unit of time.
    pub const TRIANGLE_WAVE: &dyn Fn(f32) -> f32 = &triangle_wave;
    /// The sawtooth wave function with a period of 1 unit of time.
    pub const SAWTOOTH_WAVE: &dyn Fn(f32) -> f32 = &sawtooth_wave;
}

pub fn play(mut oscillator: WavetableOscillator, mut track: Track) {
    let tick_ms = track.get_tick_duration();
    let stream_result = OutputStream::try_default();
    if stream_result.is_err() {
        println!("No sound card detected!");
        return;
    }
    let (_stream, stream_handle) = stream_result.unwrap();
    let sink_result = Sink::try_new(&stream_handle);
    if sink_result.is_err() {
        println!("Could not create a sink!");
        return;
    }
    let sink = sink_result.unwrap();
    loop {
        let current_event_option = track.get_next_event();
        if current_event_option.is_none() {
            break;
        }
        let current_event = current_event_option.unwrap();
        let note = current_event.get_note();
        let delta_ticks = current_event.get_delta_ticks();
        if delta_ticks > 0 {
            let tmp_oscillator = oscillator.clone();
            sink.append(tmp_oscillator);
            sink.play();
            std::thread::sleep(Duration::from_millis((tick_ms * delta_ticks as f32) as u64));
            sink.clear();
        }
        if current_event.is_active() {
            oscillator.add_channel(note);
        } else {
            oscillator.remove_channel(note);
        }
    }
}