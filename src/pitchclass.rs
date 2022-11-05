pub struct PitchClass {
    value: u8,
    names: [&'static str; 3]
}

impl PitchClass {
    pub fn get_value(&self) -> u8 {
        return self.value;
    }

    pub fn get_name(&self) -> &'static str {
        return self.names[0];
    }

    pub fn get_all_names(&self) -> Vec<&'static str> {
        let mut names: Vec<&'static str> = Vec::from(self.names);
        if names[2] == "Ab" {
            names.remove(2);
        }
        return names;
    }
}

const PITCH_CLASSES: [PitchClass; 12] = [
    PitchClass {
        value: 0,
        names: ["C", "B#", "Dbb"]
    },
    PitchClass {
        value: 1,
        names: ["C#", "Db", "Bx"]
    },
    PitchClass {
        value: 2,
        names: ["D", "Ebb", "Cx"]
    },
    PitchClass {
        value: 3,
        names: ["D#", "Eb", "Fbb"]
    },
    PitchClass {
        value: 4,
        names: ["E", "Fb", "Dx"]
    },
    PitchClass {
        value: 5,
        names: ["F", "E#", "Gbb"]
    },
    PitchClass {
        value: 6,
        names: ["F#", "Gb", "Ex"]
    },
    PitchClass {
        value: 7,
        names: ["G", "Abb", "Fx"]
    },
    PitchClass {
        value: 8,
        names: ["G#", "Ab", "Ab"]
    },
    PitchClass {
        value: 9,
        names: ["A", "Bbb", "Gx"]
    },
    PitchClass {
        value: 10,
        names: ["A#", "Bb", "Cbb"]
    },
    PitchClass {
        value: 11,
        names: ["B", "Cb", "Ax"]
    }
];

pub struct PitchClasses;

impl PitchClasses {
    pub const C: &PitchClass = &PITCH_CLASSES[0];
    pub const C_SHARP: &PitchClass = &PITCH_CLASSES[1];
    pub const D_FLAT: &PitchClass = &PITCH_CLASSES[1];
    pub const D: &PitchClass = &PITCH_CLASSES[2];
    pub const D_SHARP: &PitchClass = &PITCH_CLASSES[3];
    pub const E_FLAT: &PitchClass = &PITCH_CLASSES[3];
    pub const E: &PitchClass = &PITCH_CLASSES[4];
    pub const F: &PitchClass = &PITCH_CLASSES[5];
    pub const F_SHARP: &PitchClass = &PITCH_CLASSES[6];
    pub const G_FLAT: &PitchClass = &PITCH_CLASSES[6];
    pub const G: &PitchClass = &PITCH_CLASSES[7];
    pub const G_SHARP: &PitchClass = &PITCH_CLASSES[8];
    pub const A_FLAT: &PitchClass = &PITCH_CLASSES[8];
    pub const A: &PitchClass = &PITCH_CLASSES[9];
    pub const A_SHARP: &PitchClass = &PITCH_CLASSES[10];
    pub const B_FLAT: &PitchClass = &PITCH_CLASSES[10];
    pub const B: &PitchClass = &PITCH_CLASSES[11];
}

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

pub fn get_pitch_class_from_name(pitch_class_name: String) -> &'static PitchClass {
    for pitch_class_index in 0..12 {
        let current_pitch_class = &PITCH_CLASSES[pitch_class_index];
        for current_name in current_pitch_class.names {
            if current_name == pitch_class_name {
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