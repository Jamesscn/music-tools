use std::cmp;
use musictools::note::Note;
use musictools::chord::Chord;
use musictools::pitchclass::PitchClasses;
use musictools::common::TriadQuality;

#[test]
fn test_triads() {
    //Major Triads
    let c_major = Chord::from_triad(TriadQuality::Major).to_notes(PitchClasses::C, 0);
    let e_major = Chord::from_triad(TriadQuality::Major).to_notes(PitchClasses::E, 1);
    let f_major = Chord::from_triad(TriadQuality::Major).to_notes(PitchClasses::F, 7);
    let f_sharp_major = Chord::from_triad(TriadQuality::Major).to_notes(PitchClasses::F_SHARP, 8);

    // Minor Triads
    let a_minor = Chord::from_triad(TriadQuality::Minor).to_notes(PitchClasses::A, 4);
    let a_flat_minor = Chord::from_triad(TriadQuality::Minor).to_notes(PitchClasses::A_FLAT, 1);
    let a_sharp_minor = Chord::from_triad(TriadQuality::Minor).to_notes(PitchClasses::A_SHARP, 1);
    let c_minor = Chord::from_triad(TriadQuality::Minor).to_notes(PitchClasses::C, 0);
    
    //Augmented Triads
    let b_augmented = Chord::from_triad(TriadQuality::Augmented).to_notes(PitchClasses::B, 5);
    let e_augmented = Chord::from_triad(TriadQuality::Augmented).to_notes(PitchClasses::E, 7);
    
    //Diminished Triads
    let d_diminished = Chord::from_triad(TriadQuality::Diminished).to_notes(PitchClasses::D, 3);
    let g_sharp_diminished = Chord::from_triad(TriadQuality::Diminished).to_notes(PitchClasses::G_SHARP, 3);

    //Sus2 Triads
    let g_sus2 = Chord::from_triad(TriadQuality::Sus2).to_notes(PitchClasses::G, 0);
    let g_flat_sus2 = Chord::from_triad(TriadQuality::Sus2).to_notes(PitchClasses::G_FLAT, 1);
    
    //Sus4 Triads
    let a_sharp_sus4 = Chord::from_triad(TriadQuality::Sus4).to_notes(PitchClasses::A_SHARP, 7);
    let f_sus4 = Chord::from_triad(TriadQuality::Sus4).to_notes(PitchClasses::F, 8);
    
    let test_cases = [
        (c_major, ["C0", "E0", "G0"]),
        (e_major, ["E1", "G#1", "B1"]),
        (f_major, ["F7", "A7", "C8"]),
        (f_sharp_major, ["F#8", "A#8", "C#9"]),
        (a_minor, ["A4", "C5", "E5"]),
        (a_flat_minor, ["Ab1", "Cb1", "Eb2"]),
        (a_sharp_minor, ["A#1", "C#2", "E#2"]),
        (c_minor, ["C0", "Eb0", "G0"]),
        (b_augmented, ["B5", "D#6", "F##6"]),
        (e_augmented, ["E7", "G#7", "B#8"]),
        (d_diminished, ["D3", "F3", "Ab3"]),
        (g_sharp_diminished, ["G#3", "B3", "D4"]),
        (g_sus2, ["G0", "A0", "D1"]),
        (g_flat_sus2, ["Gb1", "Ab1", "Db2"]),
        (a_sharp_sus4, ["A#7", "D#8", "E#8"]),
        (f_sus4, ["F8", "Bb8", "C9"])
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