use music_tools::common::{PentatonicType, ScaleType};
use music_tools::pitchclass::{PitchClass, PitchClasses};
use music_tools::scale::Scale;
use std::cmp;

#[test]
fn test_scale_notes() {
    //Blues Scales
    let c_major_blues = Scale::from(ScaleType::MajorBlues, PentatonicType::None)
        .unwrap()
        .to_pitch_classes(PitchClasses::C);
    let a_minor_blues = Scale::from(ScaleType::MinorBlues, PentatonicType::None)
        .unwrap()
        .to_pitch_classes(PitchClasses::A);
    let a_nonatonic_blues = Scale::from(ScaleType::NonatonicBlues, PentatonicType::None)
        .unwrap()
        .to_pitch_classes(PitchClasses::A);

    let test_cases = [
        (c_major_blues, vec!["C", "D", "E♭", "E", "G", "A", "C"]),
        (a_minor_blues, vec!["A", "C", "D", "E♭", "E", "G", "A"]),
        (
            a_nonatonic_blues,
            vec!["A", "B", "C", "D♭", "D", "E", "G♭", "G", "A♭", "A"],
        ),
    ];

    for test_case in test_cases {
        let output_vec: Vec<&'static PitchClass> = test_case.0;
        let expected_vec: Vec<&'static PitchClass> = test_case
            .1
            .into_iter()
            .map(|x| PitchClass::from_name(x).unwrap())
            .collect();
        assert_eq!(output_vec.len(), expected_vec.len());
        for index in 0..cmp::min(output_vec.len(), expected_vec.len()) {
            assert_eq!(output_vec[index], expected_vec[index]);
        }
    }
}
