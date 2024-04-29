use music_tools::pitchclass::*;

#[test]
fn test_twelve_tone() {
    //Test augmenting and diminishing C
    let p = C;
    assert_eq!(p.clone().to_string(), "C");
    assert_eq!(
        p.clone().diminish().map(|p| p.to_string()),
        Some("C♭".to_string())
    );
    assert_eq!(
        p.clone()
            .diminish()
            .and_then(|p| p.diminish())
            .map(|p| p.to_string()),
        Some("C♭♭".to_string())
    );
    assert_eq!(
        p.clone()
            .diminish()
            .and_then(|p| p.diminish())
            .and_then(|p| p.diminish())
            .map(|p| p.to_string()),
        None
    );
    assert_eq!(
        p.clone().augment().map(|p| p.to_string()),
        Some("C♯".to_string())
    );
    assert_eq!(
        p.clone()
            .augment()
            .and_then(|p| p.augment())
            .map(|p| p.to_string()),
        Some("C♯♯".to_string())
    );
    assert_eq!(
        p.clone()
            .augment()
            .and_then(|p| p.augment())
            .and_then(|p| p.augment())
            .map(|p| p.to_string()),
        None
    );

    //Test augmenting and diminishing C flag
    let p = C_FLAT;
    assert_eq!(p.clone().to_string(), "C♭");
    assert_eq!(
        p.clone().diminish().map(|p| p.to_string()),
        Some("C♭♭".to_string())
    );
    assert_eq!(
        p.clone()
            .diminish()
            .and_then(|p| p.diminish())
            .map(|p| p.to_string()),
        None
    );
    assert_eq!(
        p.clone().augment().map(|p| p.to_string()),
        Some("C".to_string())
    );
    assert_eq!(
        p.clone()
            .augment()
            .and_then(|p| p.augment())
            .map(|p| p.to_string()),
        Some("C♯".to_string())
    );
    assert_eq!(
        p.clone()
            .augment()
            .and_then(|p| p.augment())
            .and_then(|p| p.augment())
            .map(|p| p.to_string()),
        Some("C♯♯".to_string())
    );
    assert_eq!(
        p.clone()
            .augment()
            .and_then(|p| p.augment())
            .and_then(|p| p.augment())
            .and_then(|p| p.augment())
            .map(|p| p.to_string()),
        None
    );

    //Test obtaining pitch classes from strings
    assert_eq!(TwelveTone::from_string("A"), Ok(A));
    assert_eq!(TwelveTone::from_string("C"), Ok(C));
    assert_eq!(TwelveTone::from_string("E"), Ok(E));
    assert_eq!(TwelveTone::from_string("A♮"), Ok(A));
    assert_eq!(TwelveTone::from_string("Bb"), Ok(B_FLAT));
    assert_eq!(TwelveTone::from_string("D#"), Ok(D_SHARP));
    assert_eq!(TwelveTone::from_string("Fbb"), Ok(F_DOUBLE_FLAT));
    assert_eq!(TwelveTone::from_string("G##"), Ok(G_DOUBLE_SHARP));
    assert_eq!(TwelveTone::from_string("F♭♭"), Ok(F_DOUBLE_FLAT));
    assert_eq!(TwelveTone::from_string("G♯♯"), Ok(G_DOUBLE_SHARP));
    assert_eq!(TwelveTone::from_string("Gx"), Ok(G_DOUBLE_SHARP));
    assert!(TwelveTone::from_string("H").is_err());
    assert!(TwelveTone::from_string("1").is_err());
    assert!(TwelveTone::from_string("Some text").is_err());
}
