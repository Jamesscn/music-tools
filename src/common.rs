use crate::note::Note;
use crate::pitchclass::PitchClass;
use std::any::Any;
use std::error::Error;
use std::fmt;
use std::hash::Hash;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use std::time::Duration;

/// A structure which is used to hold the exact representation of a fraction. Fractions are used in
/// this library to precisely represent time signatures and the durations of beats. These fractions
/// are not simplified when they are stored.
#[derive(Copy, Clone, Debug, Eq)]
pub struct Fraction {
    numerator: u64,
    denominator: u64,
}

impl Fraction {
    /// Creates a new fraction with a given numerator and denominator.
    ///
    /// # Parameters
    ///
    /// - `numerator`: A positive integer representing the numerator of the fraction.
    /// - `denominator`: A positive integer representing the denominator of the fraction.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use music_tools::common::Fraction;
    ///
    /// let one_half = Fraction::new(1, 2);
    /// assert_eq!(one_half.get_as_float(), 0.5);
    /// ```
    ///
    /// # Panics
    ///
    /// This function panics if the denominator is equal to zero.
    pub const fn new(numerator: u64, denominator: u64) -> Self {
        if denominator == 0 {
            panic!("Cannot create a fraction with a denominator of zero!");
        }
        Self {
            numerator,
            denominator,
        }
    }

    /// Returns the numerator or top half of the fraction.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use music_tools::common::Fraction;
    ///
    /// let five_sevenths = Fraction::new(5, 7);
    /// let numerator = five_sevenths.get_numerator();
    /// assert_eq!(5, numerator);
    /// ```
    pub fn get_numerator(&self) -> u64 {
        self.numerator
    }

    /// Returns the denominator or bottom half of the fraction.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use music_tools::common::Fraction;
    ///
    /// let five_sevenths = Fraction::new(5, 7);
    /// let denominator = five_sevenths.get_denominator();
    /// assert_eq!(7, denominator);
    /// ```
    pub fn get_denominator(&self) -> u64 {
        self.denominator
    }

    /// Returns the value of the fraction as a floating point number. This can panic if the
    /// denominator is zero.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use music_tools::common::Fraction;
    ///
    /// let two_and_a_half = Fraction::new(5, 2);
    /// let float_value = two_and_a_half.get_as_float();
    /// assert_eq!(2.5, float_value);
    /// ```
    pub fn get_as_float(&self) -> f32 {
        self.numerator as f32 / self.denominator as f32
    }

    /// Returns a new fraction with a simplified numerator and denominator.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use music_tools::common::Fraction;
    ///
    /// let two_quarters = Fraction::new(2, 4);
    /// let one_half = Fraction::new(1, 2);
    /// assert_ne!(two_quarters.get_numerator(), one_half.get_numerator());
    /// assert_ne!(two_quarters.get_denominator(), one_half.get_denominator());
    /// assert_eq!(two_quarters.get_as_float(), one_half.get_as_float());
    ///
    /// let two_quarters_simplified = two_quarters.get_simplified();
    /// assert_eq!(two_quarters_simplified.get_numerator(), one_half.get_numerator());
    /// assert_eq!(two_quarters_simplified.get_denominator(), one_half.get_denominator());
    /// assert_eq!(two_quarters_simplified.get_as_float(), one_half.get_as_float());
    /// ```
    pub fn get_simplified(&self) -> Self {
        let common_factor = gcd(self.numerator, self.denominator);
        Self {
            numerator: self.numerator / common_factor,
            denominator: self.denominator / common_factor,
        }
    }
}

impl Default for Fraction {
    fn default() -> Self {
        Self {
            numerator: 1,
            denominator: 1,
        }
    }
}

impl PartialEq for Fraction {
    fn eq(&self, other: &Self) -> bool {
        let left_simplified = self.get_simplified();
        let right_simplified = other.get_simplified();
        left_simplified.numerator == right_simplified.numerator
            && left_simplified.denominator == right_simplified.denominator
    }
}

impl PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Fraction {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.get_as_float()
            .partial_cmp(&other.get_as_float())
            .unwrap()
    }
}

impl Hash for Fraction {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let hash_tuple = (self.numerator, self.denominator);
        hash_tuple.hash(state);
    }
}

impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}

impl Add for Fraction {
    type Output = Fraction;

    fn add(self, rhs: Self) -> Self::Output {
        let new_denominator = lcm(self.denominator, rhs.denominator);
        Fraction::new(
            self.numerator * new_denominator / self.denominator
                + rhs.numerator * new_denominator / rhs.denominator,
            new_denominator,
        )
    }
}

impl AddAssign for Fraction {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Fraction {
    type Output = Fraction;

    fn sub(self, rhs: Self) -> Self::Output {
        let new_denominator = lcm(self.denominator, rhs.denominator);
        Fraction::new(
            self.numerator * new_denominator / self.denominator
                - rhs.numerator * new_denominator / rhs.denominator,
            new_denominator,
        )
    }
}

impl SubAssign for Fraction {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Mul for Fraction {
    type Output = Fraction;

    fn mul(self, rhs: Self) -> Self::Output {
        Fraction::new(
            self.numerator * rhs.numerator,
            self.denominator * rhs.denominator,
        )
    }
}

impl MulAssign for Fraction {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl Div for Fraction {
    type Output = Fraction;

    fn div(self, rhs: Self) -> Self::Output {
        Fraction::new(
            self.numerator * rhs.denominator,
            self.denominator * rhs.numerator,
        )
    }
}

impl DivAssign for Fraction {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

/// The Beat type is the same as a [`Fraction`] but used to keep track of the duration of a
/// rhythmic beat with respect to the time signature.
pub type Beat = Fraction;

impl Beat {
    /// The duration corresponding to a whole note.
    pub const WHOLE: Self = Self::new(1, 1);
    /// The duration corresponding to a half note.
    pub const HALF: Self = Self::new(1, 2);
    /// The duration corresponding to a quarter note.
    pub const QUARTER: Self = Self::new(1, 4);
    /// The duration corresponding to an eighth note.
    pub const EIGHTH: Self = Self::new(1, 8);
    /// The duration corresponding to a sixteenth note.
    pub const SIXTEENTH: Self = Self::new(1, 16);
    /// The duration corresponding to a thirty-second note.
    pub const THIRTYSECOND: Self = Self::new(1, 32);
    /// The duration corresponding to a dotted whole note.
    pub const WHOLE_DOTTED: Self = Self::new(3, 2);
    /// The duration corresponding to a dotted half note.
    pub const HALF_DOTTED: Self = Self::new(3, 4);
    /// The duration corresponding to a dotted quarter note.
    pub const QUARTER_DOTTED: Self = Self::new(3, 8);
    /// The duration corresponding to a dotted eighth note.
    pub const EIGHTH_DOTTED: Self = Self::new(3, 16);
    /// The duration corresponding to a dotted sixteenth note.
    pub const SIXTEENTH_DOTTED: Self = Self::new(3, 32);
    /// The duration corresponding to a dotted thirty-second note.
    pub const THIRTYSECOND_DOTTED: Self = Self::new(3, 64);
}

/// A trait that defines a structure with a time duration for playing audio.
pub trait AudioDuration {
    /// A [`Duration`] representing the duration of time.
    fn get_duration(&self, tempo: f32) -> Duration;
}

impl AudioDuration for Beat {
    fn get_duration(&self, tempo: f32) -> Duration {
        Duration::from_millis((60000.0 * 4.0 * self.get_as_float() / tempo) as u64)
    }
}

impl AudioDuration for Duration {
    fn get_duration(&self, _: f32) -> Duration {
        *self
    }
}

/// The Rhythm type is nothing more than a [`Vec<Beat>`] which can be used by certain functions to
/// represent a rhythmical idea.
///
/// Examples
///
/// ```rust
/// use music_tools::common::{AudioDuration, Beat, Rhythm};
///
/// let mut rhythm = Rhythm::from(vec![Beat::HALF, Beat::HALF]);
/// rhythm.push(Beat::QUARTER);
/// rhythm.push(Beat::QUARTER);
/// for beat in rhythm.iter() {
///     println!("Beat duration: {}s", beat.get_duration(120.0).as_secs_f32());
/// }
/// ```
pub type Rhythm = Vec<Beat>;

/// This enum contains representations for the different types of triads that can be distinguished
/// or generated by the library.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub enum TriadQuality {
    #[default]
    /// A major triad, which consists of the tonic, a major third and a perfect fifth.
    Major,
    /// A minor triad, which consists of the tonic, a minor third and a perfect fifth.
    Minor,
    /// A suspended second triad, which consists of the tonic, a major second and a perfect fifth.
    Sus2,
    /// A suspended fourth triad, which consists of the tonic, a perfect fourth and a perfect
    /// fifth.
    Sus4,
    /// An augmented triad, which consists of the tonic, a major third and a minor sixth.
    Augmented,
    /// A diminished triad, which consists of the tonic, a minor third and a diminished fifth.
    Diminished,
}

impl fmt::Display for TriadQuality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} triad", self)
    }
}

pub trait Tuning: Clone + PartialEq {
    fn get_frequency<PitchClassType: PitchClass>(
        &self,
        base_frequency: f32,
        base_note: Note<PitchClassType>,
        note: Note<PitchClassType>,
    ) -> f32;
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct EqualTemperament;

impl EqualTemperament {
    pub fn new() -> Self {
        Self
    }
}

impl Tuning for EqualTemperament {
    fn get_frequency<PitchClassType: PitchClass>(
        &self,
        base_frequency: f32,
        base_note: Note<PitchClassType>,
        note: Note<PitchClassType>,
    ) -> f32 {
        base_frequency
            * 2f32.powf(
                (note.get_value() - base_note.get_value()) as f32
                    / PitchClassType::get_num_classes() as f32,
            )
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PythagoreanTuning {
    num_tones: usize,
    ratios: Vec<Fraction>,
}

impl PythagoreanTuning {
    pub fn new(num_tones: usize) -> Self {
        let mut ratios: Vec<Fraction> = Vec::new();
        let mut current_fraction = Fraction::new(1, 1);
        let three_halves = Fraction::new(3, 2);
        let one_half = Fraction::new(1, 2);
        ratios.push(current_fraction);
        for _ in 1..num_tones {
            current_fraction *= three_halves; //Go up a fifth
            if current_fraction.get_as_float() > 2f32 {
                current_fraction *= one_half; // Go down an octave if the ratio is too big
            }
            ratios.push(current_fraction)
        }
        ratios.sort();
        Self { num_tones, ratios }
    }
}

impl Tuning for PythagoreanTuning {
    fn get_frequency<PitchClassType: PitchClass>(
        &self,
        base_frequency: f32,
        base_note: Note<PitchClassType>,
        note: Note<PitchClassType>,
    ) -> f32 {
        let octave_difference = (note.get_value() - base_note.get_value())
            .div_floor(PitchClassType::get_num_classes() as i32);
        let ratio_index = (note.get_pitch_class().get_value()
            - base_note.get_pitch_class().get_value())
        .rem_euclid(PitchClassType::get_num_classes());
        base_frequency * 2f32.powi(octave_difference) * self.ratios[ratio_index].get_as_float()
    }
}

/// An error which is returned when a function receives an input that was not in the expected
/// format.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InputError {
    /// A more specific message that explains the reason why the string was invalid.
    pub message: String,
}

impl Error for InputError {}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid input provided - {}", self.message)
    }
}

pub fn convert_error_to_input_error<E: 'static>(
    error: E,
    conversion_failure_message: String,
) -> InputError {
    let error_ref: &dyn Any = &error;
    match error_ref.downcast_ref::<InputError>() {
        Some(input_error) => (*input_error).clone(),
        None => InputError {
            message: conversion_failure_message,
        },
    }
}

/// An error which is returned when a [`crate::chord::Chord`] without a tonic or octave is used in
/// an operation that requires either or both of these items to be defined for the chord.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct IncompleteChordError {
    /// A boolean indicating if the chord needs a tonic in order to perform the operation.
    pub needs_tonic: bool,
    /// A boolean indicating if the chord needs an octave in order to perform the operation.
    pub needs_octave: bool,
    /// A boolean indicating if the chord has a tonic.
    pub has_tonic: bool,
    /// A boolean indicating if the chord has an octave.
    pub has_octave: bool,
}

impl Error for IncompleteChordError {}

impl fmt::Display for IncompleteChordError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let requirements = if self.needs_tonic && self.needs_octave {
            "a tonic and an octave"
        } else if self.needs_tonic {
            "a tonic"
        } else if self.needs_octave {
            "an octave"
        } else {
            unreachable!("IncompleteChordError was thrown but not needed")
        };
        let given = if self.has_tonic && self.has_octave {
            unreachable!("IncompleteChordError was thrown but not needed")
        } else if self.has_tonic {
            "only had a tonic"
        } else if self.has_octave {
            "only had an octave"
        } else {
            "did not have a tonic nor an octave"
        };
        write!(
            f,
            "operation requiring a chord with {requirements} was called but the chord {given}",
        )
    }
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}
