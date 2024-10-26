[![](https://img.shields.io/crates/v/ydke_parser.svg)](https://crates.io/crates/ydke_parser)
![Maintenance](https://img.shields.io/badge/maintenance-activly--developed-brightgreen.svg)
[![Tests](https://img.shields.io/github/actions/workflow/status/Satellaa/ydke_parser/miri.yml)](https://github.com/Satellaa/ydke_parser/actions)
[![Docs](https://docs.rs/ydke_parser/badge.svg)](https://docs.rs/ydke_parser)

# ydke_parser

A crate for parsing and generating YGOPro deck URLs in the YDKE format.

## Example

```rust
use ydke_parser::{Deck, parse_url, to_url};

// Create a deck
let deck = Deck {
    main: vec![26077387, 37351133],
    extra: vec![63288574],
    side: vec![37351133],
};

let url1 = "ydke://y+iNAd3uOQI=!/rTFAw==!3e45Ag==!";
// Convert to URL
let url2 = to_url(&deck);

// Parse the URL back into a deck
let parsed_deck1 = parse_url(&url1).unwrap();
let parsed_deck2 = parse_url(&url2).unwrap();
assert_eq!(deck, parsed_deck1);
assert_eq!(deck, parsed_deck2);
```

