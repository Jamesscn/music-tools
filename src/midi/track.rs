use super::common::MIDIEvent;
use crate::common::Beat;
use crate::note::Note;
use std::fmt;
use std::slice::Iter;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum TrackItem {
    Event(MIDIEvent),
    Rest(Beat),
}

impl fmt::Display for TrackItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TrackItem::Event(event) => write!(f, "Track item: {}", event),
            TrackItem::Rest(beat) => write!(f, "Track item: Pause for a {} beat", beat),
        }
    }
}

/// This structure is used to store a track with MIDI data and a sequence of [`TrackItem`], which
/// can either be a MIDI event or a pause between consecutive MIDI events.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Track {
    accumulated_beats: Beat,
    empty: bool,
    items: Vec<TrackItem>,
}

impl Track {
    pub fn new() -> Self {
        Self {
            accumulated_beats: Beat::new(0, 1),
            empty: true,
            items: Vec::new(),
        }
    }

    pub fn push_note<'a>(&mut self, note: impl Into<Note>, duration: Beat) {
        let note = note.into();
        self.push_notes([note], duration);
    }

    pub fn push_notes(&mut self, notes: impl IntoIterator<Item = Note>, duration: Beat) {
        let notes: Vec<Note> = notes.into_iter().collect();
        for note in &notes {
            self.push_event(MIDIEvent::NoteOn(*note));
        }
        self.push_rest(duration);
        for note in &notes {
            self.push_event(MIDIEvent::NoteOff(*note));
        }
    }

    pub fn push_event(&mut self, event: MIDIEvent) {
        if self.accumulated_beats.get_numerator() > 0 {
            self.items.push(TrackItem::Rest(self.accumulated_beats));
            self.accumulated_beats = Beat::new(0, 1);
        }
        self.items.push(TrackItem::Event(event));
        self.empty = false;
    }

    pub fn push_rest(&mut self, beat: Beat) {
        self.accumulated_beats += beat;
    }

    pub fn is_empty(&self) -> bool {
        self.empty
    }
}

impl Default for Track {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> IntoIterator for &'a Track {
    type Item = &'a TrackItem;
    type IntoIter = Iter<'a, TrackItem>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter()
    }
}

impl IntoIterator for Track {
    type Item = TrackItem;
    type IntoIter = <Vec<TrackItem> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}
