// In this example we create a seven tone equal temperament system of notes. To do this a new pitch
// class system is defined with only seven tones (P1, P2, P3, P4, P5, P6 and P7).
use music_tools::{note::Note, pitchclass::PitchClass};
use std::fmt;

// This enum holds the seven tones of our pitch class system we will use.
#[derive(Copy, Clone, Debug)]
enum SevenTone {
    P1,
    P2,
    P3,
    P4,
    P5,
    P6,
    P7,
}

// This array is defined to make the code in the offset function shorter.
const SEVEN_TONE_PITCH_CLASSES: [SevenTone; 7] = [
    SevenTone::P1,
    SevenTone::P2,
    SevenTone::P3,
    SevenTone::P4,
    SevenTone::P5,
    SevenTone::P6,
    SevenTone::P7,
];

impl PitchClass for SevenTone {
    // Each tone in the pitch class system has to have a unique value, which will allow us to
    // differentiate from each tone and also compare tones to each other. If two tones have the
    // same value, they are considered to be the same tone. This is useful for example in the twelve
    // tone system where pitch classes such as D♯ and E♭ are the same.
    fn get_value(&self) -> usize {
        match self {
            Self::P1 => 0,
            Self::P2 => 1,
            Self::P3 => 2,
            Self::P4 => 3,
            Self::P5 => 4,
            Self::P6 => 5,
            Self::P7 => 6,
        }
    }

    // There are seven tones in the seven tone system.
    fn get_num_classes(&self) -> usize {
        7
    }

    // We want the first tone (P1) to be the one used for the base frequency of 440 Hz.
    fn base_frequency_class_value(&self) -> usize {
        Self::P1.get_value()
    }

    // We use the SEVEN_TONE_PITCH_CLASSES array defined earlier to find the tone at a given offset
    // from the current tone. We assume that the value of the tone is the same as the index of the
    // tone in the array.
    fn offset(&self, offset: isize) -> Self
    where
        Self: Sized,
    {
        SEVEN_TONE_PITCH_CLASSES[(self.get_value() as isize + offset)
            .rem_euclid(self.get_num_classes() as isize) as usize]
    }
}

// We define that two tones are the same if their values are the same. This function can be changed
// if you want to add some special exceptions to the rule.
impl PartialEq for SevenTone {
    fn eq(&self, other: &Self) -> bool {
        self.get_value() == other.get_value()
    }
}

// We display the tone as the letter P followed by its value plus one if turned into a string.
impl fmt::Display for SevenTone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "P{}", self.get_value() + 1)
    }
}

fn main() {
    // This function will always assume equal temperament. To use another tuning or temperament
    // use the function new_with_tuning instead. In this case though we do want equal temperament.
    let p1_4 = Note::new(SevenTone::P1, 4);

    // Print the frequencies of the notes from P1 octave 4 to P1 octave 5 with our newly defined
    // system.
    for i in 0..8 {
        let current_note = p1_4.offset(i);
        println!(
            "Frequency of note with pitch class {} and octave {}: {}",
            current_note.get_pitch_class(),
            current_note.get_octave(),
            current_note.get_frequency()
        );
    }
}
