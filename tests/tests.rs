use ydke_parser::{Deck, parse_url, to_url, parse_urls, to_urls, YdkeError};

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

#[test]
fn test_multiple_url_parsing() {
    let decks = vec![
        Deck {
            main: vec![26077387, 37351133],
            extra: vec![63288574],
            side: vec![],
        },
        Deck {
            main: vec![26077387],
            extra: vec![],
            side: vec![37351133],
        },
    ];
    
    let urls = to_urls(&decks);
    let parsed_decks = parse_urls(&[&urls[0], &urls[1]]).unwrap();
    assert_eq!(decks, parsed_decks);
}

#[test]
fn test_large_deck() {
    let main_deck: Vec<u32> = (0..60).map(|_| 26077387).collect();
    let extra_deck: Vec<u32> = (0..15).map(|_| 63288574).collect();
    let side_deck: Vec<u32> = (0..15).map(|_| 37351133).collect();
    
    let deck = Deck {
        main: main_deck,
        extra: extra_deck,
        side: side_deck,
    };
    
    let url = to_url(&deck);
    let parsed_deck = parse_url(&url).unwrap();
    assert_eq!(deck, parsed_deck);
    assert_eq!(parsed_deck.main.len(), 60);
    assert_eq!(parsed_deck.extra.len(), 15);
    assert_eq!(parsed_deck.side.len(), 15);
}