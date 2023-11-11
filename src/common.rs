use std::error::Error;
use std::fmt;
use std::hash::Hash;
use std::ops::{Add, AddAssign, Sub, SubAssign};
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

/// This enum contains representations for the different modes or types of musical scales that can
/// be distinguished or generated by the library.
#[derive(Copy, Clone, Debug, Default, Eq)]
pub enum ScaleType {
    #[default]
    /// The major scale, which is the same as the Ionian mode.
    Major,
    /// The scale of the Ionian mode, which is the first mode and is the same as the major scale.
    Ionian,
    /// The scale of the Dorian mode, which is the second mode and is equal to natural minor scale
    /// with a major sixth instead of a minor sixth.
    Dorian,
    /// The scale of the Phrygian mode, which is the third mode and is equal to the natural minor
    /// scale with a minor second instead of a major second.
    Phrygian,
    /// The scale of the Lydian mode, which is the fourth mode and is equal to the major scale with
    /// an augmented fourth instead of a perfect fourth.
    Lydian,
    /// The scale of the Mixolydian mode, which is the fifth mode and is equal to the major scale
    /// with a minor seventh instead of a major seventh.
    Mixolydian,
    /// The modern minor scale, which differs from the natural minor scale and the Aeolian mode
    /// only in that it's fifth diatonic chord is major instead of minor.
    Minor,
    /// The natural minor scale, which is the same as the Aeolian mode.
    NaturalMinor,
    /// The descending melodic minor scale, which is the same as the natural minor scale but is
    /// intended to be used when playing the melodic minor scale in a descending manner.
    DescendingMelodicMinor,
    /// The scale of the Aeolian mode, which is the sixth mode and is the same as the natural minor
    /// scale.
    Aeolian,
    /// The scale of the Locrian mode, which is the seventh mode and is equal to the natural minor
    /// scale with a minor second instead of a major second and a diminished fifth instead of a
    /// perfect fifth.
    Locrian,
    /// The harmonic minor scale, which is equal to the natural minor scale with a major seventh
    /// instead of a minor seventh.
    HarmonicMinor,
    /// The Aeolian ♯7 scale, which is the same as the harmonic minor scale.
    AeolianSharpSeven,
    /// The Locrian ♮6 scale, which is the second mode of the harmonic minor scale and the same as
    /// the Locrian scale with a natural sixth.
    LocrianNaturalSix,
    /// The Ionian ♯5 scale, which is the third mode of the harmonic minor scale and the same as
    /// the Ionian scale with a sharp fifth.
    IonianSharpFive,
    /// The Dorian ♯4 scale, which is the fourth mode of the harmonic minor scale and the same as
    /// the Dorian scale with a sharp fourth.
    DorianSharpFour,
    /// The Romanian minor scale, which is the same as the Dorian ♯4 scale.
    RomanianMinor,
    /// The Ukranian Dorian scale, which is the same as the Dorian ♯4 scale.
    UkranianDorian,
    /// The Phrygian Dominant scale, which is the fifth mode of the harmonic minor scale and is the
    /// equal to the Phrygian scale with a major third instead of a minor third.
    PhrygianDominant,
    /// The Lydian ♯2 scale, which is the sixth mode of the harmonic minor scale and the same as
    /// the Lydian scale with a sharp second.
    LydianSharpTwo,
    /// The altered diminished scale, which is the seventh mode of the harmonic minor scale and the
    /// same as the Locrian scale with a flat fourth and a double flat seventh.
    AlteredDiminished,
    /// The Super Locrian ♭♭7 scale, which is the same as the altered diminished scale.
    SuperLocrianDoubleFlatSeven,
    /// The ascending melodic minor scale, which is equal to the natural minor scale with a major
    /// sixth and major seventh, and is intended to be used when playing the melodic minor scale in
    /// an ascending manner. Also known as just the melodic minor scale.
    AscendingMelodicMinor,
    /// The melodic minor scale, which is the same as the ascending melodic minor scale.
    MelodicMinor,
    /// The jazz minor scale, which is the same as the ascending melodic minor scale.
    JazzMinor,
    /// The Dorian ♭2 scale, which is the second mode of the melodic minor scale and the same as
    /// the Dorian scale but with a flat second.
    DorianFlatTwo,
    /// The Phrygian ♯6 scale, which is the same as the Dorian ♭2 scale.
    PhrygianSharpSix,
    /// The Lyidan augmented scale, which is the third mode of the melodic minor scale and the
    /// same as the major scale with a raised fourth and fifth.
    LydianAugmented,
    /// The Lydian dominant scale, which is the fourth mode of the melodic minor scale and the same
    /// as the mixolydian scale with a sharp fourth.
    LydianDominant,
    /// The overtone scale, which is the same as the Lydian dominant scale.
    Overtone,
    /// The acoustic scale, which is the same as the Lydian dominant scale.
    Acoustic,
    /// The Mixolydian ♯4 scale, which is the same as the Lydian dominant scale.
    MixolydianSharpFour,
    /// The Mixolydian ♭6 scale, which is the fifth mode of the melodic minor scale and the same as
    /// the major scale with a flat sixth and seventh.
    MixolydianFlatSix,
    /// The Aeolian dominant scale, which is the same as the Mixolydian ♭6 scale.
    AeolianDominant,
    /// The descending melodic major scale, which is the same as the Mixolydian ♭6 scale.
    DescendingMelodicMajor,
    /// The hindu scale, which is the same as the Mixolydian ♭6 scale.
    Hindu,
    /// The Locrian ♯2 scale, which is the sixth mode of the melodic minor scale and the same as
    /// the locrian scale with a natural second.
    LocrianSharpTwo,
    /// The Aeolian ♭5 scale, which is the same as the Locrian ♯2 scale.
    AeolianFlatFive,
    /// The half diminished scale, which is the same as the Locrian ♯2 scale.
    HalfDiminished,
    /// The altered scale, which is the seventh mode of the melodic minor scale and the same as the
    /// major scale with all four altered extensions of the major mode
    Altered,
    /// The altered dominant scale, which is the same as the altered scale.
    AlteredDominant,
    /// The super locrian scale, which is the same as the altered scale.
    SuperLocrian,
    /// The diminished scale, which contains an alternating pattern of whole tones followed by
    /// semitones, starting with a whole tone.
    Diminished,
    /// The dominant diminished scale, which contains an alternating pattern of semitones followed
    /// by whole tones, starting with a semitone.
    DominantDiminished,
    /// A nonatonic blues scale, which is derived from the major scale with an added flat third and
    /// an added flat seventh of the key.
    NonatonicBlues,
    /// The major blues scale, which is a hexatonic scale derived from the major pentatonic scale
    /// with an added flat third of the key.
    MajorBlues,
    /// The minor blues scale, which is a hexatonic scale derived from the minor pentatonic scale
    /// with an added flat fifth of the key.
    MinorBlues,
    /// The whole tone scale, which is a hexatonic scale where each tone or pitch class is
    /// separated by a whole note or two semitones.
    Whole,
    /// The chromatic scale, which consists of all twelve pitch classes separated by a semitone.
    Chromatic,
}

impl ScaleType {
    /// Returns an ID which can be used to compare scale types between each other.
    pub fn get_id(&self) -> u8 {
        match self {
            ScaleType::Major | ScaleType::Ionian => 1,
            ScaleType::Dorian => 2,
            ScaleType::Phrygian => 3,
            ScaleType::Lydian => 4,
            ScaleType::Mixolydian => 5,
            ScaleType::Minor
            | ScaleType::NaturalMinor
            | ScaleType::DescendingMelodicMinor
            | ScaleType::Aeolian => 6,
            ScaleType::Locrian => 7,
            ScaleType::HarmonicMinor | ScaleType::AeolianSharpSeven => 8,
            ScaleType::LocrianNaturalSix => 9,
            ScaleType::IonianSharpFive => 10,
            ScaleType::DorianSharpFour | ScaleType::RomanianMinor | ScaleType::UkranianDorian => 11,
            ScaleType::PhrygianDominant => 12,
            ScaleType::LydianSharpTwo => 13,
            ScaleType::AlteredDiminished | ScaleType::SuperLocrianDoubleFlatSeven => 14,
            ScaleType::AscendingMelodicMinor | ScaleType::MelodicMinor | ScaleType::JazzMinor => 15,
            ScaleType::DorianFlatTwo | ScaleType::PhrygianSharpSix => 16,
            ScaleType::LydianAugmented => 17,
            ScaleType::LydianDominant
            | ScaleType::Overtone
            | ScaleType::Acoustic
            | ScaleType::MixolydianSharpFour => 18,
            ScaleType::MixolydianFlatSix
            | ScaleType::AeolianDominant
            | ScaleType::DescendingMelodicMajor
            | ScaleType::Hindu => 19,
            ScaleType::LocrianSharpTwo | ScaleType::AeolianFlatFive | ScaleType::HalfDiminished => {
                20
            }
            ScaleType::Altered | ScaleType::AlteredDominant | ScaleType::SuperLocrian => 21,
            ScaleType::Diminished => 22,
            ScaleType::DominantDiminished => 23,
            ScaleType::NonatonicBlues => 24,
            ScaleType::MajorBlues => 25,
            ScaleType::MinorBlues => 26,
            ScaleType::Whole => 27,
            ScaleType::Chromatic => 28,
        }
    }
}

impl PartialEq for ScaleType {
    fn eq(&self, other: &Self) -> bool {
        self.get_id() == other.get_id()
    }
}

impl Hash for ScaleType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.get_id().hash(state);
    }
}

impl fmt::Display for ScaleType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut scale_string = format!("{:?}Scale", self)
            .chars()
            .map(|x| {
                if x.is_ascii_uppercase() {
                    " ".to_owned() + x.to_ascii_lowercase().to_string().as_str()
                } else {
                    x.to_string()
                }
            })
            .collect::<Vec<String>>()
            .join("");
        scale_string.replace_range(
            ..2,
            &scale_string
                .chars()
                .nth(1)
                .unwrap()
                .to_ascii_uppercase()
                .to_string(),
        );
        write!(f, "{}", scale_string)
    }
}

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

/// This enum is used to represent the type of a pentatonic.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub enum PentatonicType {
    #[default]
    /// Corresponds to no pentatonic.
    None,
    /// Corresponds to a major pentatonic.
    Major,
    /// Corresponds to a minor pentatonic.
    Minor,
}

impl fmt::Display for PentatonicType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pentatonic_name = match self {
            PentatonicType::None => String::from("No pentatonic"),
            _ => format!("{:?} pentatonic", self),
        };
        write!(f, "{pentatonic_name}")
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
