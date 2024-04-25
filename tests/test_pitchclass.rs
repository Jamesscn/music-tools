use music_tools::pitchclass::TwelveTone;

#[test]
fn test_twelve_tone() {
    //Test augmenting and diminishing C
    let p = TwelveTone::C();
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
    let p = TwelveTone::C_FLAT();
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
    assert_eq!(TwelveTone::from_string("A"), Ok(TwelveTone::A()));
    assert_eq!(TwelveTone::from_string("C"), Ok(TwelveTone::C()));
    assert_eq!(TwelveTone::from_string("E"), Ok(TwelveTone::E()));
    assert_eq!(TwelveTone::from_string("A♮"), Ok(TwelveTone::A()));
    assert_eq!(TwelveTone::from_string("Bb"), Ok(TwelveTone::B_FLAT()));
    assert_eq!(TwelveTone::from_string("D#"), Ok(TwelveTone::D_SHARP()));
    assert_eq!(
        TwelveTone::from_string("Fbb"),
        Ok(TwelveTone::F_DOUBLE_FLAT())
    );
    assert_eq!(
        TwelveTone::from_string("G##"),
        Ok(TwelveTone::G_DOUBLE_SHARP())
    );
    assert_eq!(
        TwelveTone::from_string("F♭♭"),
        Ok(TwelveTone::F_DOUBLE_FLAT())
    );
    assert_eq!(
        TwelveTone::from_string("G♯♯"),
        Ok(TwelveTone::G_DOUBLE_SHARP())
    );
    assert_eq!(
        TwelveTone::from_string("Gx"),
        Ok(TwelveTone::G_DOUBLE_SHARP())
    );
    assert!(TwelveTone::from_string("H").is_err());
    assert!(TwelveTone::from_string("1").is_err());
    assert!(TwelveTone::from_string("Some text").is_err());
}
