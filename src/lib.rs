//! A crate for parsing and generating YGOPro deck URLs in the YDKE format.
//! 
//! # Examples
//! 
//! ```
//! use ydke_parser::{Deck, parse_url, to_url};
//! 
//! // Create a deck
//! let deck = Deck {
//!     main: vec![26077387, 37351133],
//!     extra: vec![63288574],
//!     side: vec![37351133],
//! };
//!
//! let url1 = "ydke://y+iNAd3uOQI=!/rTFAw==!3e45Ag==!";
//! // Convert deck to URL
//! let url2 = to_url(&deck);
//! 
//! // Parse the URL back into a deck
//! let parsed_deck1 = parse_url(&url1).unwrap();
//! let parsed_deck2 = parse_url(&url2).unwrap();
//! assert_eq!(deck, parsed_deck1);
//! assert_eq!(deck, parsed_deck2);
//! ```

mod error;
mod deck;
mod conversion;
mod utils;

pub use error::YdkeError;
pub use deck::Deck;
pub use conversion::{parse_url, to_url};
pub use utils::extract_urls;