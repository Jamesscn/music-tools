/// The common submodule contains common structures and traits used by the other submodules.
pub mod common;

/// The player submodule contains a structure for playing and rendering audio.
pub mod player;

/// The processor submodule contains a structure for processing frequencies and generating an audio
/// signal that can be used by other modules.
pub mod processor;

/// The wavetable submodule contains a wavetable oscillator synthesizer that can be used by the
/// audio processor.
pub mod wavetable;
