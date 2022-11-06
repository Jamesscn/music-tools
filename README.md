# Music Tools

A set of tools related to music theory.

## Installation

1. Download/clone this repository
2. Install [rust and cargo](https://doc.rust-lang.org/book/ch01-01-installation.html)
3. Run the code with the `cargo run` command

## What has been implemented so far:

- The pitchclass module contains all of the pitch classes from A to G including accidentals as structs.
- The note module contains the note struct which allows you to specify a pitch class and octave (e.g. A4)
  - This struct also calculates the frequency of the note.
- The audio module contains a wavetable oscillator which allows one to play certain frequencies or notes as sine waves.
  - Currently only sine waves are supported.
- The scale module can make the following scales:
  - Chromatic scales
  - Diatonic scales
    - Scales for the seven modes
  - Pentatonic scales
  - A few other scales
- The chord module can return the following types of triads/tetrads
  - Inversions of chords
  - Tetrads of triads with sevenths
  - Major and minor triads
  - sus2 and sus4 triads
  - Augmented and diminished triads
  - The short name of the triad, e.g. C#/Em(maj7)
- Functional harmony
  - You can get the diatonic chords of any key and mode
  - You can get a diatonic chord given a roman numeral
- The rhythm module allows you to make rhythms
  - Supports fractional beats

## Things that will be implemented in the future:

- Getting intervals between notes
- Playing multiple notes at the same time