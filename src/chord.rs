pub use crate::pitchclass::PitchClass;
pub use crate::scale::get_scale;

pub struct Chord {
    pub pitch_classes: Vec<&'static PitchClass>,
    pub inversion: u8,
    pub quality: &'static str,
    pub seventh: &'static str
}

impl Chord {
    pub fn set_inversion(&mut self, inversion: u8) {
        self.inversion = inversion % self.pitch_classes.len() as u8;
    }

    pub fn get_pitch_classes(&self) -> Vec<&'static PitchClass> {
        let mut pitch_classes = Vec::from(&self.pitch_classes[self.inversion as usize..]);
        let mut second_half = Vec::from(&self.pitch_classes[..self.inversion as usize]);
        pitch_classes.append(&mut second_half);
        return pitch_classes;
    }
}

pub fn get_chord_with_quality(tonic: &'static PitchClass, quality: &'static str, seventh: &'static str, inversion: u8) -> Chord {
    let major_scale = get_scale(tonic, "major", "none");
    let minor_scale = get_scale(tonic, "minor", "none");
    let whole_scale = get_scale(tonic, "whole", "none");
    let locrian_scale = get_scale(tonic, "locrian", "none");
    let mut pitch_classes: Vec<&'static PitchClass> = match quality {
        "major" =>  Vec::from([major_scale.pitch_classes[0], major_scale.pitch_classes[2], major_scale.pitch_classes[4]]),
        "minor" =>  Vec::from([minor_scale.pitch_classes[0], minor_scale.pitch_classes[2], minor_scale.pitch_classes[4]]),
        "sus2" =>  Vec::from([major_scale.pitch_classes[0], major_scale.pitch_classes[1], major_scale.pitch_classes[4]]),
        "sus4" =>  Vec::from([major_scale.pitch_classes[0], major_scale.pitch_classes[3], major_scale.pitch_classes[4]]),
        "augmented" =>  Vec::from([whole_scale.pitch_classes[0], whole_scale.pitch_classes[2], whole_scale.pitch_classes[4]]),
        "diminished" =>  Vec::from([locrian_scale.pitch_classes[0], locrian_scale.pitch_classes[2], locrian_scale.pitch_classes[4]]),
        _ => panic!("Chord quality/mode {0} is not valid.", quality)
    };
    if seventh == "major" {
        pitch_classes.push(major_scale.pitch_classes[6]);
    } else if seventh == "minor" {
        pitch_classes.push(minor_scale.pitch_classes[6]);
    }
    let num_pitch_classes = pitch_classes.len() as u8;
    return Chord {
        pitch_classes: pitch_classes,
        inversion: inversion % num_pitch_classes,
        quality: quality,
        seventh: seventh
    }
}