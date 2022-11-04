pub struct PitchClass {
    pub value: u8,
    pub names: [(&'static str, bool); 3]
}

pub const PITCH_CLASSES: [PitchClass; 12] = [
    PitchClass {
        value: 0,
        names: [("C", true), ("B#", false), ("Dbb", false)]
    },
    PitchClass {
        value: 1,
        names: [("C#", true), ("Db", true), ("Bx", false)]
    },
    PitchClass {
        value: 2,
        names: [("D", true), ("Ebb", false), ("Cx", false)]
    },
    PitchClass {
        value: 3,
        names: [("D#", true), ("Eb", true), ("Fbb", false)]
    },
    PitchClass {
        value: 4,
        names: [("E", true), ("Fb", false), ("Dx", false)]
    },
    PitchClass {
        value: 5,
        names: [("F", true), ("E#", false), ("Gbb", false)]
    },
    PitchClass {
        value: 6,
        names: [("F#", true), ("Gb", true), ("Ex", false)]
    },
    PitchClass {
        value: 7,
        names: [("G", true), ("Abb", false), ("Fx", false)]
    },
    PitchClass {
        value: 8,
        names: [("G#", true), ("Ab", true), ("Ab", true)]
    },
    PitchClass {
        value: 9,
        names: [("A", true), ("Bbb", false), ("Gx", false)]
    },
    PitchClass {
        value: 10,
        names: [("A#", true), ("Bb", true), ("Cbb", false)]
    },
    PitchClass {
        value: 11,
        names: [("B", true), ("Cb", false), ("Ax", false)]
    }
];

const LETTERS: [char; 7] = ['A', 'B', 'C', 'D', 'E', 'F', 'G'];

pub fn get_letter_at_increment(letter: char, increment: i8) -> char {
    for letter_index in 0..7 as i8 {
        let current_letter = LETTERS[letter_index as usize];
        if current_letter == letter {
            let new_index = (letter_index + increment) % 7;
            return LETTERS[new_index as usize];
        }
    }
    panic!("The letter {0} is not valid.", letter);
}

pub fn get_pitch_class(pitch_class_name: String) -> &'static PitchClass {
    for pitch_class_index in 0..12 {
        let current_pitch_class = &PITCH_CLASSES[pitch_class_index];
        for current_name in current_pitch_class.names {
            if current_name.0 == pitch_class_name {
                return &PITCH_CLASSES[pitch_class_index];
            } 
        }
    }
    panic!("Pitch class {0} does not exist.", pitch_class_name);
}

pub fn get_pitch_class_at_increment(current_pitch_class: &'static PitchClass, increment: i8) -> &'static PitchClass {
    let current_pitch_value = current_pitch_class.value as i8;
    let next_pitch_value = (current_pitch_value + increment) % 12;
    return &PITCH_CLASSES[next_pitch_value as usize];
}