use music_tools::chord::Chord;
use music_tools::common::TriadQuality;
use music_tools::note::Note;
use music_tools::pitchclass::PitchClass;
use std::cmp;

#[test]
fn test_triads() {
    //Major Triads
    let c_major = Chord::from_triad(TriadQuality::Major, Some(PitchClass::C), Some(0));
    let e_major = Chord::from_triad(TriadQuality::Major, Some(PitchClass::E), Some(1));
    let f_major = Chord::from_triad(TriadQuality::Major, Some(PitchClass::F), Some(7));
    let f_sharp_major = Chord::from_triad(TriadQuality::Major, Some(PitchClass::FSharp), Some(8));

    // Minor Triads
    let a_minor = Chord::from_triad(TriadQuality::Minor, Some(PitchClass::A), Some(4));
    let a_flat_minor = Chord::from_triad(TriadQuality::Minor, Some(PitchClass::AFlat), Some(1));
    let a_sharp_minor = Chord::from_triad(TriadQuality::Minor, Some(PitchClass::ASharp), Some(1));
    let c_minor = Chord::from_triad(TriadQuality::Minor, Some(PitchClass::C), Some(0));

    //Augmented Triads
    let b_augmented = Chord::from_triad(TriadQuality::Augmented, Some(PitchClass::B), Some(5));
    let e_augmented = Chord::from_triad(TriadQuality::Augmented, Some(PitchClass::E), Some(7));

    //Diminished Triads
    let d_diminished = Chord::from_triad(TriadQuality::Diminished, Some(PitchClass::D), Some(3));
    let g_sharp_diminished =
        Chord::from_triad(TriadQuality::Diminished, Some(PitchClass::GSharp), Some(3));

    //Sus2 Triads
    let g_sus2 = Chord::from_triad(TriadQuality::Sus2, Some(PitchClass::G), Some(0));
    let g_flat_sus2 = Chord::from_triad(TriadQuality::Sus2, Some(PitchClass::GFlat), Some(1));

    //Sus4 Triads
    let a_sharp_sus4 = Chord::from_triad(TriadQuality::Sus4, Some(PitchClass::ASharp), Some(7));
    let f_sus4 = Chord::from_triad(TriadQuality::Sus4, Some(PitchClass::F), Some(8));

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
        (f_sus4, ["F8", "Bb8", "C9"]),
    ];

    for test_case in test_cases {
        let output_vec: Vec<Note> = Vec::try_from(test_case.0).unwrap();
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
                output_vec[index].get_frequency(),
                expected_vec[index].get_frequency()
            );
        }
    }
}
