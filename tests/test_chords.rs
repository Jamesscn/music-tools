use music_tools::audio::common::Playable;
use music_tools::chord::Chord;
use music_tools::common::{EqualTemperament, TriadQuality};
use music_tools::note::Note;
use std::cmp;

#[test]
fn test_triads() {
    let test_cases = [
        (
            Chord::from_triad(TriadQuality::Major).set_base_note("C0"),
            ["C0", "E0", "G0"],
        ),
        (
            Chord::from_triad(TriadQuality::Major).set_base_note("E1"),
            ["E1", "G#1", "B1"],
        ),
        (
            Chord::from_triad(TriadQuality::Major).set_base_note("F7"),
            ["F7", "A7", "C8"],
        ),
        (
            Chord::from_triad(TriadQuality::Major).set_base_note("F#8"),
            ["F#8", "A#8", "C#9"],
        ),
        (
            Chord::from_triad(TriadQuality::Minor).set_base_note("A4"),
            ["A4", "C5", "E5"],
        ),
        (
            Chord::from_triad(TriadQuality::Minor).set_base_note("Ab1"),
            ["Ab1", "Cb1", "Eb2"],
        ),
        (
            Chord::from_triad(TriadQuality::Minor).set_base_note("A#1"),
            ["A#1", "C#2", "E#2"],
        ),
        (
            Chord::from_triad(TriadQuality::Minor).set_base_note("C0"),
            ["C0", "Eb0", "G0"],
        ),
        (
            Chord::from_triad(TriadQuality::Augmented).set_base_note("B5"),
            ["B5", "D#6", "F##6"],
        ),
        (
            Chord::from_triad(TriadQuality::Augmented).set_base_note("E7"),
            ["E7", "G#7", "B#8"],
        ),
        (
            Chord::from_triad(TriadQuality::Diminished).set_base_note("D3"),
            ["D3", "F3", "Ab3"],
        ),
        (
            Chord::from_triad(TriadQuality::Diminished).set_base_note("G#3"),
            ["G#3", "B3", "D4"],
        ),
        (
            Chord::from_triad(TriadQuality::Sus2).set_base_note("G0"),
            ["G0", "A0", "D1"],
        ),
        (
            Chord::from_triad(TriadQuality::Sus2).set_base_note("Gb1"),
            ["Gb1", "Ab1", "Db2"],
        ),
        (
            Chord::from_triad(TriadQuality::Sus4).set_base_note("A#7"),
            ["A#7", "D#8", "E#8"],
        ),
        (
            Chord::from_triad(TriadQuality::Sus4).set_base_note("F8"),
            ["F8", "Bb8", "C9"],
        ),
    ];

    let tuning = EqualTemperament::new();
    for test_case in test_cases {
        let output_vec: Vec<Note> = test_case.0.unwrap().get_notes();
        let expected_vec: Vec<Note> = test_case
            .1
            .into_iter()
            .map(|x| Note::from_string(x).unwrap())
            .collect();
        assert_eq!(output_vec.len(), expected_vec.len());
        for index in 0..cmp::min(output_vec.len(), expected_vec.len()) {
            assert_eq!(
                output_vec[index].get_octave(),
                expected_vec[index].get_octave()
            );
            assert_eq!(
                output_vec[index].get_pitch_class(),
                expected_vec[index].get_pitch_class()
            );
            assert_eq!(
                output_vec[index].get_keyboard_index().ok(),
                expected_vec[index].get_keyboard_index().ok()
            );
            assert_eq!(
                output_vec[index].get_frequencies(&tuning, 440f32),
                expected_vec[index].get_frequencies(&tuning, 440f32)
            );
        }
    }
}
