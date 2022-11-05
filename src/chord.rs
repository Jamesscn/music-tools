pub use crate::enums::{ScaleType, Pentatonic};
pub use crate::pitchclass::PitchClass;
pub use crate::scale::get_scale;
pub use crate::enums::{ChordQuality, Seventh};

pub struct Chord {
    pitch_classes: Vec<&'static PitchClass>,
    inversion: u8,
    quality: ChordQuality,
    seventh: Seventh
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

    pub fn get_inversion(&self) -> u8 {
        return self.inversion;
    }

    pub fn get_quality(&self) -> &ChordQuality {
        return &self.quality;
    }

    pub fn get_seventh(&self) -> &Seventh {
        return &self.seventh;
    }
}

pub fn get_chord_with_quality(tonic: &'static PitchClass, quality: ChordQuality, seventh: Seventh, inversion: u8) -> Chord {
    let major_scale_obj = get_scale(tonic, ScaleType::Major, Pentatonic::None);
    let minor_scale_obj = get_scale(tonic, ScaleType::Minor, Pentatonic::None);
    let whole_scale_obj = get_scale(tonic, ScaleType::Whole, Pentatonic::None);
    let locrian_scale_obj = get_scale(tonic, ScaleType::Locrian, Pentatonic::None);
    let major_scale = major_scale_obj.get_pitch_classes();
    let minor_scale = minor_scale_obj.get_pitch_classes();
    let whole_scale = whole_scale_obj.get_pitch_classes();
    let locrian_scale = locrian_scale_obj.get_pitch_classes();
    let mut pitch_classes: Vec<&'static PitchClass> = match quality {
        ChordQuality::Major =>  Vec::from([major_scale[0], major_scale[2], major_scale[4]]),
        ChordQuality::Minor =>  Vec::from([minor_scale[0], minor_scale[2], minor_scale[4]]),
        ChordQuality::Sus2 =>  Vec::from([major_scale[0], major_scale[1], major_scale[4]]),
        ChordQuality::Sus4 =>  Vec::from([major_scale[0], major_scale[3], major_scale[4]]),
        ChordQuality::Augmented =>  Vec::from([whole_scale[0], whole_scale[2], whole_scale[4]]),
        ChordQuality::Diminished =>  Vec::from([locrian_scale[0], locrian_scale[2], locrian_scale[4]])
    };
    if seventh == Seventh::Major {
        pitch_classes.push(major_scale[6]);
    } else if seventh == Seventh::Minor {
        pitch_classes.push(minor_scale[6]);
    }
    let num_pitch_classes = pitch_classes.len() as u8;
    return Chord {
        pitch_classes: pitch_classes,
        inversion: inversion % num_pitch_classes,
        quality: quality,
        seventh: seventh
    }
}