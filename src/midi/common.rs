use super::track::{Track, TrackItem};
use crate::common::{Beat, Fraction};
use crate::note::Note;
use std::collections::VecDeque;
use std::fmt;

pub type Ticks = u64;

/// An enum representing a MIDI event.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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

pub fn iter_track_items<'a>(tracks: impl IntoIterator<Item = &'a Track> + 'a) -> TrackItemIterator {
    TrackItemIterator {
        item_queues: tracks
            .into_iter()
            .map(|track| track.into_iter().copied().collect::<VecDeque<TrackItem>>())
            .collect(),
    }
}

pub struct TrackItemIterator {
    item_queues: Vec<VecDeque<TrackItem>>,
}

impl Iterator for TrackItemIterator {
    type Item = (usize, TrackItem);

    fn next(&mut self) -> Option<Self::Item> {
        let mut next_track_index: Option<usize> = None;
        let mut min_wait_beats: Option<Beat> = None;
        for (track_index, track) in self.item_queues.iter().enumerate() {
            if let Some(track_item) = track.front() {
                match track_item {
                    TrackItem::Event(_) => {
                        next_track_index = Some(track_index);
                        break;
                    }
                    TrackItem::Rest(beat) => {
                        if beat.get_as_float() > 0.0 {
                            let mut update_min_ticks = true;
                            if let Some(value) = min_wait_beats {
                                if value <= *beat {
                                    update_min_ticks = false;
                                }
                            }
                            if update_min_ticks {
                                next_track_index = Some(track_index);
                                min_wait_beats = Some(*beat);
                            }
                        }
                    }
                }
            }
        }
        if let Some(wait_beats) = min_wait_beats {
            let next_track_index = next_track_index.unwrap();
            for (track_index, track) in self.item_queues.iter_mut().enumerate() {
                if track_index == next_track_index {
                    continue;
                }
                if let Some(TrackItem::Rest(beat)) = track.front_mut() {
                    if wait_beats < *beat {
                        *beat -= wait_beats;
                    } else {
                        track.pop_front();
                    }
                }
            }
        };
        next_track_index.map(|track_index| {
            (
                track_index,
                self.item_queues[track_index].pop_front().unwrap(),
            )
        })
    }
}
