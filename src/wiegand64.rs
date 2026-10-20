//! Wiegand 64-bit format: a 4-bit header followed by 10 characters of 6 bits each.

use alloc::{
    format,
    string::{String, ToString},
};

use crate::{error::Error, hexadecimal, sanitize::sanitize};

const FORMAT: &str = "Wiegand64";
const MAX_NUMBER_OF_CHARACTERS: usize = 10;
const NUMBER_OF_BITS_PER_CHARACTER: usize = 6;
const HEADER: u64 = 0b0110 << 60;
const CHARACTER_MASK: u64 = 0b111111;
const EMPTY: u8 = 0b000000;
const UNKNOWN_CHARACTER: u8 = 0b111111;
const DIGIT_OFFSET: u8 = 0b010000;
const LETTER_OFFSET: u8 = DIGIT_OFFSET + 10;
const LAST_LETTER: u8 = LETTER_OFFSET + 25;

/// Encodes a license plate into the Wiegand 64-bit format, as 16 uppercase hexadecimal digits.
///
/// The plate is trimmed, uppercased and stripped of every character outside `[A-Z0-9]`,
/// then right-aligned on 10 characters.
///
/// Returns `Ok(None)` when nothing remains after sanitization.
///
/// # Errors
///
/// [`Error::TooManyCharacters`] when the sanitized plate is longer than 10 characters.
pub fn encode(license_plate: &str) -> Result<Option<String>, Error> {
    let sanitized = sanitize(license_plate);
    if sanitized.is_empty() {
        return Ok(None);
    }
    if sanitized.len() > MAX_NUMBER_OF_CHARACTERS {
        return Err(Error::TooManyCharacters {
            format: FORMAT,
            max: MAX_NUMBER_OF_CHARACTERS,
            sanitized,
        });
    }

    let bits =
        sanitized
            .bytes()
            .rev()
            .enumerate()
            .fold(HEADER, |bits, (index_from_end, character)| {
                bits | (u64::from(binary_representation_of(character))
                    << (NUMBER_OF_BITS_PER_CHARACTER * index_from_end))
            });
    Ok(Some(format!("{bits:X}")))
}

/// Decodes a hexadecimal Wiegand 64 value back into an uppercase license plate. The header is ignored.
///
/// Codes that do not map to `[A-Z0-9 ]` are decoded as `?`. Leading and trailing spaces are removed.
///
/// Returns `Ok(None)` when the input is blank.
///
/// # Errors
///
/// [`Error::InvalidHexadecimal`] when the input is not hexadecimal,
/// [`Error::TooLong`] when it does not fit on 64 bits.
pub fn decode(wiegand64_in_hexadecimal: &str) -> Result<Option<String>, Error> {
    let trimmed = wiegand64_in_hexadecimal.trim();
    if trimmed.is_empty() {
        return Ok(None);
    }
    let bits = hexadecimal::parse(trimmed, FORMAT)?;
    let plate: String = (0..MAX_NUMBER_OF_CHARACTERS)
        .rev()
        .map(|index_from_end| {
            let code = (bits >> (NUMBER_OF_BITS_PER_CHARACTER * index_from_end)) & CHARACTER_MASK;
            character_of(code as u8)
        })
        .collect();
    Ok(Some(plate.trim().to_string()))
}

fn binary_representation_of(character: u8) -> u8 {
    match character {
        b' ' => EMPTY,
        b'0'..=b'9' => DIGIT_OFFSET + (character - b'0'),
        b'A'..=b'Z' => LETTER_OFFSET + (character - b'A'),
        _ => UNKNOWN_CHARACTER,
    }
}

fn character_of(code: u8) -> char {
    match code {
        EMPTY => ' ',
        DIGIT_OFFSET..LETTER_OFFSET => char::from(b'0' + (code - DIGIT_OFFSET)),
        LETTER_OFFSET..=LAST_LETTER => char::from(b'A' + (code - LETTER_OFFSET)),
        _ => '?',
    }
}
