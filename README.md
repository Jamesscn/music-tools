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

## Things that will be implemented in the future:

- Getting intervals between notes
- Getting scales
  - Chromatic scales
  - Diatonic scales
  - Pentatonic scales
- Create a struct for triads
  - Major, minor, sus2, sus4, diminished and augmented
  - Allow one to add a seventh note on top
- Functional harmony
  - Get the diatonic chords of any key and mode
  - Get a diatonic chord given a roman numeral
- Playing multiple notes at the same time
- Graphical interface to allow the user to try different sounds