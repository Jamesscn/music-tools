use music_tools::{interval::*, pitchclass::*};

fn test_interval(interval: Interval, expected: &str) {
    assert_eq!(
        C.add_interval(interval).map(|p| p.to_string()),
        Some(expected.to_string())
    );
}

#[test]
fn test_twelve_tone() {
    test_interval(PERFECT_UNISON, "C");
    test_interval(DIMINISHED_SECOND, "D♭♭");
    test_interval(CHROMATIC_SEMITONE, "C♯");
    test_interval(MINOR_SECOND, "D♭");
    test_interval(WHOLE_TONE, "D");
    test_interval(DIMINISHED_THIRD, "E♭♭");
    test_interval(AUGMENTED_SECOND, "D♯");
    test_interval(MINOR_THIRD, "E♭");
    test_interval(MAJOR_THIRD, "E");
    test_interval(DIMINISHED_FOURTH, "F♭");
    test_interval(AUGMENTED_THIRD, "E♯");
    test_interval(PERFECT_FOURTH, "F");
    test_interval(AUGMENTED_FOURTH, "F♯");
    test_interval(DIMINISHED_FIFTH, "G♭");
    test_interval(PERFECT_FIFTH, "G");
    test_interval(DIMINISHED_SIXTH, "A♭♭");
    test_interval(AUGMENTED_FIFTH, "G♯");
    test_interval(MINOR_SIXTH, "A♭");
    test_interval(MAJOR_SIXTH, "A");
    test_interval(DIMINISHED_SEVENTH, "B♭♭");
    test_interval(AUGMENTED_SIXTH, "A♯");
    test_interval(MINOR_SEVENTH, "B♭");
    test_interval(MAJOR_SEVENTH, "B");
    test_interval(DIMINISHED_OCTAVE, "C♭");
    test_interval(AUGMENTED_SEVENTH, "B♯");
    test_interval(PERFECT_OCTAVE, "C");
    assert_eq!(PERFECT_OCTAVE.to_string(), "PERFECT_OCTAVE")
}
