use crate::common::InputError;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;
use std::str::FromStr;
use std::sync::{Mutex, MutexGuard, OnceLock};

//TODO: add assumptions

/// This struct is used to define a set of intervals ranging from the perfect unison to the double
/// octave. This does not cover every single possible interval and there may be a need to
/// create custom implementations of intervals, in such cases one can use the [`Interval::new()`]
/// function, which will allow you to register a new interval globally.
#[derive(Clone, Debug)]
pub struct Interval {
    semitones: usize,
    full_name: String,
    short_name: String,
}

impl Interval {
    pub fn new(
        semitones: impl Into<usize>,
        full_name: impl Into<String>,
        short_name: impl Into<String>,
    ) -> Result<Self, InputError> {
        let semitones = semitones.into();
        let interval = Self {
            semitones,
            full_name: full_name.into(),
            short_name: short_name.into(),
        };
        let ref mut mutex = Self::static_intervals()?;
        mutex.insert(semitones, interval.clone());
        Ok(interval)
    }

    pub fn from_semitones(semitones: impl Into<usize>) -> Result<Self, InputError> {
        let semitones = semitones.into();
        let ref mut mutex = Self::static_intervals()?;
        mutex.get(&semitones).ok_or(InputError {
            message: format!("there is no interval for {semitones} semitones, try registering one with Interval::new()")
        }).map(|interval| interval.clone())
    }

    pub fn from_string(string: &str) -> Result<Self, InputError> {
        todo!();
    }

    pub fn to_semitones(&self) -> usize {
        self.semitones
    }

    /// Returns the full name of the interval, such as Perfect Unison or Diminished Fifth.
    pub fn get_name(&self) -> String {
        self.full_name.clone()
    }

    /// Returns an abbreviated name for the interval, such as P1 or m6.
    pub fn get_short_name(&self) -> String {
        self.short_name.clone()
    }
}

impl Default for Interval {
    fn default() -> Self {
        Self::PERFECT_UNISON()
    }
}

impl PartialEq for Interval {
    fn eq(&self, other: &Self) -> bool {
        self.semitones == other.semitones
    }
}

impl Eq for Interval {}

impl PartialOrd for Interval {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.semitones.cmp(&other.semitones))
    }
}

impl Ord for Interval {
    fn cmp(&self, other: &Self) -> Ordering {
        self.semitones.cmp(&other.semitones)
    }
}

impl Hash for Interval {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.semitones.hash(state);
    }
}

impl fmt::Display for Interval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.full_name)
    }
}

impl TryFrom<&str> for Interval {
    type Error = InputError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_string(value)
    }
}

impl TryFrom<String> for Interval {
    type Error = InputError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_string(&value)
    }
}

impl FromStr for Interval {
    type Err = InputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_string(s)
    }
}

impl TryFrom<usize> for Interval {
    type Error = InputError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Self::from_semitones(value)
    }
}

impl From<Interval> for String {
    fn from(value: Interval) -> Self {
        value.to_string()
    }
}

impl From<Interval> for usize {
    fn from(value: Interval) -> Self {
        value.to_semitones()
    }
}

impl FromIterator<Interval> for Vec<usize> {
    fn from_iter<T: IntoIterator<Item = Interval>>(iter: T) -> Self {
        iter.into_iter()
            .map(|interval| interval.to_semitones())
            .collect()
    }
}

#[allow(non_snake_case)]
impl Interval {
    /// The interval between two identical notes.
    pub fn PERFECT_UNISON() -> Self {
        Self {
            semitones: 0_usize,
            full_name: "Perfect Unison".to_string(),
            short_name: "P1".to_string(),
        }
    }
    /// The interval between two notes separated a semitone.
    pub fn MINOR_SECOND() -> Self {
        Self {
            semitones: 1_usize,
            full_name: "Minor Second".to_string(),
            short_name: "m2".to_string(),
        }
    }
    /// The interval between two notes separated by two semitones.
    pub fn MAJOR_SECOND() -> Self {
        Self {
            semitones: 2_usize,
            full_name: "Major Second".to_string(),
            short_name: "M2".to_string(),
        }
    }
    /// The interval between two notes separated by three semitones.
    pub fn MINOR_THIRD() -> Self {
        Self {
            semitones: 3_usize,
            full_name: "Minor Third".to_string(),
            short_name: "m3".to_string(),
        }
    }
    /// The interval between two notes separated by four semitones.
    pub fn MAJOR_THIRD() -> Self {
        Self {
            semitones: 4_usize,
            full_name: "Major Third".to_string(),
            short_name: "M3".to_string(),
        }
    }
    /// The interval between two notes separated by five semitones.
    pub fn PERFECT_FOURTH() -> Self {
        Self {
            semitones: 5_usize,
            full_name: "Perfect Fourth".to_string(),
            short_name: "P4".to_string(),
        }
    }
    /// The interval between two notes separated by six semitones, which is also equivalent to the
    /// tritone and the augmented fourth.
    pub fn DIMINISHED_FIFTH() -> Self {
        Self {
            semitones: 6_usize,
            full_name: "Diminished Fifth".to_string(),
            short_name: "d5".to_string(),
        }
    }
    /// The interval between two notes separated by six semitones, which is also equivalent to the
    /// diminished fifth and the augmented fourth.
    pub fn TRITONE() -> Self {
        Self {
            semitones: 6_usize,
            full_name: "Tritone".to_string(),
            short_name: "TT".to_string(),
        }
    }
    /// The interval between two notes separated by six semitones, which is also equivalent to the
    /// tritone and the diminished fifth.
    pub fn AUGMENTED_FOURTH() -> Self {
        Self {
            semitones: 6_usize,
            full_name: "Augmented Fourth".to_string(),
            short_name: "A4".to_string(),
        }
    }
    /// The interval between two notes separated by seven semitones.
    pub fn PERFECT_FIFTH() -> Self {
        Self {
            semitones: 7_usize,
            full_name: "Perfect Fifth".to_string(),
            short_name: "P5".to_string(),
        }
    }
    /// The interval between two notes separated by eight semitones.
    pub fn MINOR_SIXTH() -> Self {
        Self {
            semitones: 8_usize,
            full_name: "Minor Sixth".to_string(),
            short_name: "m6".to_string(),
        }
    }
    /// The interval between two notes separated by nine semitones.
    pub fn MAJOR_SIXTH() -> Self {
        Self {
            semitones: 9_usize,
            full_name: "Major Sixth".to_string(),
            short_name: "M6".to_string(),
        }
    }
    /// The interval between two notes separated by ten semitones.
    pub fn MINOR_SEVENTH() -> Self {
        Self {
            semitones: 10_usize,
            full_name: "Minor Seventh".to_string(),
            short_name: "m7".to_string(),
        }
    }
    /// The interval between two notes separated by eleven semitones.
    pub fn MAJOR_SEVENTH() -> Self {
        Self {
            semitones: 11_usize,
            full_name: "Major Seventh".to_string(),
            short_name: "M7".to_string(),
        }
    }
    /// The interval between two notes separated by twelve semitones or an octave.
    pub fn PERFECT_OCTAVE() -> Self {
        Self {
            semitones: 12_usize,
            full_name: "Perfect Octave".to_string(),
            short_name: "P8".to_string(),
        }
    }
    /// The interval between two notes separated by thirteen semitones.
    pub fn MINOR_NINTH() -> Self {
        Self {
            semitones: 13_usize,
            full_name: "Minor Ninth".to_string(),
            short_name: "m9".to_string(),
        }
    }
    /// The interval between two notes separated by fourteen semitones.
    pub fn MAJOR_NINTH() -> Self {
        Self {
            semitones: 14_usize,
            full_name: "Major Ninth".to_string(),
            short_name: "M9".to_string(),
        }
    }
    /// The interval between two notes separated by fifteen semitones.
    pub fn MINOR_TENTH() -> Self {
        Self {
            semitones: 15_usize,
            full_name: "Minor Tenth".to_string(),
            short_name: "m10".to_string(),
        }
    }
    /// The interval between two notes separated by sixteen semitones.
    pub fn MAJOR_TENTH() -> Self {
        Self {
            semitones: 16_usize,
            full_name: "Major Tenth".to_string(),
            short_name: "M10".to_string(),
        }
    }
    /// The interval between two notes separated by seventeen semitones.
    pub fn PERFECT_ELEVENTH() -> Self {
        Self {
            semitones: 17_usize,
            full_name: "Perfect Eleventh".to_string(),
            short_name: "P11".to_string(),
        }
    }
    /// The interval between two notes separated by eighteen semitones, which is also equivalent to
    /// the augmented eleventh.
    pub fn DIMINISHED_TWELFTH() -> Self {
        Self {
            semitones: 18_usize,
            full_name: "Diminished Twelfth".to_string(),
            short_name: "d12".to_string(),
        }
    }
    /// The interval between two notes separated by eighteen semitones, which is also equivalent to
    /// the diminished twelfth.
    pub fn AUGMENTED_ELEVENTH() -> Self {
        Self {
            semitones: 18_usize,
            full_name: "Augmented Eleventh".to_string(),
            short_name: "A11".to_string(),
        }
    }
    /// The interval between two notes separated by nineteen semitones.
    pub fn PERFECT_TWELFTH() -> Self {
        Self {
            semitones: 19_usize,
            full_name: "Perfect Twelfth".to_string(),
            short_name: "P12".to_string(),
        }
    }
    /// The interval between two notes separated by twenty semitones.
    pub fn MINOR_THIRTEENTH() -> Self {
        Self {
            semitones: 20_usize,
            full_name: "Minor Thirteenth".to_string(),
            short_name: "m13".to_string(),
        }
    }
    /// The interval between two notes separated by twenty one semitones.
    pub fn MAJOR_THIRTEENTH() -> Self {
        Self {
            semitones: 21_usize,
            full_name: "Major Thirteenth".to_string(),
            short_name: "M13".to_string(),
        }
    }
    /// The interval between two notes separated by twenty two semitones.
    pub fn MINOR_FOURTEENTH() -> Self {
        Self {
            semitones: 22_usize,
            full_name: "Minor Fourteenth".to_string(),
            short_name: "m14".to_string(),
        }
    }
    /// The interval between two notes separated by twenty three semitones.
    pub fn MAJOR_FOURTEENTH() -> Self {
        Self {
            semitones: 23_usize,
            full_name: "Major Fourteenth".to_string(),
            short_name: "M14".to_string(),
        }
    }
    /// The interval between two notes separated by twenty four semitones or two octaves.
    pub fn PERFECT_FIFTEENTH() -> Self {
        Self {
            semitones: 24_usize,
            full_name: "Perfect Fifteenth".to_string(),
            short_name: "P15".to_string(),
        }
    }
    fn static_intervals() -> Result<MutexGuard<'static, HashMap<usize, Interval>>, InputError> {
        INTERVALS
            .get_or_init(|| {
                Mutex::new(
                    [
                        Interval::PERFECT_UNISON(),
                        Interval::MINOR_SECOND(),
                        Interval::MAJOR_SECOND(),
                        Interval::MINOR_THIRD(),
                        Interval::MAJOR_THIRD(),
                        Interval::PERFECT_FOURTH(),
                        Interval::TRITONE(),
                        Interval::PERFECT_FIFTH(),
                        Interval::MINOR_SIXTH(),
                        Interval::MAJOR_SIXTH(),
                        Interval::MINOR_SEVENTH(),
                        Interval::MAJOR_SEVENTH(),
                        Interval::PERFECT_OCTAVE(),
                        Interval::MINOR_NINTH(),
                        Interval::MAJOR_NINTH(),
                        Interval::MINOR_TENTH(),
                        Interval::MAJOR_TENTH(),
                        Interval::PERFECT_ELEVENTH(),
                        Interval::AUGMENTED_ELEVENTH(),
                        Interval::PERFECT_TWELFTH(),
                        Interval::MINOR_THIRTEENTH(),
                        Interval::MAJOR_THIRTEENTH(),
                        Interval::MINOR_FOURTEENTH(),
                        Interval::MAJOR_FOURTEENTH(),
                        Interval::PERFECT_FIFTEENTH(),
                    ]
                    .iter()
                    .enumerate()
                    .map(|(index, interval)| (index, (*interval).clone()))
                    .collect::<HashMap<usize, Interval>>(),
                )
            })
            .try_lock()
            .map_err(|_| InputError {
                message: String::from("could not lock mutex while accesing intervals"),
            })
    }
}

static INTERVALS: OnceLock<Mutex<HashMap<usize, Interval>>> = OnceLock::new();
