use crate::note::Note;
use std::fmt;

/// A struct representing a MIDI or track event.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Event {
    note: Note,
    active: bool,
    delta_ticks: u64,
}

impl Event {
    /// Creates a new MIDI event.
    ///
    /// # Parameters
    ///
    /// - `note`: A [`Note`] representing the note affected by the MIDI event.
    /// - `active`: Whether the [`Note`] should be turned on or off.
    /// - `delta_ticks`: The amount of MIDI ticks since the last event.
    pub fn new(note: Note, active: bool, delta_ticks: u64) -> Self {
        Self {
            note,
            active,
            delta_ticks,
        }
    }

    /// Returns the [`Note`] associated with the current event.
    pub fn get_note(&self) -> Note {
        self.note
    }

    /// Returns true if the event activates the current note, or false if it deactivates it.
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Returns the amount of MIDI ticks between the last event and the current event.
    pub fn get_delta_ticks(&self) -> u64 {
        self.delta_ticks
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let state = match self.active {
            true => "on",
            false => "off",
        };
        write!(
            f,
            "MIDI event: turn {} {} after {} ticks",
            self.note, state, self.delta_ticks
        )
    }
}
