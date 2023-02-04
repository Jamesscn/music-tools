use std::cmp;
use musictools::note::Note;
use musictools::chord::Chord;
use musictools::pitchclass::PitchClasses;
use musictools::common::TriadQuality;

#[test]
fn test_minor_triads() {
    let a_minor = Chord::from_triad(PitchClasses::A, TriadQuality::Minor).to_notes(4);
    let a_flat_minor = Chord::from_triad(PitchClasses::A_FLAT, TriadQuality::Minor).to_notes(1);
    let a_sharp_minor = Chord::from_triad(PitchClasses::A_SHARP, TriadQuality::Minor).to_notes(1);
    let c_minor = Chord::from_triad(PitchClasses::C, TriadQuality::Minor).to_notes(0);
    let test_cases = [
        (a_minor, ["A4", "C5", "E5"]),
        (a_flat_minor, ["Ab1", "B1", "Eb2"]),
        (a_sharp_minor, ["A#1", "C#2", "F2"]),
        (c_minor, ["C0", "Eb0", "G0"])
    ];
    for test_case in test_cases {
        let output_vec: Vec<Note> = test_case.0;
        let expected_vec: Vec<Note> = test_case.1.into_iter().map(|x| Note::from_string(x).unwrap()).collect();
        assert_eq!(output_vec.len(), expected_vec.len());
        for index in 0..cmp::min(output_vec.len(), expected_vec.len()) {
            assert_eq!(output_vec[index].get_octave(), expected_vec[index].get_octave());
            assert_eq!(output_vec[index].get_pitch_class(), expected_vec[index].get_pitch_class());
            assert_eq!(output_vec[index].get_keyboard_index(), expected_vec[index].get_keyboard_index());
            assert_eq!(output_vec[index].get_frequency(), expected_vec[index].get_frequency());
        }
    }
}