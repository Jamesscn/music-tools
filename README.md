# Music Tools

A library with tools for analysis and generation of music.

**NOTE**: As of the current moment, this library is in a prerelease state, meaning that major breaking changes will occur. Once these changes are implemented in a flexible and intuitive manner, and there is a clear vision for the package then a proper release will be made and the library will be uploaded to crates.io.

## Installation

1. Download/clone this repository
2. Install [rust and cargo](https://doc.rust-lang.org/book/ch01-01-installation.html)
3. Run the code with the `cargo run` command

## What has been implemented so far:

**Structures**

- Pitch classes from A to G including accidentals
- Notes (pitch classes + octaves)
  - You can calculate their frequencies and change the base frequency to any arbitrary value (440Hz, 432Hz, etc.)
- Chords which can be defined by:
  - Sets of intervals (major tetrad, augmented seventh chord, etc.) with no particular pitch classes or notes
  - Groups of pitch classes (for example the C major triad which is C, E and G)
  - Groups of notes (The F♯8 major triad would be F♯8, A♯8 and C♯9)
  - Inversions of the above chords and the possibility to add higher intervals such as sevenths, ninths and crazier stuff
- Scales:
  - All seven major modes
  - All seven harmonic minor modes
  - All seven melodic minor modes
  - Other scales such as the whole scale, the major blues scales and more
  - Pentatonic major and minor modifiers
- Functional harmony:
  - You can obtain the diatonic chords of any of the seven major modes
  - You can obtain chords from numeral strings

**MIDI**

This library uses the [apres](https://crates.io/crates/apres) library to parse MIDI files into Note structures. As of the current moment MIDI support is not perfect and the code needs to be improved to be easier to use, but it is good enough to be playable in most cases.

**Modularity**

You can convert most structures into other structures, for example a Chord into a Vec&lt;Note&gt;, or a Scale into a Vec&lt;Interval&gt;. In some cases information can be lost, like converting a Chord into a Vec&lt;PitchClass&gt; (octaves are lost) however you can still do this in most cases.

Optional crate features:

- Audio module: By default this feature is disabled, however by enabling it you gain access to a set of structures and submodules that allow you to process audio waves and play them. With the audio player you can also export audio into WAV files.

## Limitations

This library only works for the twelve tone equal temperament system which is commonly used in western music.