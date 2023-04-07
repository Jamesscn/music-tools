use musictools::chord::Chord;
use musictools::common::{Beat, Fraction, PentatonicType, ScaleType, TriadQuality};
use musictools::midi::MIDI;
use musictools::note::Note;
use musictools::pitchclass::PitchClasses;
use musictools::scale::Scale;
use musictools::track::Track;

fn main() {
    //The tempo and time signature of the song is set
    let tempo: f32 = 170.0;
    let waltz: Fraction = Fraction::new(3, 4);
    let mut beat_track = Track::new(tempo, waltz);
    let mut melody_track = Track::new(tempo, waltz);

    //The melody is obtained from the C minor and harmonic minor scales
    let minor_scale = Scale::from(ScaleType::Minor, PentatonicType::None).unwrap();
    let harmonic_minor_scale = Scale::from(ScaleType::HarmonicMinor, PentatonicType::None).unwrap();
    let c_minor_scale = minor_scale.to_notes(PitchClasses::C, 5);
    let c_harmonic_double_scale = [
        &harmonic_minor_scale.to_notes(PitchClasses::C, 4)[0..7],
        &harmonic_minor_scale.to_notes(PitchClasses::C, 5),
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
                    beat_track.add_note(Note::from(PitchClasses::C, 3), Beat::QUARTER);
                } else {
                    beat_track.add_note(Note::from(PitchClasses::D, 3), Beat::QUARTER);
                }
            } else {
                beat_track.add_note(Note::from(PitchClasses::G, 2), Beat::QUARTER);
            }
        } else if !(30..54).contains(&index) {
            beat_track.add_chord(
                Chord::from_triad(TriadQuality::Minor, Some(PitchClasses::C), Some(3)),
                Beat::QUARTER,
            );
        } else {
            beat_track.add_chord(
                Chord::from_triad(TriadQuality::Minor, Some(PitchClasses::F), Some(3)),
                Beat::QUARTER,
            );
        }
    }

    //The melody is added on track 2
    for index in 0..melody_notes.len() + 12 {
        if index < 12 {
            melody_track.add_rest(Beat::QUARTER);
        } else {
            melody_track.add_note(melody_notes[index - 12], melody_beats[index - 12]);
        }
    }

    //The MIDI object and file are created
    let mut midi: MIDI = MIDI::new();
    midi.add_track(beat_track);
    midi.add_track(melody_track);
    midi.export_to_file("second_waltz.mid");
}
