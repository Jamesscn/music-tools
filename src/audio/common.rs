use crate::chord::{Chord, GenericChord, NoteChord};
use crate::common::Tuning;
use crate::interval::Interval;
use crate::note::Note;
use crate::pitchclass::{PitchClass, TwelveTone};
use crate::scale::Scale;
use std::convert::Infallible;
use std::error::Error;
use std::fmt;
use std::str::FromStr;

/// A trait representing any type that can be used as a synthesizer for the audio processor.
pub trait Synth {
    /// Sets the volume of the current synthesizer.
    ///
    /// # Parameters
    ///
    /// - `volume`: The new volume of the synthesizer, which must be a value between 0.0 and 1.0.
    fn set_volume(&mut self, volume: f32);

    /// Clears all the voices or frequencies that are being played on the synthesizer.
    fn clear_voices(&mut self);

    /// Adds a voice which will play a frequency on the synthesizer.
    ///
    /// # Parameters
    ///
    /// - `frequency`: The frequency in hertz of the voice to be played.
    fn add_voice(&mut self, frequency: f32);

    /// Stops or removes a voice which is being played on the synthesizer.
    ///
    /// # Parameters
    ///
    /// - `frequency`: The frequency in hertz of the voice that will stop being played.
    fn remove_voice(&mut self, frequency: f32);

    /// Returns the current sample that is being produced by the synthesizer as an [`f32`].
    fn get_sample(&mut self) -> f32;

    /// Advances the synthesizer to play the next sample.
    ///
    /// # Parameters
    ///
    /// - `sample_rate`: The sample rate in hertz to be taken into account while advancing to the
    ///   next sample.
    fn advance_sample(&mut self, sample_rate: u32);
}

/// Represents any structure that can be broken down into a set of frequencies and processed by the
/// audio processor into an output signal.
pub trait Playable<PitchClassType: PitchClass> {
    /// Returns a list of frequencies in hertz for each of the individual audio elements contained
    /// by the structure.
    fn get_frequencies(&self, tuning: &dyn Tuning<PitchClassType>, base_frequency: f32)
        -> Vec<f32>;
}

impl<PitchClassType: PitchClass> Playable<PitchClassType> for f32 {
    fn get_frequencies(
        &self,
        _tuning: &dyn Tuning<PitchClassType>,
        _base_frequency: f32,
    ) -> Vec<f32> {
        vec![*self]
    }
}

impl<PitchClassType: PitchClass> Playable<PitchClassType> for &[f32] {
    fn get_frequencies(
        &self,
        _tuning: &dyn Tuning<PitchClassType>,
        _base_frequency: f32,
    ) -> Vec<f32> {
        self.to_vec()
    }
}

impl Playable<TwelveTone> for Note<TwelveTone> {
    fn get_frequencies(&self, tuning: &dyn Tuning<TwelveTone>, base_frequency: f32) -> Vec<f32> {
        vec![tuning.get_frequency(base_frequency, Note::from_string("A4").unwrap(), *self)]
    }
}

impl Playable<TwelveTone> for &[Note<TwelveTone>] {
    fn get_frequencies(&self, tuning: &dyn Tuning<TwelveTone>, base_frequency: f32) -> Vec<f32> {
        self.iter()
            .map(|note| note.get_frequencies(tuning, base_frequency)[0])
            .collect()
    }
}

impl Playable<TwelveTone> for GenericChord<TwelveTone> {
    fn get_frequencies(&self, tuning: &dyn Tuning<TwelveTone>, base_frequency: f32) -> Vec<f32> {
        self.clone()
            .set_base_note(Note::default())
            .to_notes()
            .as_slice()
            .get_frequencies(tuning, base_frequency)
    }
}

impl Playable<TwelveTone> for NoteChord<TwelveTone> {
    fn get_frequencies(&self, tuning: &dyn Tuning<TwelveTone>, base_frequency: f32) -> Vec<f32> {
        self.to_notes()
            .as_slice()
            .get_frequencies(tuning, base_frequency)
    }
}

impl Playable<TwelveTone> for Interval {
    fn get_frequencies(&self, tuning: &dyn Tuning<TwelveTone>, base_frequency: f32) -> Vec<f32> {
        Chord::from_semitones(&[0, self.to_semitones()]).get_frequencies(tuning, base_frequency)
    }
}

impl Playable<TwelveTone> for TwelveTone {
    fn get_frequencies(&self, tuning: &dyn Tuning<TwelveTone>, base_frequency: f32) -> Vec<f32> {
        Note::from(*self, 4).get_frequencies(tuning, base_frequency)
    }
}

impl Playable<TwelveTone> for Scale {
    fn get_frequencies(&self, tuning: &dyn Tuning<TwelveTone>, base_frequency: f32) -> Vec<f32> {
        Chord::from_semitones(&self.to_semitones()).get_frequencies(tuning, base_frequency)
    }
}

/// An enum that can be used to control the direction of arpeggiation.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub enum ArpeggioDirection {
    /// The arpeggio ascends from lowest to highest frequency before looping back around to the
    /// lowest frequency.
    #[default]
    Up,
    /// The arpeggio descends from highest to lowest frequency before looping back around to the
    /// highest frequency.
    Down,
    /// The arpeggio ascends from lowest to highest frequency, and once it reaches its peak
    /// descends from highest to lowest frequency, essentially forming a cycle.
    UpDown,
}

impl fmt::Display for ArpeggioDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} arpeggio", self)
    }
}

/// An error which is returned when audio could not be played.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AudioPlayError {
    /// A more specific message that explains why specific audio could not be played.
    pub message: String,
}

impl Error for AudioPlayError {}

impl fmt::Display for AudioPlayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "audio could not be played - {}", self.message)
    }
}

impl From<&str> for AudioPlayError {
    fn from(value: &str) -> Self {
        Self {
            message: value.to_string(),
        }
    }
}

impl From<String> for AudioPlayError {
    fn from(value: String) -> Self {
        Self { message: value }
    }
}

impl FromStr for AudioPlayError {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            message: s.to_string(),
        })
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

/// The sine wave function with a period of 1 unit of time.
pub const SINE_WAVE: fn(f32) -> f32 = sine_wave;
/// The square wave function with a period of 1 unit of time.
pub const SQUARE_WAVE: fn(f32) -> f32 = square_wave;
/// The triangle wave function with a period of 1 unit of time.
pub const TRIANGLE_WAVE: fn(f32) -> f32 = triangle_wave;
/// The sawtooth wave function with a period of 1 unit of time.
pub const SAWTOOTH_WAVE: fn(f32) -> f32 = sawtooth_wave;
