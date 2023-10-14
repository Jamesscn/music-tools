use music_tools::chord::Chord;
use music_tools::common::{Beat, Fraction, PentatonicType, ScaleType, TriadQuality};
use music_tools::midi::common::MIDIEvent;
use music_tools::midi::parser::MIDI;
use music_tools::midi::track::Track;
use music_tools::note::Note;
use music_tools::pitchclass::PitchClass;
use music_tools::scale::Scale;

fn main() {
    let mut info_track = Track::new();
    let mut beat_track = Track::new();
    let mut melody_track = Track::new();
    info_track.push_event(MIDIEvent::SetTempo(170));
    info_track.push_event(MIDIEvent::SetTimeSignature(Fraction::new(3, 4)));

    //The melody is obtained from the C minor and harmonic minor scales
    let minor_scale = Scale::try_new(ScaleType::Minor, PentatonicType::None).unwrap();
    let harmonic_minor_scale =
        Scale::try_new(ScaleType::HarmonicMinor, PentatonicType::None).unwrap();
    let c_minor_scale = minor_scale.to_notes(PitchClass::C, 5);
    let c_harmonic_double_scale = [
        &harmonic_minor_scale.to_notes(PitchClass::C, 4)[0..7],
        &harmonic_minor_scale.to_notes(PitchClass::C, 5),
    ]
    .concat();
    let c_minor_functions = [5, 3, 2, 1, 1, 2, 3, 1, 3, 5, 6, 5];
    let c_harmonic_minor_functions = [11, 11, 9, 8, 7, 5, 7, 9, 7, 9, 11, 12, 13];
    let mut melody_notes: Vec<Note> = Vec::new();
    for function in c_minor_functions {
        melody_notes.push(c_minor_scale[function - 1]);
    }
    for function in c_harmonic_minor_functions {
        melody_notes.push(c_harmonic_double_scale[function - 1]);
    }
    melody_notes.push(Note::from_string("Gb5").unwrap());
    melody_notes.push(Note::from_string("G5").unwrap());

    //The beats for each of the notes are predefined
    let melody_beats = [
        Beat::HALF_DOTTED,
        Beat::HALF,
        Beat::QUARTER,
        Beat::WHOLE,
        Beat::QUARTER,
        Beat::QUARTER,
        Beat::QUARTER,
        Beat::QUARTER,
        Beat::QUARTER,
        Beat::HALF,
        Beat::QUARTER,
        Beat::HALF_DOTTED,
        Beat::HALF_DOTTED,
        Beat::HALF_DOTTED,
        Beat::HALF,
        Beat::QUARTER,
        Beat::WHOLE,
        Beat::QUARTER,
        Beat::QUARTER,
        Beat::QUARTER,
        Beat::QUARTER,
        Beat::QUARTER,
        Beat::QUARTER,
        Beat::QUARTER,
        Beat::QUARTER,
        Beat::HALF_DOTTED,
        Beat::HALF_DOTTED,
    ];

    //The chord progression is added on track 1
    for index in 0..60 {
        if index % 3 == 0 {
            if index % 6 == 0 {
                if !(30..54).contains(&index) {
                    beat_track.push_note(Note::new(PitchClass::C, 3), Beat::QUARTER);
                } else {
                    beat_track.push_note(Note::new(PitchClass::D, 3), Beat::QUARTER);
                }
            } else {
                beat_track.push_note(Note::new(PitchClass::G, 2), Beat::QUARTER);
            }
        } else if !(30..54).contains(&index) {
            beat_track.push_notes(
                Vec::try_from(Chord::from_triad(
                    TriadQuality::Minor,
                    Some(PitchClass::C),
                    Some(3),
                ))
                .unwrap(),
                Beat::QUARTER,
            );
        } else {
            beat_track.push_notes(
                Vec::try_from(Chord::from_triad(
                    TriadQuality::Minor,
                    Some(PitchClass::F),
                    Some(3),
                ))
                .unwrap(),
                Beat::QUARTER,
            );
        }
    }

    //The melody is added on track 2
    for index in 0..melody_notes.len() + 12 {
        if index < 12 {
            melody_track.push_rest(Beat::QUARTER);
        } else {
            melody_track.push_note(melody_notes[index - 12], melody_beats[index - 12]);
        }
    }

    //The MIDI object and file are created
    let mut midi = MIDI::new();
    midi.push(info_track);
    midi.push(beat_track);
    midi.push(melody_track);
    midi.export("second_waltz.mid")
        .expect("could not create midi file");
}
