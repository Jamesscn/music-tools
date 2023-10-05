## 0.2.0 (2023-10-05)

### Feat

- forgot to add function to convert scale to interval
- queue system and export wav
- more conversion functions
- convert chord to vectors of other types
- add standard traits to structs
- modal madness
- add melodic minor mode and diminished modes
- create tochord trait to easily create chords
- allow playing a midi file at a different tempo
- customizable volume and better volume leveling
- arpeggios, simplify play functions and allow playing generics
- Return a Note at an offset

### Fix

- add setvolume to player and clamp volume between 0 and 1
- update preexisting documentation and small linting changes
- Bug fixes related to rests and tracks
- change unwrap to expect in examples

### Refactor

- remove rand crate
- make the audio module optional pt2
- make the audio module optional
- allow references to arrays instead of just vecs in functions
- major refactor of audio player
- use saw wave for midi_in example
- make interfacing with pitchclass less confusing and add from_str functions
- prefer from trait over custom trait
- create audioplayer structure to play audio
- use results instead of options and create custom error types

## 0.1.0 (2023-04-07)
