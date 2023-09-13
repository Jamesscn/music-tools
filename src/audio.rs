use crate::chord::Chord;
use crate::common::{AudioDuration, AudioPlayError, IncompleteChordError};
use crate::midi::MIDI;
use crate::note::Note;
use crate::track::Event;
use rodio::{OutputStream, Sink, Source};
use std::cmp::min;
use std::time::Duration;

const DEFAULT_WAVETABLE_SIZE: usize = 128;

/**
 * A structure used to play a specific wavetable at a specific frequency. These voices are meant
 * to play their frequency as long as they exist.
 */
#[derive(Copy, Clone)]
struct WavetableVoice {
    wavetable_index: usize,
    frequency: f32,
    table_index: f32,
    playing: bool,
}

impl WavetableVoice {
    pub fn new(wavetable_index: usize, frequency: f32) -> WavetableVoice {
        WavetableVoice {
            wavetable_index,
            frequency,
            table_index: 0.0,
            playing: false,
        }
    }

    pub fn start(&mut self) {
        self.table_index = 0.0;
        self.playing = true;
    }

    pub fn resume(&mut self) {
        self.playing = true;
    }

    pub fn stop(&mut self) {
        self.playing = false;
    }

    pub fn add_delta_time(&mut self, table_size: usize, sample_rate: u32) {
        let table_delta = self.frequency * table_size as f32 / sample_rate as f32;
        self.table_index += table_delta;
        self.table_index %= table_size as f32;
    }

    pub fn get_wavetable_index(&self) -> usize {
        self.wavetable_index
    }

    pub fn get_frequency(&self) -> f32 {
        self.frequency
    }

    pub fn get_table_index(&self) -> f32 {
        self.table_index
    }

    pub fn is_playing(&self) -> bool {
        self.playing
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
    wavetables: Vec<Vec<f32>>,
    voices: Vec<WavetableVoice>,
    sample_rate: u32,
}

impl WavetableOscillator {
    /// Creates and returns an empty wavetable oscillator which can be used as a [`rodio::Source`].
    pub fn empty() -> WavetableOscillator {
        WavetableOscillator {
            wavetables: Vec::new(),
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
    }

    /// Creates a new wavetable given a [`Vec<f32>`] containing the audio signal with values between
    /// -1 and 1.
    pub fn add_wavetable_from_vec(&mut self, wavetable: Vec<f32>) -> usize {
        let mut clamped_wavetable = Vec::with_capacity(wavetable.len());
        for value in wavetable {
            clamped_wavetable.push(value.clamp(-1.0, 1.0));
        }
        self.wavetables.push(clamped_wavetable);
        self.wavetables.len() - 1
    }

    /// Creates a new wavetable given a function of the height of the audio signal between -1 and 1
    /// with respect to time, returning the index of this new wavetable.
    ///
    /// # Parameters
    ///
    /// - `wave_function`: The function used to generate the shape of the wave that will be played
    ///   by the new wavetable. It must recieve a parameter of type [`f32`] representing the time
    ///   value of the wave between 0 and `max_time`, and it must return an [`f32`] representing the
    ///   height of the wave at that time between -1 and 1.
    /// - `max_time`: This parameter scales the time variable that is passed to `wave_function`.
    /// - `wavetable_size`: The amount of points to store in the wavetable. The higher this value,
    /// the higher the quality of the signal.
    pub fn add_wavetable_from_function(
        &mut self,
        wave_function: fn(f32) -> f32,
        max_time: f32,
        wavetable_size: usize,
    ) -> usize {
        let mut wavetable = Vec::with_capacity(wavetable_size);
        for i in 0..wavetable_size {
            let time_value = i as f32 / wavetable_size as f32;
            let wave_value = wave_function(max_time * time_value).clamp(-1.0, 1.0);
            wavetable.push(wave_value);
        }
        self.wavetables.push(wavetable);
        self.wavetables.len() - 1
    }

    pub fn add_voice(&mut self, wavetable_index: usize, frequency: f32) -> usize {
        if wavetable_index < self.wavetables.len() {
            let note_voice = WavetableVoice::new(wavetable_index, frequency);
            self.voices.push(note_voice);
            self.voices.len() - 1
        } else {
            usize::MAX
        }
    }

    pub fn start_voice(&mut self, voice_index: usize) {
        if voice_index < self.voices.len() {
            self.voices[voice_index].start();
        }
    }

    pub fn resume_voice(&mut self, voice_index: usize) {
        if voice_index < self.voices.len() {
            self.voices[voice_index].resume();
        }
    }

    pub fn stop_voice(&mut self, voice_index: usize) {
        if voice_index < self.voices.len() {
            self.voices[voice_index].stop();
        }
    }

    pub fn stop_frequency(&mut self, frequency: f32) {
        for voice in &mut self.voices {
            if voice.get_frequency() == frequency {
                voice.stop();
            }
        }
    }

    pub fn clear_voices(&mut self) {
        self.voices.clear();
    }
}

impl Default for WavetableOscillator {
    fn default() -> Self {
        Self::empty()
    }
}

impl Iterator for WavetableOscillator {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        let mut sample = 0.0;
        for voice in &mut self.voices {
            if !voice.is_playing() {
                continue;
            }
            let wavetable_index = voice.get_wavetable_index();
            let table_size = self.wavetables[wavetable_index].len();
            let current_index = voice.get_table_index() as usize;
            let next_index = (current_index + 1) % table_size;
            let lerp_frac = voice.get_table_index() - current_index as f32;
            let current_value = self.wavetables[wavetable_index][current_index];
            let next_value = self.wavetables[wavetable_index][next_index];
            let lerp_value = current_value + lerp_frac * (next_value - current_value);
            sample += lerp_value * 0.2;
            voice.add_delta_time(table_size, self.sample_rate);
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

pub struct AudioPlayer {
    tempo: Option<f32>,
    sink: Sink,
    _stream: OutputStream,
}

impl AudioPlayer {
    pub fn new() -> Result<AudioPlayer, AudioPlayError> {
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
        Ok(AudioPlayer {
            tempo: None,
            sink: sink_result.unwrap(),
            _stream,
        })
    }

    pub fn set_tempo(&mut self, tempo: f32) {
        self.tempo = Some(tempo);
    }

    pub fn unset_tempo(&mut self) {
        self.tempo = None;
    }

    pub fn play_note(&self, note: Note, duration: impl AudioDuration) {
        let mut oscillator = WavetableOscillator::empty();
        let wavetable_index = oscillator.add_wavetable_from_function(
            Waveforms::SINE_WAVE,
            1.0,
            DEFAULT_WAVETABLE_SIZE,
        );
        let voice_index = oscillator.add_voice(wavetable_index, note.get_frequency());
        oscillator.start_voice(voice_index);
        self.sink.append(oscillator);
        self.sink.play();
        std::thread::sleep(duration.get_duration(self.tempo.unwrap_or(120.0)));
        self.sink.clear();
    }

    pub fn play_chord(
        &self,
        chord: Chord,
        duration: impl AudioDuration,
    ) -> Result<(), IncompleteChordError> {
        let notes = chord.to_notes()?;
        let mut oscillator = WavetableOscillator::empty();
        let wavetable_index = oscillator.add_wavetable_from_function(
            Waveforms::SINE_WAVE,
            1.0,
            DEFAULT_WAVETABLE_SIZE,
        );
        for note in notes {
            let voice_index = oscillator.add_voice(wavetable_index, note.get_frequency());
            oscillator.start_voice(voice_index);
        }
        self.sink.append(oscillator);
        self.sink.play();
        std::thread::sleep(duration.get_duration(self.tempo.unwrap_or(120.0)));
        self.sink.clear();
        Ok(())
    }

    pub fn play_frequencies(&self, frequencies: Vec<f32>, duration: impl AudioDuration) {
        let mut oscillator = WavetableOscillator::empty();
        let wavetable_index = oscillator.add_wavetable_from_function(
            Waveforms::SINE_WAVE,
            1.0,
            DEFAULT_WAVETABLE_SIZE,
        );
        for frequency in frequencies {
            let voice_index = oscillator.add_voice(wavetable_index, frequency);
            oscillator.start_voice(voice_index);
        }
        self.sink.append(oscillator);
        self.sink.play();
        std::thread::sleep(duration.get_duration(self.tempo.unwrap_or(120.0)));
        self.sink.clear();
    }

    pub fn play_midi(&self, midi: MIDI) {
        let tick_ms = midi.get_tracks()[0].get_tick_duration();
        let mut tracks = midi.get_tracks();
        let mut oscillator = WavetableOscillator::empty();
        let wavetable_index = oscillator.add_wavetable_from_function(
            Waveforms::SINE_WAVE,
            1.0,
            DEFAULT_WAVETABLE_SIZE,
        );
        let mut pending_event_tuples: Vec<(Event, u64, usize)> = Vec::new();
        for (track_index, track) in &mut tracks.iter_mut().enumerate() {
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
                while wait_time == 0 {
                    if current_event.is_active() {
                        let voice_index = oscillator
                            .add_voice(wavetable_index, current_event.get_note().get_frequency());
                        oscillator.start_voice(voice_index)
                    } else {
                        oscillator.stop_frequency(current_event.get_note().get_frequency());
                    }
                    let next_event_option = &mut tracks[track_index].get_next_event();
                    if next_event_option.is_none() {
                        oscillator.clear_voices();
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
            self.sink.append(oscillator.clone());
            self.sink.play();
            std::thread::sleep(Duration::from_millis(
                (tick_ms * (min_wait_ticks as f32)) as u64,
            ));
            self.sink.clear();
            for event in &mut next_event_tuples {
                *event = (event.0, event.1 - min_wait_ticks, event.2);
            }
            pending_event_tuples = next_event_tuples;
        }
        for mut track in tracks {
            track.reset_tracker();
        }
    }

    pub fn rest(&self, duration: impl AudioDuration) {
        std::thread::sleep(duration.get_duration(self.tempo.unwrap_or(120.0)));
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
