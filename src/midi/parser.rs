use super::common::{beat_to_ticks, ticks_to_beat, MIDIEvent, Ticks};
use super::track::{Track, TrackItem};
use crate::common::{Beat, Fraction, InputError};
use crate::note::Note;
use apres::MIDIEvent as Apres_MIDIEvent;
use apres::MIDI as Apres_MIDI;
use std::collections::VecDeque;
use std::path::Path;
use std::slice::Iter;
use std::time::Duration;

/// A structure which holds a MIDI object that can be imported from or exported to a MIDI file,
/// containing a set of [`Track`] objects.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MIDI {
    ticks_per_quarter_note: Ticks,
    tracks: Vec<Track>,
}

impl MIDI {
    /// Creates an empty MIDI file with no tracks
    pub fn new() -> Self {
        Self {
            ticks_per_quarter_note: 360,
            tracks: Vec::new(),
        }
    }

    /// Imports a MIDI object from a MIDI file. The return value is a [`Result`] which can be either
    /// a [`MIDI`] or an [`InputError`] if the MIDI file provided does not exist or is invalid.
    ///
    /// # Parameters
    ///
    /// - `file_path`: The path to the MIDI file to import.
    pub fn import(file_path: impl AsRef<Path>) -> Result<Self, InputError> {
        let str_path = match file_path.as_ref().to_str() {
            Some(path) => path,
            None => {
                return Err(InputError {
                    message: String::from("the file path must be a valid unicode string"),
                })
            }
        };
        let midi_object = match Apres_MIDI::from_path(str_path) {
            Ok(apres_midi_object) => apres_midi_object,
            Err(_) => {
                return Err(InputError {
                    message: String::from(
                        "the path provided does not exist or the midi file was invalid",
                    ),
                })
            }
        };
        let ticks_per_quarter_note = midi_object.get_ppqn() as Ticks;
        let midi_tracks = midi_object.get_tracks();
        let mut tracks: Vec<Track> = Vec::new();
        for midi_track_info in midi_tracks {
            let mut track = Track::new();
            for (delta_ticks, event_id) in midi_track_info {
                let event = match midi_object.get_event(event_id) {
                    Some(event_object) => event_object,
                    None => continue,
                };
                track.push_rest(ticks_to_beat(delta_ticks as Ticks, ticks_per_quarter_note));
                match event {
                    Apres_MIDIEvent::NoteOn(_channel, note_index, velocity) => {
                        if velocity > 0 {
                            track.push_event(MIDIEvent::NoteOn(Note::from_midi_index(note_index)?));
                        } else {
                            track
                                .push_event(MIDIEvent::NoteOff(Note::from_midi_index(note_index)?));
                        }
                    }
                    Apres_MIDIEvent::NoteOff(_channel, note_index, _velocity) => {
                        track.push_event(MIDIEvent::NoteOff(Note::from_midi_index(note_index)?));
                    }
                    Apres_MIDIEvent::SetTempo(us_per_quarter_note) => {
                        track.push_event(MIDIEvent::SetTempo(60000000 / us_per_quarter_note));
                    }
                    Apres_MIDIEvent::TimeSignature(
                        numerator,
                        denominator,
                        _clocks_per_metronome,
                        _thirtysecondths_per_quarter,
                    ) => {
                        track.push_event(MIDIEvent::SetTimeSignature(Fraction::new(
                            numerator as u64,
                            u64::pow(2, denominator as u32),
                        )));
                    }
                    _ => {}
                }
            }
            if !track.is_empty() {
                tracks.push(track);
            }
        }
        Ok(Self {
            ticks_per_quarter_note,
            tracks,
        })
    }

    /// Exports a MIDI object to a MIDI file. The function returns a [`Result`] which can be an
    /// [`InputError`] if the MIDI file could not be saved. Unfortunately the apres library does
    /// not return if the file was successfully saved, so this is something that has to be looked
    /// into in the future.
    ///
    /// # Parameters
    ///
    /// - `file_path`: The path to export the MIDI file to.
    pub fn export(&self, file_path: impl AsRef<Path>) -> Result<(), InputError> {
        let str_path = match file_path.as_ref().to_str() {
            Some(path) => path,
            None => {
                return Err(InputError {
                    message: String::from("the file path must be a valid unicode string"),
                })
            }
        };
        let mut midi_object = Apres_MIDI::new();
        midi_object.set_ppqn(self.ticks_per_quarter_note as u16);
        if self.tracks.is_empty() {
            return Err(InputError {
                message: String::from(
                    "the midi object could not be saved because it has no tracks",
                ),
            });
        }
        for (track_index, track) in self.tracks.iter().enumerate() {
            let mut curr_tick = 0;
            for track_item in track {
                match track_item {
                    TrackItem::Event(event) => {
                        let apres_event = match event {
                            MIDIEvent::NoteOn(note) => {
                                Apres_MIDIEvent::NoteOn(0, note.get_midi_index()?, 100)
                            }
                            MIDIEvent::NoteOff(note) => {
                                Apres_MIDIEvent::NoteOff(0, note.get_midi_index()?, 0)
                            }
                            MIDIEvent::SetTempo(tempo) => {
                                Apres_MIDIEvent::SetTempo((60000000.0 / *tempo as f32) as u32)
                            }
                            MIDIEvent::SetTimeSignature(time_signature) => {
                                let midi_num = time_signature.get_numerator() as u8;
                                let midi_denom =
                                    f64::log2(time_signature.get_denominator() as f64) as u8;
                                Apres_MIDIEvent::TimeSignature(midi_num, midi_denom, 24, 8)
                            }
                        };
                        midi_object.insert_event(track_index, curr_tick, apres_event);
                    }
                    TrackItem::Rest(beat) => {
                        curr_tick += beat_to_ticks(*beat, self.ticks_per_quarter_note) as usize;
                    }
                }
            }
        }
        midi_object.save(str_path); // This function does not indicate if saving was successful!
        Ok(())
    }

    /// Pushes a [`Track`] onto the MIDI object.
    ///
    /// # Parameters
    ///
    /// - `track`: The [`Track`] to push onto to the current MIDI object.
    pub fn push(&mut self, track: Track) {
        self.tracks.push(track);
    }

    pub fn pop(&mut self) -> Option<Track> {
        self.tracks.pop()
    }

    pub fn get_num_tracks(&self) -> usize {
        self.tracks.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tracks.is_empty()
    }

    /// Returns the amount of MIDI ticks in a quarter note.
    pub fn get_ticks_per_quarter_note(&self) -> Ticks {
        self.ticks_per_quarter_note
    }

    pub fn set_ticks_per_quarter_note(&mut self, ticks_per_quarter_note: impl Into<Ticks>) {
        self.ticks_per_quarter_note = ticks_per_quarter_note.into();
    }

    pub fn get_tick_duration(&self, tempo: f32) -> Duration {
        Duration::from_micros((60000000.0 / (tempo * self.ticks_per_quarter_note as f32)) as u64)
    }

    pub fn iter_track_items(&self) -> TrackItemIterator {
        TrackItemIterator {
            item_queues: self
                .tracks
                .iter()
                .map(|track| track.clone().into_iter().collect::<VecDeque<TrackItem>>())
                .collect(),
        }
    }
}

impl Default for MIDI {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> IntoIterator for &'a MIDI {
    type Item = &'a Track;
    type IntoIter = Iter<'a, Track>;

    fn into_iter(self) -> Self::IntoIter {
        self.tracks.iter()
    }
}

impl IntoIterator for MIDI {
    type Item = Track;
    type IntoIter = <Vec<Track> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.tracks.into_iter()
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
                        min_wait_beats = None;
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
