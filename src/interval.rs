use crate::common::InputError;
use lazy_static::lazy_static;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;
use std::sync::Mutex;

//TODO: add assumptions

/// This struct is used to define a set of intervals ranging from the perfect unison to the double
/// octave. This does not cover every single possible interval and there may be a need to
/// create custom implementations of intervals, in such cases one can use the [`Interval::new()`]
/// function, which will allow you to register a new interval globally.
#[derive(Clone, Debug, PartialEq, Eq)]
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
        match INTERVALS.try_lock() {
            Ok(ref mut mutex) => {
                mutex.insert(semitones, interval.clone());
                Ok(interval)
            }
            Err(_) => Err(InputError {
                message: String::from("could not lock mutex while creating interval"),
            }),
        }
    }

    pub fn from_semitones(semitones: impl Into<usize>) -> Result<Self, InputError> {
        let semitones = semitones.into();
        Ok(INTERVALS.try_lock().map_err(|_| InputError {
            message: String::from("could not log mutex while fetching interval")
        })?.get(&semitones).ok_or(InputError {
            message: format!("there is no interval for {semitones} semitones, try registering one with Interval::new()")
        })?.clone())
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

impl PartialOrd for Interval {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Interval {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_semitones().cmp(&other.to_semitones())
    }
}

impl Hash for Interval {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.to_semitones().hash(state);
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

lazy_static! {
    /// The interval between two identical notes.
    pub static ref PERFECT_UNISON: Interval = Interval {
        semitones: 0_usize,
        full_name: "Perfect Unison".to_string(),
        short_name: "P1".to_string()
    };
    /// The interval between two notes separated a semitone.
    pub static ref MINOR_SECOND: Interval = Interval {
        semitones: 1_usize,
        full_name: "Minor Second".to_string(),
        short_name: "m2".to_string()
    };
    /// The interval between two notes separated by two semitones.
    pub static ref MAJOR_SECOND: Interval = Interval {
        semitones: 2_usize,
        full_name: "Major Second".to_string(),
        short_name: "M2".to_string()
    };
    /// The interval between two notes separated by three semitones.
    pub static ref MINOR_THIRD: Interval = Interval {
        semitones: 3_usize,
        full_name: "Minor Third".to_string(),
        short_name: "m3".to_string()
    };
    /// The interval between two notes separated by four semitones.
    pub static ref MAJOR_THIRD: Interval = Interval {
        semitones: 4_usize,
        full_name: "Major Third".to_string(),
        short_name: "M3".to_string()
    };
    /// The interval between two notes separated by five semitones.
    pub static ref PERFECT_FOURTH: Interval = Interval {
        semitones: 5_usize,
        full_name: "Perfect Fourth".to_string(),
        short_name: "P4".to_string()
    };
    /// The interval between two notes separated by six semitones, which is also equivalent to the
    /// tritone and the augmented fourth.
    pub static ref DIMINISHED_FIFTH: Interval = Interval {
        semitones: 6_usize,
        full_name: "Diminished Fifth".to_string(),
        short_name: "d5".to_string()
    };
    /// The interval between two notes separated by six semitones, which is also equivalent to the
    /// diminished fifth and the augmented fourth.
    pub static ref TRITONE: Interval = Interval {
        semitones: 6_usize,
        full_name: "Tritone".to_string(),
        short_name: "TT".to_string()
    };
    /// The interval between two notes separated by six semitones, which is also equivalent to the
    /// tritone and the diminished fifth.
    pub static ref AUGMENTED_FOURTH: Interval = Interval {
        semitones: 6_usize,
        full_name: "Augmented Fourth".to_string(),
        short_name: "A4".to_string()
    };
    /// The interval between two notes separated by seven semitones.
    pub static ref PERFECT_FIFTH: Interval = Interval {
        semitones: 7_usize,
        full_name: "Perfect Fifth".to_string(),
        short_name: "P5".to_string()
    };
    /// The interval between two notes separated by eight semitones.
    pub static ref MINOR_SIXTH: Interval = Interval {
        semitones: 8_usize,
        full_name: "Minor Sixth".to_string(),
        short_name: "m6".to_string()
    };
    /// The interval between two notes separated by nine semitones.
    pub static ref MAJOR_SIXTH: Interval = Interval {
        semitones: 9_usize,
        full_name: "Major Sixth".to_string(),
        short_name: "M6".to_string()
    };
    /// The interval between two notes separated by ten semitones.
    pub static ref MINOR_SEVENTH: Interval = Interval {
        semitones: 10_usize,
        full_name: "Minor Seventh".to_string(),
        short_name: "m7".to_string()
    };
    /// The interval between two notes separated by eleven semitones.
    pub static ref MAJOR_SEVENTH: Interval = Interval {
        semitones: 11_usize,
        full_name: "Major Seventh".to_string(),
        short_name: "M7".to_string()
    };
    /// The interval between two notes separated by twelve semitones or an octave.
    pub static ref PERFECT_OCTAVE: Interval = Interval {
        semitones: 12_usize,
        full_name: "Perfect Octave".to_string(),
        short_name: "P8".to_string()
    };
    /// The interval between two notes separated by thirteen semitones.
    pub static ref MINOR_NINTH: Interval = Interval {
        semitones: 13_usize,
        full_name: "Minor Ninth".to_string(),
        short_name: "m9".to_string()
    };
    /// The interval between two notes separated by fourteen semitones.
    pub static ref MAJOR_NINTH: Interval = Interval {
        semitones: 14_usize,
        full_name: "Major Ninth".to_string(),
        short_name: "M9".to_string()
    };
    /// The interval between two notes separated by fifteen semitones.
    pub static ref MINOR_TENTH: Interval = Interval {
        semitones: 15_usize,
        full_name: "Minor Tenth".to_string(),
        short_name: "m10".to_string()
    };
    /// The interval between two notes separated by sixteen semitones.
    pub static ref MAJOR_TENTH: Interval = Interval {
        semitones: 16_usize,
        full_name: "Major Tenth".to_string(),
        short_name: "M10".to_string()
    };
    /// The interval between two notes separated by seventeen semitones.
    pub static ref PERFECT_ELEVENTH: Interval = Interval {
        semitones: 17_usize,
        full_name: "Perfect Eleventh".to_string(),
        short_name: "P11".to_string()
    };
    /// The interval between two notes separated by eighteen semitones, which is also equivalent to
    /// the augmented eleventh.
    pub static ref DIMINISHED_TWELFTH: Interval = Interval {
        semitones: 18_usize,
        full_name: "Diminished Twelfth".to_string(),
        short_name: "d12".to_string()
    };
    /// The interval between two notes separated by eighteen semitones, which is also equivalent to
    /// the diminished twelfth.
    pub static ref AUGMENTED_ELEVENTH: Interval = Interval {
        semitones: 18_usize,
        full_name: "Augmented Eleventh".to_string(),
        short_name: "A11".to_string()
    };
    /// The interval between two notes separated by nineteen semitones.
    pub static ref PERFECT_TWELFTH: Interval = Interval {
        semitones: 19_usize,
        full_name: "Perfect Twelfth".to_string(),
        short_name: "P12".to_string() };
    /// The interval between two notes separated by twenty semitones.
    pub static ref MINOR_THIRTEENTH: Interval = Interval {
        semitones: 20_usize,
        full_name: "Minor Thirteenth".to_string(),
        short_name: "m13".to_string() };
    /// The interval between two notes separated by twenty one semitones.
    pub static ref MAJOR_THIRTEENTH: Interval = Interval {
        semitones: 21_usize,
        full_name: "Major Thirteenth".to_string(),
        short_name: "M13".to_string() };
    /// The interval between two notes separated by twenty two semitones.
    pub static ref MINOR_FOURTEENTH: Interval = Interval {
        semitones: 22_usize,
        full_name: "Minor Fourteenth".to_string(),
        short_name: "m14".to_string() };
    /// The interval between two notes separated by twenty three semitones.
    pub static ref MAJOR_FOURTEENTH: Interval = Interval {
        semitones: 23_usize,
        full_name: "Major Fourteenth".to_string(),
        short_name: "M14".to_string() };
    /// The interval between two notes separated by twenty four semitones or two octaves.
    pub static ref PERFECT_FIFTEENTH: Interval = Interval {
        semitones: 24_usize,
        full_name: "Perfect Fifteenth".to_string(),
        short_name: "P15".to_string()
    };
    static ref INTERVALS: Mutex<HashMap<usize, Interval>> = Mutex::new([
        &*PERFECT_UNISON,
        &*MINOR_SECOND,
        &*MAJOR_SECOND,
        &*MINOR_THIRD,
        &*MAJOR_THIRD,
        &*PERFECT_FOURTH,
        &*TRITONE,
        &*PERFECT_FIFTH,
        &*MINOR_SIXTH,
        &*MAJOR_SIXTH,
        &*MINOR_SEVENTH,
        &*MAJOR_SEVENTH,
        &*PERFECT_OCTAVE,
        &*MINOR_NINTH,
        &*MAJOR_NINTH,
        &*MINOR_TENTH,
        &*MAJOR_TENTH,
        &*PERFECT_ELEVENTH,
        &*AUGMENTED_ELEVENTH,
        &*PERFECT_TWELFTH,
        &*MINOR_THIRTEENTH,
        &*MAJOR_THIRTEENTH,
        &*MINOR_FOURTEENTH,
        &*MAJOR_FOURTEENTH,
        &*PERFECT_FIFTEENTH,
    ].iter().enumerate().map(|(index, interval)| (index, (*interval).clone())).collect::<HashMap<usize, Interval>>());
}
