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
//! // Convert to URL
//! let url = to_url(&deck);
//! 
//! // Parse the URL back into a deck
//! let parsed_deck = parse_url(&url).unwrap();
//! assert_eq!(deck, parsed_deck);
//! ```

mod error;
mod deck;
mod conversion;
mod utils;

pub use error::YdkeError;
pub use deck::Deck;
pub use conversion::{parse_url, to_url};
pub use utils::extract_urls;