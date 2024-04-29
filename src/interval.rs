use std::borrow::Cow;
use std::cmp::Ordering;
use std::fmt;
use std::hash::Hash;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Interval {
    name: Cow<'static, str>,
    semitones: usize,
    letter_classes: usize,
}

impl Interval {
    pub fn new(name: &str, semitones: usize, letter_classes: usize) -> Self {
        Self {
            name: Cow::Owned(name.to_string()),
            semitones,
            letter_classes,
        }
    }

    pub fn get_semitones(&self) -> usize {
        self.semitones
    }

    pub fn get_letter_classes(&self) -> usize {
        self.letter_classes
    }
}

impl fmt::Display for Interval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

macro_rules! interval {
    ($name: ident, $semitones: expr, $letter_classes: expr) => {
        pub const $name: Interval = Interval {
            name: Cow::Borrowed(stringify!($name)),
            semitones: $semitones,
            letter_classes: $letter_classes,
        };
    };
}

interval!(PERFECT_UNISON, 0, 0);
interval!(DIMINISHED_SECOND, 0, 1);
interval!(CHROMATIC_SEMITONE, 1, 0);
interval!(MINOR_SECOND, 1, 1);
interval!(WHOLE_TONE, 2, 1);
interval!(DIMINISHED_THIRD, 2, 2);
interval!(AUGMENTED_SECOND, 3, 1);
interval!(MINOR_THIRD, 3, 2);
interval!(MAJOR_THIRD, 4, 2);
interval!(DIMINISHED_FOURTH, 4, 3);
interval!(AUGMENTED_THIRD, 5, 2);
interval!(PERFECT_FOURTH, 5, 3);
interval!(AUGMENTED_FOURTH, 6, 3);
interval!(DIMINISHED_FIFTH, 6, 4);
interval!(PERFECT_FIFTH, 7, 4);
interval!(DIMINISHED_SIXTH, 7, 5);
interval!(AUGMENTED_FIFTH, 8, 4);
interval!(MINOR_SIXTH, 8, 5);
interval!(MAJOR_SIXTH, 9, 5);
interval!(DIMINISHED_SEVENTH, 9, 6);
interval!(AUGMENTED_SIXTH, 10, 5);
interval!(MINOR_SEVENTH, 10, 6);
interval!(MAJOR_SEVENTH, 11, 6);
interval!(DIMINISHED_OCTAVE, 11, 7);
interval!(AUGMENTED_SEVENTH, 12, 6);
interval!(PERFECT_OCTAVE, 12, 7);

impl Default for Interval {
    fn default() -> Self {
        PERFECT_UNISON
    }
}

impl PartialOrd for Interval {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Interval {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.get_semitones() == other.get_semitones() {
            self.get_letter_classes().cmp(&other.get_letter_classes())
        } else {
            self.get_semitones().cmp(&other.get_semitones())
        }
    }
}
