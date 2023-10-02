use crate::chord::Chord;
use crate::interval::Interval;
use crate::note::Note;
use crate::pitchclass::PitchClass;
use crate::scale::Scale;
use std::error::Error;
use std::fmt;

pub trait Synth {
    fn set_volume(&mut self, volume: f32);
    fn clear_voices(&mut self);
    fn add_voice(&mut self, frequency: f32);
    fn remove_voice(&mut self, frequency: f32);
    fn get_sample(&mut self) -> f32;
    fn advance_sample(&mut self, sample_rate: u32);
}

pub trait Playable {
    fn get_frequencies(&self) -> Vec<f32>;
}

impl Playable for f32 {
    fn get_frequencies(&self) -> Vec<f32> {
        vec![*self]
    }
}

impl Playable for Vec<f32> {
    fn get_frequencies(&self) -> Vec<f32> {
        self.clone()
    }
}

impl Playable for Note {
    fn get_frequencies(&self) -> Vec<f32> {
        vec![self.get_frequency()]
    }
}

impl Playable for Chord {
    fn get_frequencies(&self) -> Vec<f32> {
        let default_note = Note::default();
        let mut chord = self.clone();
        //If the chord is missing data, middle C is chosen as the tonic
        if chord.get_tonic().is_none() {
            chord.set_tonic(Some(default_note.get_pitch_class()));
        }
        if chord.get_octave().is_none() {
            chord.set_octave(Some(default_note.get_octave()));
        }
        let notes = Vec::<Note>::try_from(chord).unwrap();
        notes.iter().map(|note| note.get_frequency()).collect()
    }
}

impl Playable for Interval {
    fn get_frequencies(&self) -> Vec<f32> {
        let tonic = Note::default();
        let interval_note = tonic.at_offset(self.get_value() as isize);
        vec![tonic.get_frequency(), interval_note.get_frequency()]
    }
}

impl Playable for PitchClass {
    fn get_frequencies(&self) -> Vec<f32> {
        let default_note = Note::default();
        let note = Note::new(*self, default_note.get_octave());
        vec![note.get_frequency()]
    }
}

impl Playable for Scale {
    fn get_frequencies(&self) -> Vec<f32> {
        let default_note = Note::default();
        let notes = self.to_notes(default_note.get_pitch_class(), default_note.get_octave());
        notes.iter().map(|note| note.get_frequency()).collect()
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub enum ArpeggioDirection {
    #[default]
    Up,
    Down,
    UpDown,
    Random,
}

/// An error which is returned when audio could not be played.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct AudioPlayError {
    /// A more specific message that explains why specific audio could not be played.
    pub message: &'static str,
}

impl Error for AudioPlayError {}

impl fmt::Display for AudioPlayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "audio could not be played - {}", self.message)
    }
}

fn sine_wave(time: f32) -> f32 {
    f32::sin(2.0 * std::f32::consts::PI * time)
}

fn square_wave(time: f32) -> f32 {
    if time < 0.5 {
        -1.0
    } else {
        1.0
    }
}

fn triangle_wave(time: f32) -> f32 {
    if time < 0.5 {
        -4.0 * time + 1.0
    } else {
        4.0 * time - 3.0
    }
}

fn sawtooth_wave(time: f32) -> f32 {
    2.0 * time - 1.0
}

#[non_exhaustive]
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
