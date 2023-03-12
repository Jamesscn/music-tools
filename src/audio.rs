use std::time::Duration;
use rodio::{Sink, Source, OutputStream};
use crate::note::Note;
use crate::track::Track;

#[derive(Clone)]
struct Channel {
    wave_table: Vec<f32>,
    wave_table_size: usize,
    wave_function: fn(f32) -> f32,
    wave_function_time_scale: f32
}

impl Channel {
    pub fn new(table_size: usize, wave_function: fn(f32) -> f32, time_scale: f32) -> Channel {
        let mut new_channel = Channel {
            wave_table: Vec::new(),
            wave_table_size: table_size,
            wave_function,
            wave_function_time_scale: time_scale,
        };
        new_channel.generate_wave_table();
        return new_channel;
    }

    pub fn generate_wave_table(&mut self) {
        self.wave_table = Vec::with_capacity(self.wave_table_size);
        for i in 0..self.wave_table_size {
            let time_value = i as f32 / self.wave_table_size as f32;
            let wave_function = self.wave_function;
            let wave_value = wave_function(self.wave_function_time_scale * time_value);
            self.wave_table.push(wave_value);
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
    pub fn set_wave_function(&mut self, wave_function: fn(f32) -> f32, time_scale: f32) {
        self.wave_function = wave_function;
        self.wave_function_time_scale = time_scale;
        self.generate_wave_table();
    }

    pub fn get_wave_table_value(&self, index: usize) -> f32 {
        return self.wave_table[index];
    }

    pub fn get_wave_table_size(&self) -> usize {
        return self.wave_table.len();
    }
}

#[derive(Copy, Clone)]
struct Voice {
    channel_index: usize,
    frequency: f32,
    table_index: f32,
    sample_rate: u32
}

impl Voice {
    pub fn new(channel_index: usize, frequency: f32, sample_rate: u32) -> Voice {
        return Voice {
            channel_index,
            frequency,
            table_index: 0.0,
            sample_rate
        }
    }

    pub fn update_table_index(&mut self, table_size: usize) {
        let table_delta = self.frequency * table_size as f32 / self.sample_rate as f32;
        self.table_index += table_delta;
        self.table_index %= table_size as f32;
    }

    pub fn set_sample_rate(&mut self, sample_rate: u32) {
        self.sample_rate = sample_rate;
    }

    pub fn get_channel_index(&self) -> usize {
        return self.channel_index;
    }
    
    pub fn get_table_index(&self) -> f32 {
        return self.table_index;
    }

    pub fn get_frequency(&self) -> f32 {
        return self.frequency;
    }
}

#[derive(Clone)]
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
    channels: Vec<Channel>,
    voices: Vec<Voice>,
    sample_rate: u32
}

impl WavetableOscillator {
    /// Creates and returns a new wavetable oscillator which can be used as a
    /// [`rodio::Source`].
    pub fn new() -> WavetableOscillator {
        return WavetableOscillator {
            channels: Vec::new(),
            voices: Vec::new(),
            sample_rate: 44100
        }
    }

    pub fn set_sample_rate(&mut self, sample_rate: u32) {
        self.sample_rate = sample_rate;
        for voice in &mut self.voices {
            voice.set_sample_rate(sample_rate);
        }
    }

    pub fn add_channel(&mut self, wave_function: fn(f32) -> f32, time_scale: f32) -> usize {
        self.channels.push(Channel::new(128, wave_function, time_scale));
        return self.channels.len() - 1;
    }

    pub fn set_channel_wave_function(&mut self, channel_index: usize, wave_function: fn(f32) -> f32, time_scale: f32) {
        if channel_index >= self.channels.len() {
            return;
        }
        self.channels[channel_index].set_wave_function(wave_function, time_scale);
    }

    pub fn play_note(&mut self, channel_index: usize, note: Note) -> bool {
        if channel_index >= self.channels.len() {
            return false;
        }
        let note_voice = Voice::new(channel_index, note.get_frequency(), self.sample_rate);
        self.voices.push(note_voice);
        return true;
    }

    pub fn stop_note(&mut self, note: Note) {
        for voice_index in 0..self.voices.len() {
            if self.voices[voice_index].get_frequency() == note.get_frequency() {
                self.voices.remove(voice_index);
                return;
            }
        }
    }

    pub fn play_track(&mut self, channel_index: usize, mut track: Track) {
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
                let tmp_oscillator = self.clone();
                sink.append(tmp_oscillator);
                sink.play();
                std::thread::sleep(Duration::from_millis((tick_ms * delta_ticks as f32) as u64));
                sink.clear();
            }
            if current_event.is_active() {
                self.play_note(channel_index, note);
            } else {
                self.stop_note(note);
            }
        }
    }
}

impl Iterator for WavetableOscillator {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        let mut sample = 0.0;
        for voice_index in 0..self.voices.len() {
            let voice = &mut self.voices[voice_index];
            let channel = &self.channels[voice.get_channel_index()];
            let table_size = channel.get_wave_table_size();
            let current_index = voice.get_table_index() as usize;
            let next_index = (current_index + 1) % table_size;
            let lerp_frac = voice.get_table_index() - current_index as f32;
            let current_value = channel.get_wave_table_value(current_index);
            let next_value = channel.get_wave_table_value(next_index);
            let lerp_value = current_value + lerp_frac * (next_value - current_value);
            sample += lerp_value * 0.2;
            voice.update_table_index(table_size);
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
    pub const SINE_WAVE: fn(f32) -> f32 = sine_wave;
    /// The square wave function with a period of 1 unit of time.
    pub const SQUARE_WAVE: fn(f32) -> f32 = square_wave;
    /// The triangle wave function with a period of 1 unit of time.
    pub const TRIANGLE_WAVE: fn(f32) -> f32 = triangle_wave;
    /// The sawtooth wave function with a period of 1 unit of time.
    pub const SAWTOOTH_WAVE: fn(f32) -> f32 = sawtooth_wave;
}