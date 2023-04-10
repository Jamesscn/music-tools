use crate::common::AudioPlayError;
use crate::midi::MIDI;
use crate::note::Note;
use crate::track::{Event, Track};
use rodio::{OutputStream, Sink, Source};
use std::cmp::min;
use std::time::Duration;

#[derive(Clone)]
struct Channel {
    wave_table: Vec<f32>,
    wave_table_size: usize,
    wave_function: fn(f32) -> f32,
    wave_function_time_scale: f32,
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
        new_channel
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

    pub fn set_wave_function(&mut self, wave_function: fn(f32) -> f32, time_scale: f32) {
        self.wave_function = wave_function;
        self.wave_function_time_scale = time_scale;
        self.generate_wave_table();
    }

    pub fn get_wave_table_value(&self, index: usize) -> f32 {
        self.wave_table[index]
    }

    pub fn get_wave_table_size(&self) -> usize {
        self.wave_table.len()
    }
}

#[derive(Copy, Clone)]
struct Voice {
    channel_index: usize,
    track_index: usize,
    frequency: f32,
    table_index: f32,
    sample_rate: u32,
}

impl Voice {
    pub fn new(
        track_index: usize,
        channel_index: usize,
        frequency: f32,
        sample_rate: u32,
    ) -> Voice {
        Voice {
            track_index,
            channel_index,
            frequency,
            table_index: 0.0,
            sample_rate,
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
        self.channel_index
    }

    pub fn get_track_index(&self) -> usize {
        self.track_index
    }

    pub fn get_table_index(&self) -> f32 {
        self.table_index
    }

    pub fn get_frequency(&self) -> f32 {
        self.frequency
    }
}

/// A structure which holds a wavetable oscillator.
///
/// A wavetable oscillator is used to store the shape of a wave in a table or an array which can
/// later be played at a specific frequency. There are several advantages to storing a wave this
/// way, most notably:
///
/// - Efficiency: It is more efficient to use a lookup table to store certain shapes of waves such
///   as sine waves than to call the sin() function.
/// - Timbre: It is easy to change the shape of the wave to something more complex such as a square,
///   sawtooth or triangle wave.
///
/// This implementation of a wavetable oscillator also allows you to play multiple frequencies of
/// the wave at the same time.
///
/// # Examples
///
/// ```rust
/// use music_tools::note::Note;
/// use music_tools::track::Track;
/// use music_tools::common::{Fraction, Beat};
/// use music_tools::audio::{WavetableOscillator, Waveforms};
///
/// let mut oscillator = WavetableOscillator::new();
/// let square_wave_channel = oscillator.add_channel(Waveforms::SQUARE_WAVE, 1.0);
/// let mut track = Track::new(120.0, Fraction::new(4, 4));
/// track.add_note(Note::from_string("A4").unwrap(), Beat::WHOLE);
/// oscillator.play_single_track(square_wave_channel, track);
/// ```
#[derive(Clone)]
pub struct WavetableOscillator {
    channels: Vec<Channel>,
    voices: Vec<Voice>,
    sample_rate: u32,
}

impl WavetableOscillator {
    /// Creates and returns a new wavetable oscillator which can be used as a [`rodio::Source`].
    pub fn new() -> WavetableOscillator {
        WavetableOscillator {
            channels: Vec::new(),
            voices: Vec::new(),
            sample_rate: 44100,
        }
    }

    /// Changes the sample rate of the wavetable oscillator.
    ///
    /// # Parameters
    ///
    /// - `sample_rate`: A positive integer representing the new sample rate of the oscillator in
    ///   hertz.
    pub fn set_sample_rate(&mut self, sample_rate: u32) {
        self.sample_rate = sample_rate;
        for voice in &mut self.voices {
            voice.set_sample_rate(sample_rate);
        }
    }

    /// Adds a new instrument channel to the wavetable oscillator, returning the index of this new
    /// channel.
    ///
    /// # Parameters
    ///
    /// - `wave_function`: The function used to generate the shape of the wave that will be played
    ///   by the new channel. It must recieve a parameter of type [`f32`] representing the time
    ///   value of the wave between 0 and `time_scale`, and it must return an [`f32`] representing
    ///   the height of the wave at that time between -1 and 1.
    /// - `time_scale`: This parameter scales the time variable that is passed to `wave_function`.
    pub fn add_channel(&mut self, wave_function: fn(f32) -> f32, time_scale: f32) -> usize {
        self.channels
            .push(Channel::new(128, wave_function, time_scale));
        self.channels.len() - 1
    }

    /// Changes the wave function used by an instrument channel.
    ///
    /// # Parameters
    ///
    /// - `channel_index`: The index of the channel to change.
    /// - `wave_function`: The function used to generate the shape of the wave that will be played
    ///   by the new channel. It must recieve a parameter of type [`f32`] representing the time
    ///   value of the wave between 0 and `time_scale`, and it must return an [`f32`] representing
    ///   the height of the wave at that time between -1 and 1.
    /// - `time_scale`: This parameter scales the time variable that is passed to `wave_function`.
    pub fn set_channel_wave_function(
        &mut self,
        channel_index: usize,
        wave_function: fn(f32) -> f32,
        time_scale: f32,
    ) {
        if channel_index >= self.channels.len() {
            return;
        }
        self.channels[channel_index].set_wave_function(wave_function, time_scale);
    }

    /// Plays a note on a given track on the instrument channel provided. This note will play
    /// indefinitely until it is stopped by another function.
    ///
    /// # Parameters
    ///
    /// - `track_index`: The index of the track this note belongs to. This value is used to keep
    ///   track of the note, as the same note might be played on multiple tracks.
    /// - `channel_index`: The index of the instrument channel the note will be played on.
    /// - `note`: The [`Note`] to be played.
    pub fn play_note(&mut self, track_index: usize, channel_index: usize, note: Note) -> bool {
        if channel_index >= self.channels.len() {
            return false;
        }
        let note_voice = Voice::new(
            track_index,
            channel_index,
            note.get_frequency(),
            self.sample_rate,
        );
        self.voices.push(note_voice);
        true
    }

    /// Stops playing a note if it was already playing. If the note was not being played then the
    /// function will do nothing.
    ///
    /// # Parameters
    ///
    /// - `track_index`: The index of the track the note was being played on.
    /// - `note`: The [`Note`] to be stopped.
    pub fn stop_note(&mut self, track_index: usize, note: Note) {
        for voice_index in (0..self.voices.len()).rev() {
            if self.voices[voice_index].get_frequency() == note.get_frequency()
                && self.voices[voice_index].get_track_index() == track_index
            {
                self.voices.remove(voice_index);
                return;
            }
        }
    }

    /// Stops playing all of the notes on a given track. If no notes were playing then the function
    /// will do nothing.
    ///
    /// # Parameters
    ///
    /// - `track_index`: The index of the track to stop all notes on.
    pub fn stop_all_notes(&mut self, track_index: usize) {
        for voice_index in (0..self.voices.len()).rev() {
            if self.voices[voice_index].get_track_index() == track_index {
                self.voices.remove(voice_index);
            }
        }
    }

    /// Plays a [`Track`] on a set of channels. If the number of channels is less than the number of
    /// tracks then the channels will be rotated across the tracks.
    ///
    /// # Parameters
    ///
    /// - `channel_indexes`: A vector of channel indexes that will be used to play each track.
    /// - `tracks`: A vector of all the [`Track`] objects to be played.
    pub fn play_tracks(
        &mut self,
        channel_indexes: Vec<usize>,
        tracks: Vec<Track>,
    ) -> Result<(), AudioPlayError> {
        if tracks.len() == 0 {
            return Err(AudioPlayError {
                message: "no tracks to play",
            });
        }
        let tick_ms = tracks[0].get_tick_duration();
        let stream_result = OutputStream::try_default();
        if stream_result.is_err() {
            return Err(AudioPlayError {
                message: "no sound card detected",
            });
        }
        let (_stream, stream_handle) = stream_result.unwrap();
        let sink_result = Sink::try_new(&stream_handle);
        if sink_result.is_err() {
            return Err(AudioPlayError {
                message: "sink could not be created",
            });
        }
        let sink = sink_result.unwrap();
        let mut mut_tracks = tracks.clone();
        let mut pending_event_tuples: Vec<(Event, u64, usize)> = Vec::new();
        for (track_index, track) in mut_tracks.iter_mut().enumerate() {
            let first_event_option = track.get_next_event();
            if let Some(first_event) = first_event_option {
                let event_tuple = (first_event, first_event.get_delta_ticks(), track_index);
                pending_event_tuples.push(event_tuple);
            }
        }
        loop {
            let mut next_event_tuples: Vec<(Event, u64, usize)> = Vec::new();
            let mut min_wait_ticks = u64::MAX;
            'track: for event_index in (0..pending_event_tuples.len()).rev() {
                let event_tuple = pending_event_tuples[event_index];
                let mut current_event = event_tuple.0;
                let mut wait_time = event_tuple.1;
                let track_index = event_tuple.2;
                let channel_index = channel_indexes[track_index % channel_indexes.len()];
                while wait_time == 0 {
                    if current_event.is_active() {
                        self.play_note(track_index, channel_index, current_event.get_note());
                    } else {
                        self.stop_note(track_index, current_event.get_note());
                    }
                    let next_event_option = mut_tracks[track_index].get_next_event();
                    if next_event_option.is_none() {
                        self.stop_all_notes(track_index);
                        continue 'track;
                    }
                    let next_event = next_event_option.unwrap();
                    wait_time = next_event.get_delta_ticks();
                    current_event = next_event;
                }
                min_wait_ticks = min(min_wait_ticks, wait_time);
                next_event_tuples.insert(0, (current_event, wait_time, track_index));
            }
            if next_event_tuples.is_empty() {
                break;
            }
            let tmp_oscillator = self.clone();
            sink.append(tmp_oscillator);
            sink.play();
            std::thread::sleep(Duration::from_millis(
                (tick_ms * (min_wait_ticks as f32)) as u64,
            ));
            sink.clear();
            for event in next_event_tuples.iter_mut() {
                *event = (event.0, event.1 - min_wait_ticks, event.2);
            }
            pending_event_tuples = next_event_tuples;
        }
        Ok(())
    }

    /// Plays a [`MIDI`] on a set of channels. If the number of channels is less than the number of
    /// MIDI tracks then the channels will be rotated across the tracks.
    ///
    /// # Parameters
    ///
    /// - `channel_indexes`: A vector of channel indexes that will be used to play each MIDI track.
    /// - `midi`: The [`MIDI`] to be played.
    pub fn play_midi(
        &mut self,
        channel_indexes: Vec<usize>,
        midi: MIDI,
    ) -> Result<(), AudioPlayError> {
        self.play_tracks(channel_indexes, midi.get_tracks())
    }
}

impl Default for WavetableOscillator {
    fn default() -> Self {
        Self::new()
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
        Some(sample)
    }
}

impl Source for WavetableOscillator {
    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

fn sine_wave(time: f32) -> f32 {
    f32::sin(2.0 * std::f32::consts::PI * time)
}

fn square_wave(time: f32) -> f32 {
    if time < 0.5 {
        return -1.0;
    }
    1.0
}

fn triangle_wave(time: f32) -> f32 {
    if time < 0.5 {
        return -4.0 * time + 1.0;
    }
    4.0 * time - 3.0
}

fn sawtooth_wave(time: f32) -> f32 {
    2.0 * time - 1.0
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
