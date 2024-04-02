use std::cmp::Ordering;
use std::fmt;
use std::hash::Hash;

use crate::scale::Scale;

/// This trait defines an interval, which can be used to determine which note is a certain
/// interval away from another note. Implementations of the [`crate::common::Tuning`] trait can be
/// used to determine the frequencies of these intervals.
pub trait Interval: Clone {
    /// Returns an [`Option`] holding the [`Interval`] with a size of a certain number of semitones
    /// if that interval exists. Otherwise it returns [`None`].
    ///
    /// # Parameters
    ///
    /// - `semitones`: The difference in semitones of the interval to return, if it exists.
    fn from_semitones(semitones: usize) -> Option<Self>;
    /// Returns a positive integer representing the number of semitones held by the interval.
    fn get_semitones(&self) -> usize;
}

/// This struct is used to define a set of standard intervals ranging from the perfect unison to
/// the double octave. This does not cover every single possible interval and there may be a need to
/// create custom implementations of intervals, in such cases one should create their own structure
/// or enum implementing the [`Interval`] trait instead.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct StandardInterval {
    semitones: usize,
    full_name: &'static str,
    short_name: &'static str,
}

impl StandardInterval {
    /// Returns the full name of the interval if it exists, such as Perfect Unison or Diminished
    /// Fifth.
    pub fn get_name(&self) -> &'static str {
        self.full_name
    }

    /// Returns an abbreviated name for the interval if it exists, such as P1 or m6.
    pub fn get_short_name(&self) -> &'static str {
        self.short_name
    }
}

impl Interval for StandardInterval {
    fn from_semitones(semitones: usize) -> Option<Self> {
        INTERVALS.get(semitones).copied()
    }

    fn get_semitones(&self) -> usize {
        self.semitones
    }
}

impl PartialOrd for StandardInterval {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for StandardInterval {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_semitones().cmp(&other.get_semitones())
    }
}

impl Hash for StandardInterval {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.get_semitones().hash(state);
    }
}

impl fmt::Display for StandardInterval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.full_name)
    }
}

impl From<StandardInterval> for usize {
    fn from(value: StandardInterval) -> Self {
        value.get_semitones()
    }
}

impl From<StandardInterval> for isize {
    fn from(value: StandardInterval) -> Self {
        value.get_semitones() as isize
    }
}

/*
//TODO
impl From<Chord> for Vec<StandardInterval> {
    fn from(value: Chord) -> Self {
        value.get_intervals()
    }
}
*/

impl From<Scale> for Vec<StandardInterval> {
    fn from(value: Scale) -> Self {
        // As long as the scale interval values are between 0 and 12 this line should never panic.
        value.get_intervals().unwrap()
    }
}

/// The interval between two identical notes.
pub const PERFECT_UNISON: StandardInterval = StandardInterval {
    semitones: 0,
    full_name: "Perfect Unison",
    short_name: "P1",
};
/// The interval between two notes separated a semitone.
pub const MINOR_SECOND: StandardInterval = StandardInterval {
    semitones: 1,
    full_name: "Minor Second",
    short_name: "m2",
};
/// The interval between two notes separated by two semitones.
pub const MAJOR_SECOND: StandardInterval = StandardInterval {
    semitones: 2,
    full_name: "Major Second",
    short_name: "M2",
};
/// The interval between two notes separated by three semitones.
pub const MINOR_THIRD: StandardInterval = StandardInterval {
    semitones: 3,
    full_name: "Minor Third",
    short_name: "m3",
};
/// The interval between two notes separated by four semitones.
pub const MAJOR_THIRD: StandardInterval = StandardInterval {
    semitones: 4,
    full_name: "Major Third",
    short_name: "M3",
};
/// The interval between two notes separated by five semitones.
pub const PERFECT_FOURTH: StandardInterval = StandardInterval {
    semitones: 5,
    full_name: "Perfect Fourth",
    short_name: "P4",
};
/// The interval between two notes separated by six semitones, which is also equivalent to the
/// tritone and the augmented fourth.
pub const DIMINISHED_FIFTH: StandardInterval = StandardInterval {
    semitones: 6,
    full_name: "Diminished Fifth",
    short_name: "d5",
};
/// The interval between two notes separated by six semitones, which is also equivalent to the
/// diminished fifth and the augmented fourth.
pub const TRITONE: StandardInterval = StandardInterval {
    semitones: 6,
    full_name: "Tritone",
    short_name: "TT",
};
/// The interval between two notes separated by six semitones, which is also equivalent to the
/// tritone and the diminished fifth.
pub const AUGMENTED_FOURTH: StandardInterval = StandardInterval {
    semitones: 6,
    full_name: "Augmented Fourth",
    short_name: "A4",
};
/// The interval between two notes separated by seven semitones.
pub const PERFECT_FIFTH: StandardInterval = StandardInterval {
    semitones: 7,
    full_name: "Perfect Fifth",
    short_name: "P5",
};
/// The interval between two notes separated by eight semitones.
pub const MINOR_SIXTH: StandardInterval = StandardInterval {
    semitones: 8,
    full_name: "Minor Sixth",
    short_name: "m6",
};
/// The interval between two notes separated by nine semitones.
pub const MAJOR_SIXTH: StandardInterval = StandardInterval {
    semitones: 9,
    full_name: "Major Sixth",
    short_name: "M6",
};
/// The interval between two notes separated by ten semitones.
pub const MINOR_SEVENTH: StandardInterval = StandardInterval {
    semitones: 10,
    full_name: "Minor Seventh",
    short_name: "m7",
};
/// The interval between two notes separated by eleven semitones.
pub const MAJOR_SEVENTH: StandardInterval = StandardInterval {
    semitones: 11,
    full_name: "Major Seventh",
    short_name: "M7",
};
/// The interval between two notes separated by twelve semitones or an octave.
pub const PERFECT_OCTAVE: StandardInterval = StandardInterval {
    semitones: 12,
    full_name: "Perfect Octave",
    short_name: "P8",
};
/// The interval between two notes separated by thirteen semitones.
pub const MINOR_NINTH: StandardInterval = StandardInterval {
    semitones: 13,
    full_name: "Minor Ninth",
    short_name: "m9",
};
/// The interval between two notes separated by fourteen semitones.
pub const MAJOR_NINTH: StandardInterval = StandardInterval {
    semitones: 14,
    full_name: "Major Ninth",
    short_name: "M9",
};
/// The interval between two notes separated by fifteen semitones.
pub const MINOR_TENTH: StandardInterval = StandardInterval {
    semitones: 15,
    full_name: "Minor Tenth",
    short_name: "m10",
};
/// The interval between two notes separated by sixteen semitones.
pub const MAJOR_TENTH: StandardInterval = StandardInterval {
    semitones: 16,
    full_name: "Major Tenth",
    short_name: "M10",
};
/// The interval between two notes separated by seventeen semitones.
pub const PERFECT_ELEVENTH: StandardInterval = StandardInterval {
    semitones: 17,
    full_name: "Perfect Eleventh",
    short_name: "P11",
};
/// The interval between two notes separated by eighteen semitones, which is also equivalent to
/// the augmented eleventh.
pub const DIMINISHED_TWELFTH: StandardInterval = StandardInterval {
    semitones: 18,
    full_name: "Diminished Twelfth",
    short_name: "d12",
};
/// The interval between two notes separated by eighteen semitones, which is also equivalent to
/// the diminished twelfth.
pub const AUGMENTED_ELEVENTH: StandardInterval = StandardInterval {
    semitones: 18,
    full_name: "Augmented Eleventh",
    short_name: "A11",
};
/// The interval between two notes separated by nineteen semitones.
pub const PERFECT_TWELFTH: StandardInterval = StandardInterval {
    semitones: 19,
    full_name: "Perfect Twelfth",
    short_name: "P12",
};
/// The interval between two notes separated by twenty semitones.
pub const MINOR_THIRTEENTH: StandardInterval = StandardInterval {
    semitones: 20,
    full_name: "Minor Thirteenth",
    short_name: "m13",
};
/// The interval between two notes separated by twenty one semitones.
pub const MAJOR_THIRTEENTH: StandardInterval = StandardInterval {
    semitones: 21,
    full_name: "Major Thirteenth",
    short_name: "M13",
};
/// The interval between two notes separated by twenty two semitones.
pub const MINOR_FOURTEENTH: StandardInterval = StandardInterval {
    semitones: 22,
    full_name: "Minor Fourteenth",
    short_name: "m14",
};
/// The interval between two notes separated by twenty three semitones.
pub const MAJOR_FOURTEENTH: StandardInterval = StandardInterval {
    semitones: 23,
    full_name: "Major Fourteenth",
    short_name: "M14",
};
/// The interval between two notes separated by twenty four semitones or two octaves.
pub const PERFECT_FIFTEENTH: StandardInterval = StandardInterval {
    semitones: 24,
    full_name: "Perfect Fifteenth",
    short_name: "P15",
};

const INTERVALS: [StandardInterval; 25] = [
    PERFECT_UNISON,
    MINOR_SECOND,
    MAJOR_SECOND,
    MINOR_THIRD,
    MAJOR_THIRD,
    PERFECT_FOURTH,
    TRITONE,
    PERFECT_FIFTH,
    MINOR_SIXTH,
    MAJOR_SIXTH,
    MINOR_SEVENTH,
    MAJOR_SEVENTH,
    PERFECT_OCTAVE,
    MINOR_NINTH,
    MAJOR_NINTH,
    MINOR_TENTH,
    MAJOR_TENTH,
    PERFECT_ELEVENTH,
    AUGMENTED_ELEVENTH,
    PERFECT_TWELFTH,
    MINOR_THIRTEENTH,
    MAJOR_THIRTEENTH,
    MINOR_FOURTEENTH,
    MAJOR_FOURTEENTH,
    PERFECT_FIFTEENTH,
];
