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
  - Multiple notes can be played at the same time.
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
- Functional harmony
  - You can get the diatonic chords of any key and mode
  - You can get a diatonic chord given a roman numeral
- The track module allows you to combine notes with a rhythm and play them
  - Supports chords and rests
  - Supports time signatures and tempos
- Notes return their frequencies based on a given base frequency
  - This allows you to set A4 equal to any frequency (such as 432 Hz)
- The midi module allows you to import from and export to MIDI files
- There is also support to convert a track into a GRUB bootloader tune

## Limitations

This library only works for the twelve tone equal temperament system which is commonly used in western music.