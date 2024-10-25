![Maintenance](https://img.shields.io/badge/maintenance-activly--developed-brightgreen.svg)
[![Tests](https://img.shields.io/github/actions/workflow/status/ThatOneFrench/ydke_parser/miri.yml)](https://github.com/ThatOneFrench/ydke_parser/actions)

# ydke_parser

A crate for parsing and generating YGOPro deck URLs in the YDKE format.

## Examples

```rust
use ydke_parser::{Deck, parse_url, to_url};

// Create a deck
let deck = Deck {
    main: vec![26077387, 37351133],
    extra: vec![63288574],
    side: vec![37351133],
};

// Convert to URL
let url = to_url(&deck);

// Parse the URL back into a deck
let parsed_deck = parse_url(&url).unwrap();
assert_eq!(deck, parsed_deck);
```

