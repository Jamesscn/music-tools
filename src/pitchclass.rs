pub struct PitchClass {
    pub value: u8,
    pub names: [(&'static str, bool); 3]
}

const PITCH_CLASSES: [PitchClass; 12] = [
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
        names: [("D", true), ("Cx", false), ("Ebb", false)]
    },
    PitchClass {
        value: 3,
        names: [("D#", true), ("Eb", true), ("Fbb", false)]
    },
    PitchClass {
        value: 4,
        names: [("E", true), ("Dx", false), ("Fb", false)]
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
        names: [("G", true), ("Fx", false), ("Abb", false)]
    },
    PitchClass {
        value: 8,
        names: [("G#", true), ("Ab", true), ("Ab", true)]
    },
    PitchClass {
        value: 9,
        names: [("A", true), ("Gx", false), ("Bbb", false)]
    },
    PitchClass {
        value: 10,
        names: [("A#", true), ("Bb", true), ("Cbb", false)]
    },
    PitchClass {
        value: 11,
        names: [("B", true), ("Ax", false), ("Cb", false)]
    }
];

const LETTERS: [&'static str; 7] = ["A", "B", "C", "D", "E", "F", "G"];

pub fn get_letter_at_increment(letter: String, increment: i8) -> String {
    for letter_index in 0..7 as i8 {
        let current_letter = LETTERS[letter_index as usize];
        if current_letter == letter {
            let new_index = (letter_index + increment) % 7;
            return String::from(LETTERS[new_index as usize]);
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

pub fn get_pitch_class_at_increment(current_pitch_class: PitchClass, increment: i8) -> &'static PitchClass {
    let current_pitch_value = current_pitch_class.value as i8;
    let next_pitch_value = (current_pitch_value + increment) % 12;
    return &PITCH_CLASSES[next_pitch_value as usize];
}