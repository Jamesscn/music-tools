use music_tools::{interval::Interval, pitchclass::TwelveTone};

fn test_interval(interval: Interval, expected: &str) {
    assert_eq!(
        TwelveTone::C()
            .add_interval(interval)
            .map(|p| p.to_string()),
        Some(expected.to_string())
    );
}

#[test]
fn test_twelve_tone() {
    test_interval(Interval::PERFECT_UNISON(), "C");
    test_interval(Interval::DIMINISHED_SECOND(), "D♭♭");
    test_interval(Interval::CHROMATIC_SEMITONE(), "C♯");
    test_interval(Interval::MINOR_SECOND(), "D♭");
    test_interval(Interval::WHOLE_TONE(), "D");
    test_interval(Interval::DIMINISHED_THIRD(), "E♭♭");
    test_interval(Interval::AUGMENTED_SECOND(), "D♯");
    test_interval(Interval::MINOR_THIRD(), "E♭");
    test_interval(Interval::MAJOR_THIRD(), "E");
    test_interval(Interval::DIMINISHED_FOURTH(), "F♭");
    test_interval(Interval::AUGMENTED_THIRD(), "E♯");
    test_interval(Interval::PERFECT_FOURTH(), "F");
    test_interval(Interval::AUGMENTED_FOURTH(), "F♯");
    test_interval(Interval::DIMINISHED_FIFTH(), "G♭");
    test_interval(Interval::PERFECT_FIFTH(), "G");
    test_interval(Interval::DIMINISHED_SIXTH(), "A♭♭");
    test_interval(Interval::AUGMENTED_FIFTH(), "G♯");
    test_interval(Interval::MINOR_SIXTH(), "A♭");
    test_interval(Interval::MAJOR_SIXTH(), "A");
    test_interval(Interval::DIMINISHED_SEVENTH(), "B♭♭");
    test_interval(Interval::AUGMENTED_SIXTH(), "A♯");
    test_interval(Interval::MINOR_SEVENTH(), "B♭");
    test_interval(Interval::MAJOR_SEVENTH(), "B");
    test_interval(Interval::DIMINISHED_OCTAVE(), "C♭");
    test_interval(Interval::AUGMENTED_SEVENTH(), "B♯");
    test_interval(Interval::PERFECT_OCTAVE(), "C");
    assert_eq!(Interval::PERFECT_OCTAVE().to_string(), "PERFECT OCTAVE")
}
