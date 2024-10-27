# Changelog

## [0.2.0]

### Added
- Added `parse_urls` function to parse multiple YDKE URLs into a vector of Deck structures
- Added `to_urls` function to convert multiple Deck structures into a vector of YDKE URLs

### Documentation
- Added comprehensive examples for new batch processing functions

## [0.1.0]

### Added
- Core YDKE URL parser and generator functionality
  - Added `parse_url` function to parse single YDKE URL into a Deck structure
  - Added `to_url` function to convert a Deck structure into YDKE URL
  - Added `extract_urls` utility function to extract YDKE URLs from text
- Implemented platform-independent binary data handling
  - Added support for both little-endian and big-endian systems
- Added error handling with custom `YdkeError` enum
  - Base64 decoding errors
  - URL format validation
  - Missing delimiter detection
- Added core data structures
  - `Deck` structure for representing YGOPro decks with main, extra, and side deck components

### Documentation
- Added comprehensive documentation with examples for all public functions
- Added crate-level documentation with usage examples