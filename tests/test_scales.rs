use music_tools::chord::Chord;
use music_tools::note::Note;
use music_tools::scale::Scale;
use std::cmp;

#[test]
fn test_scale_notes() {
    let test_cases = [
        (
            Chord::from_semitones(&Scale::MAJOR_BLUES().to_semitones())
                .set_base_note("C4".try_into().unwrap()),
            vec!["C4", "D4", "E♭4", "E4", "G4", "A4", "C5"],
        ),
        (
            Chord::from_semitones(&Scale::MINOR_BLUES().to_semitones())
                .set_base_note("A4".try_into().unwrap()),
            vec!["A4", "C5", "D5", "E♭5", "E5", "G5", "A5"],
        ),
        (
            Chord::from_semitones(&Scale::NONATONIC_BLUES().to_semitones())
                .set_base_note("A4".try_into().unwrap()),
            vec![
                "A4", "B4", "C5", "D♭5", "D5", "E5", "G♭5", "G5", "A♭5", "A5",
            ],
        ),
    ];

    for test_case in test_cases {
        let output_vec: Vec<Note> = test_case.0.to_notes();
        let expected_vec: Vec<Note> = test_case
            .1
            .into_iter()
            .map(|x| Note::from_string(x).unwrap())
            .collect();
        assert_eq!(output_vec.len(), expected_vec.len());
        for index in 0..cmp::min(output_vec.len(), expected_vec.len()) {
            assert_eq!(output_vec[index], expected_vec[index]);
        }
    }
}
