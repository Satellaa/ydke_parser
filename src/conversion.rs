use base64::{engine::general_purpose::STANDARD as base64_engine, Engine};
use crate::error::YdkeError;
use crate::deck::Deck;

/// Converts bytes to u32 according to platform endianness
#[cfg(target_endian = "little")]
fn bytes_to_u32(bytes: &[u8]) -> u32 {
    u32::from_le_bytes(bytes.try_into().unwrap())
}

#[cfg(target_endian = "big")]
fn bytes_to_u32(bytes: &[u8]) -> u32 {
    let mut arr = [0u8; 4];
    arr.copy_from_slice(bytes);
    arr.reverse();
    u32::from_be_bytes(arr)
}

/// Converts u32 to bytes according to platform endianness
#[cfg(target_endian = "little")]
fn u32_to_bytes(value: u32) -> [u8; 4] {
    value.to_le_bytes()
}

#[cfg(target_endian = "big")]
fn u32_to_bytes(value: u32) -> [u8; 4] {
    let mut bytes = value.to_be_bytes();
    bytes.reverse();
    bytes
}

/// Converts a base64 string to a vector of card passcodes.
fn base64_to_passcodes(base64: &str) -> Result<Vec<u32>, YdkeError> {
    let bytes = base64_engine.decode(base64)?;
    
    if bytes.len() % 4 != 0 {
        return Ok(Vec::new());
    }
    
    let passcodes: Vec<u32> = bytes
        .chunks_exact(4)
        .map(bytes_to_u32)
        .collect();
    
    Ok(passcodes)
}

/// Converts a slice of card passcodes to a base64 string.
fn passcodes_to_base64(passcodes: &[u32]) -> String {
    let bytes: Vec<u8> = passcodes
        .iter()
        .flat_map(|&code| u32_to_bytes(code).to_vec())
        .collect();
    
    base64_engine.encode(bytes)
}

/// Parses a YDKE URL into a Deck structure.
/// 
/// # Example
/// 
/// ```
/// use ydke_parser::parse_url;
/// 
/// let url = "ydke://F/8hCg==!!!";
/// let deck = parse_url(url).unwrap();
/// assert_eq!(deck.main.len(), 1);
/// ```
pub fn parse_url(ydke: &str) -> Result<Deck, YdkeError> {
    if !ydke.starts_with("ydke://") {
        return Err(YdkeError::NotYdkeUrl);
    }

    let components: Vec<&str> = ydke["ydke://".len()..]
        .split('!')
        .collect();
    
    if components.len() < 3 {
        return Err(YdkeError::MissingDelimiters);
    }

    Ok(Deck {
        main: base64_to_passcodes(components[0])?,
        extra: base64_to_passcodes(components[1])?,
        side: base64_to_passcodes(components[2])?,
    })
}

/// Parses multiple YDKE URLs into a vector of Deck structures.
/// 
/// # Example
/// 
/// ```
/// use ydke_parser::parse_urls;
/// 
/// let urls = &[
///     "ydke://F/8hCg==!!!",
///     "ydke://y+iNAQ==!!!"
/// ];
/// let decks = parse_urls(urls).unwrap();
/// assert_eq!(decks.len(), 2);
/// ```
pub fn parse_urls(urls: &[&str]) -> Result<Vec<Deck>, YdkeError> {
    urls.iter()
        .map(|url| parse_url(url))
        .collect()
}

/// Converts a Deck structure to a YDKE URL.
/// 
/// # Example
/// 
/// ```
/// use ydke_parser::{Deck, to_url};
/// 
/// let deck = Deck {
///     main: vec![26077387],
///     extra: vec![],
///     side: vec![],
/// };
/// let url = to_url(&deck);
/// assert!(url.starts_with("ydke://"));
/// ```
pub fn to_url(deck: &Deck) -> String {
    format!(
        "ydke://{}!{}!{}!",
        passcodes_to_base64(&deck.main),
        passcodes_to_base64(&deck.extra),
        passcodes_to_base64(&deck.side)
    )
}

/// Converts multiple Deck structures into a vector of YDKE URLs.
/// 
/// # Example
/// 
/// ```
/// use ydke_parser::{Deck, to_urls};
/// 
/// let decks = vec![
///     Deck {
///         main: vec![26077387],
///         extra: vec![],
///         side: vec![],
///     },
///     Deck {
///         main: vec![37351133],
///         extra: vec![],
///         side: vec![],
///     },
/// ];
/// let urls = to_urls(&decks);
/// assert_eq!(urls.len(), 2);
/// assert!(urls[0].starts_with("ydke://"));
/// assert!(urls[1].starts_with("ydke://"));
/// ```
pub fn to_urls(decks: &[Deck]) -> Vec<String> {
    decks.iter()
        .map(to_url)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64_conversion() {
        let passcodes = vec![26077387, 37351133];
        let base64 = passcodes_to_base64(&passcodes);
        let decoded = base64_to_passcodes(&base64).unwrap();
        assert_eq!(passcodes, decoded);
    }
}