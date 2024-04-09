use crate::common::{Beat, Fraction};
use crate::note::Note;
use std::fmt;

pub type Ticks = u64;

/// An enum representing a MIDI event.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum MIDIEvent {
    NoteOn(Note),
    NoteOff(Note),
    SetTempo(u32),
    SetTimeSignature(Fraction),
}

impl fmt::Display for MIDIEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MIDIEvent::NoteOn(note) => write!(f, "MIDI event: turn on {}", note),
            MIDIEvent::NoteOff(note) => write!(f, "MIDI event: turn off {}", note),
            MIDIEvent::SetTempo(tempo) => write!(f, "MIDI event: set tempo to {}", tempo),
            MIDIEvent::SetTimeSignature(time_signature) => {
                write!(f, "MIDI event: set time signature to {}", time_signature)
            }
        }
    }
}

pub fn beat_to_ticks(beat: Beat, ticks_per_quarter_note: Ticks) -> Ticks {
    (4 * ticks_per_quarter_note * beat.get_numerator() as Ticks) / beat.get_denominator() as Ticks
}

pub fn ticks_to_beat(ticks: Ticks, ticks_per_quarter_note: Ticks) -> Beat {
    Beat::new(ticks, 4 * ticks_per_quarter_note).get_simplified()
}
