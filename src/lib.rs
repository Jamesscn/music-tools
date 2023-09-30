#![warn(missing_docs)]
#![feature(int_roundings)]
#![feature(iter_map_windows)]

//! This library contains modules which can be used to create, analyze and reproduce musical
//! structures such as chords, scales and rhythms.

/// The common module contains common structures, enums and functions that are used by other
/// modules.
pub mod common;

/// The note module contains a structure that can be used to represent notes with a pitch class and
/// a value.
pub mod note;

/// The audio module contains structures for playing frequencies and music.
pub mod audio;

/// The chord module contains a structure which can be used to represent a chord.
pub mod chord;

/// The scale module contains a structure which can be used to represent a scale.
pub mod scale;

/// The interval module contains a structure which can be used to represent an interval.
pub mod interval;

/// The pitchclass module contains a structure which can be used to represent a pitch class.
pub mod pitchclass;

/// The midi module contains a structure which can be used to import and export tracks to MIDI
/// files.
pub mod midi;

/// The track module contains a structure which can be used to represent a MIDI file track.
pub mod track;
