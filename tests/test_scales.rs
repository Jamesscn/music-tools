use music_tools::common::{PentatonicType, ScaleType};
use music_tools::pitchclass::PitchClass;
use music_tools::scale::Scale;
use std::cmp;
use std::str::FromStr;

#[test]
fn test_scale_notes() {
    //Blues Scales
    let c_major_blues = Scale::try_new(ScaleType::MajorBlues, PentatonicType::None)
        .unwrap()
        .to_pitch_classes(PitchClass::C);
    let a_minor_blues = Scale::try_new(ScaleType::MinorBlues, PentatonicType::None)
        .unwrap()
        .to_pitch_classes(PitchClass::A);
    let a_nonatonic_blues = Scale::try_new(ScaleType::NonatonicBlues, PentatonicType::None)
        .unwrap()
        .to_pitch_classes(PitchClass::A);

    let test_cases = [
        (c_major_blues, vec!["C", "D", "E♭", "E", "G", "A", "C"]),
        (a_minor_blues, vec!["A", "C", "D", "E♭", "E", "G", "A"]),
        (
            a_nonatonic_blues,
            vec!["A", "B", "C", "D♭", "D", "E", "G♭", "G", "A♭", "A"],
        ),
    ];

    for test_case in test_cases {
        let output_vec: Vec<PitchClass> = test_case.0;
        let expected_vec: Vec<PitchClass> = test_case
            .1
            .into_iter()
            .map(|x| PitchClass::from_str(x).unwrap())
            .collect();
        assert_eq!(output_vec.len(), expected_vec.len());
        for index in 0..cmp::min(output_vec.len(), expected_vec.len()) {
            assert_eq!(output_vec[index], expected_vec[index]);
        }
    }
}
