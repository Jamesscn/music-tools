use crate::note::Note;
use crate::chord::Chord;
use crate::interval::Interval;
use crate::pitchclass::PitchClass;
use crate::common::{ScaleType, PentatonicType};

pub struct Scale {
    intervals: Vec<Interval>,
    scale: ScaleType,
    pentatonic: PentatonicType
}

impl Scale {
    pub fn from(scale: ScaleType, pentatonic: PentatonicType) -> Option<Scale> {
        let scale_steps: Vec<u8> = match scale {
            ScaleType::Major | ScaleType::Ionian => vec![2, 2, 1, 2, 2, 2, 1],
            ScaleType::Minor | ScaleType::Aeolian | ScaleType::NaturalMinor | ScaleType::DescendingMelodicMinor => vec![2, 1, 2, 2, 1, 2, 2],
            ScaleType::Dorian => vec![2, 1, 2, 2, 2, 1, 2],
            ScaleType::Phrygian => vec![1, 2, 2, 2, 1, 2, 2],
            ScaleType::Lydian => vec![2, 2, 2, 1, 2, 2, 1],
            ScaleType::Mixolydian => vec![2, 2, 1, 2, 2, 1, 2],
            ScaleType::Locrian => vec![1, 2, 2, 1, 2, 2, 2],
            ScaleType::HarmonicMinor => vec![2, 1, 2, 2, 1, 3, 1],
            ScaleType::AscendingMelodicMinor => vec![2, 1, 2, 2, 2, 2, 1],
            ScaleType::PhrygianDominant => vec![1, 3, 1, 2, 1, 2, 2],
            ScaleType::Whole => vec![2, 2, 2, 2, 2, 2],
            ScaleType::Chromatic => vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]
        };
        let mut intervals: Vec<Interval> = Vec::new();
        let mut interval_value = 0;
        for step in scale_steps {
            intervals.push(Interval::from(interval_value));
            interval_value += step;
        }
        intervals.push(Interval::from(interval_value));
        if pentatonic != PentatonicType::None && intervals.len() != 8 {
            return None;
        }
        if pentatonic == PentatonicType::Major {
            intervals.remove(6);
            intervals.remove(3);
        } else if pentatonic == PentatonicType::Minor {
            intervals.remove(5);
            intervals.remove(1);
        }
        return Some(Scale {
            intervals,
            scale,
            pentatonic
        });
    }

    pub fn get_intervals(&self) -> Vec<Interval> {
        return self.intervals.clone();
    }

    pub fn get_scale_type(&self) -> ScaleType {
        return self.scale;
    }

    pub fn get_pentatonic_type(&self) -> PentatonicType {
        return self.pentatonic;
    }

    pub fn is_diatonic(&self) -> bool {
        if self.intervals.len() == 8 {
            return true;
        }
        return false;
    }

    pub fn is_pentatonic(&self) -> bool {
        if self.intervals.len() == 6 {
            return true;
        }
        return false;
    }

    pub fn get_diatonic_chords(&self, tonic: PitchClass, with_seventh: bool) -> Option<Vec<Chord>> {
        let minor_numerals: [&str; 7];
        let ionian_numerals: [&str; 7];
        let dorian_numerals: [&str; 7];
        let phrygian_numerals: [&str; 7];
        let lydian_numerals: [&str; 7];
        let mixolydian_numerals: [&str; 7];
        let aeolian_numerals: [&str; 7];
        let locrian_numerals: [&str; 7];
        if with_seventh {
            minor_numerals = ["i7", "ii°7", "bIIImaj7", "iv7", "Vmaj7", "bVImaj7", "bVII7"];
            ionian_numerals = ["Imaj7", "ii7", "iii7", "IVmaj7", "V7", "vi7", "vii°7"];
            dorian_numerals = ["i7", "ii7", "bIIImaj7", "IV7", "v7", "vi°7", "bVIImaj7"];
            phrygian_numerals = ["i7", "bIImaj7", "bIII7", "iv7", "v°7", "bVImaj7", "bvii7"];
            lydian_numerals = ["Imaj7", "II7", "iii7", "#iv°7", "Vmaj7", "vi7", "vii7"];
            mixolydian_numerals = ["I7", "ii7", "iii°7", "IVmaj7", "v7", "vi7", "bVIImaj7"];
            aeolian_numerals = ["i7", "ii°7", "bIIImaj7", "iv7", "v7", "bVImaj7", "bVII7"];
            locrian_numerals = ["i°7", "bIImaj7", "biii7", "iv7", "bVmaj7", "bVI7", "bvii7"];
        } else {
            minor_numerals = ["i", "ii°", "bIII", "iv", "V", "bVI", "bVII"];
            ionian_numerals = ["I", "ii", "iii", "IV", "V", "vi", "vii°"];
            dorian_numerals = ["i", "ii", "bIII", "IV", "v", "vi°", "bVII"];
            phrygian_numerals = ["i", "bII", "bIII", "iv", "v°", "bVI", "bvii"];
            lydian_numerals = ["I", "II", "iii", "#iv°", "V", "vi", "vii"];
            mixolydian_numerals = ["I", "ii", "iii°", "IV", "v", "vi", "bVII"];
            aeolian_numerals = ["i", "ii°", "bIII", "iv", "v", "bVI", "bVII"];
            locrian_numerals = ["i°", "bII", "biii", "iv", "bV", "bVI", "bvii"];
        }
        let chords: Vec<Chord> = match self.scale {
            ScaleType::Minor => minor_numerals,
            ScaleType::Major | ScaleType::Ionian => ionian_numerals,
            ScaleType::Dorian => dorian_numerals,
            ScaleType::Phrygian => phrygian_numerals,
            ScaleType::Lydian => lydian_numerals,
            ScaleType::Mixolydian => mixolydian_numerals,
            ScaleType::Aeolian | ScaleType::NaturalMinor => aeolian_numerals,
            ScaleType::Locrian => locrian_numerals,
            _ => return None
        }.iter().map(|x| Chord::from_numeral(tonic, x).unwrap()).collect();
        return Some(chords);
    }

    pub fn to_chord(&self, tonic: PitchClass) -> Chord {
        let mut chord = Chord::new(tonic);
        for index in 1..self.intervals.len() {
            chord.add_interval(self.intervals[index]);
        }
        return chord;
    }

    pub fn to_notes(&self, tonic: PitchClass, starting_octave: u8) -> Vec<Note> {
        return self.to_chord(tonic).to_notes(starting_octave);
    }

    pub fn to_pitch_classes(&self, tonic: PitchClass) -> Vec<PitchClass> {
        return self.to_chord(tonic).get_pitch_classes();
    }
}

impl PartialEq for Scale {
    fn eq(&self, other: &Self) -> bool {
        return self.scale == other.scale && self.pentatonic == other.pentatonic;
    }
}