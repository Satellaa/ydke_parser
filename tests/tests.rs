use ydke_parser::{Deck, parse_url, to_url, YdkeError};

#[test]
fn test_url_parsing() {
    let deck = Deck {
        main: vec![26077387, 26077387],
        extra: vec![63288574],
        side: vec![37351133],
    };
    let url = to_url(&deck);
    let parsed_deck = parse_url(&url).unwrap();
    assert_eq!(deck.main[0], parsed_deck.main[0]);
    assert_eq!(deck, parsed_deck);
}

#[test]
fn test_invalid_ydke_url() {
    let result = parse_url("invalid://test");
    assert!(matches!(result, Err(YdkeError::NotYdkeUrl)));
}

#[test]
fn test_empty_deck() {
    let deck = Deck {
        main: vec![],
        extra: vec![],
        side: vec![],
    };
    let url = to_url(&deck);
    let parsed = parse_url(&url).unwrap();
    assert_eq!(deck, parsed);
}