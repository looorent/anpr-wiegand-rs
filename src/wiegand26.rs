//! Wiegand 26-bit format, derived from the SHA-1 of the sanitized license plate.
//!
//! Layout: 1 even parity bit, 8 bits of facility code, 16 bits of ID number, 1 odd parity bit.

use alloc::{format, string::String};

use crate::{error::Error, hexadecimal, sanitize::sanitize, sha1::sha1};

const FORMAT: &str = "Wiegand26";
const MAX_NUMBER_OF_CHARACTERS: usize = 10;
const WIEGAND26_LENGTH: u32 = 26;
const EVEN_PARITY_FIELD: u32 = 0b11111111111110000000000000;
const ODD_PARITY_FIELD: u32 = 0b00000000000001111111111111;
const EVEN_PARITY_MASK: u32 = 0b10000000000000000000000000;
const ODD_PARITY_MASK: u32 = 0b00000000000000000000000001;

/// A Wiegand 26-bit value and its constituent fields.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Wiegand26 {
    /// The 26 bits (parity included) as uppercase hexadecimal, e.g. `"1A98B4B"`.
    pub hexadecimal: String,
    /// The 24-bit payload (parity excluded) as a number.
    pub decimal: u32,
    /// The 8-bit facility code.
    pub facility_code: u8,
    /// The 16-bit ID number.
    pub id_number: u16,
    /// The facility code followed by the ID number left-padded to 5 digits, e.g. `212` and `50597` give `21250597`.
    pub facility_code_and_id_number: u32,
}

impl Wiegand26 {
    fn from_payload(hexadecimal: String, payload: u32) -> Self {
        let facility_code = (payload >> 16) as u8;
        let id_number = payload as u16;
        Wiegand26 {
            hexadecimal,
            decimal: payload,
            facility_code,
            id_number,
            facility_code_and_id_number: concatenate(facility_code, id_number),
        }
    }
}

/// Encodes a license plate into the Wiegand 26-bit format.
///
/// The plate is trimmed, uppercased and stripped of every character outside `[A-Z0-9]`.
///
/// Returns `Ok(None)` when nothing remains after sanitization.
///
/// # Errors
///
/// [`Error::TooManyCharacters`] when the sanitized plate is longer than 10 characters.
pub fn encode(license_plate: &str) -> Result<Option<Wiegand26>, Error> {
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

    let digest = sha1(sanitized.as_bytes());
    let payload = u32::from_be_bytes([0, digest[17], digest[18], digest[19]]);
    let bits = add_parity_bits(payload << 1);
    Ok(Some(Wiegand26::from_payload(
        format!("{bits:07X}"),
        payload,
    )))
}

/// Decodes a hexadecimal Wiegand 26 value into its fields. Parity bits are not checked.
///
/// Returns `Ok(None)` when the input is blank. The `hexadecimal` field holds the input unchanged.
///
/// # Errors
///
/// [`Error::InvalidHexadecimal`] when the input is not hexadecimal,
/// [`Error::TooLong`] when it does not fit on 26 bits.
pub fn decode(wiegand26_in_hexadecimal: &str) -> Result<Option<Wiegand26>, Error> {
    if wiegand26_in_hexadecimal.trim().is_empty() {
        return Ok(None);
    }
    let payload = read_payload(wiegand26_in_hexadecimal)?;
    Ok(Some(Wiegand26::from_payload(
        String::from(wiegand26_in_hexadecimal),
        payload,
    )))
}

/// Reads the 8-bit facility code from a hexadecimal Wiegand 26 value.
///
/// # Errors
///
/// [`Error::Blank`], [`Error::InvalidHexadecimal`] or [`Error::TooLong`].
pub fn read_facility_code_from(wiegand26_in_hexadecimal: &str) -> Result<u8, Error> {
    read_payload(wiegand26_in_hexadecimal).map(|payload| (payload >> 16) as u8)
}

/// Reads the 16-bit ID number from a hexadecimal Wiegand 26 value.
///
/// # Errors
///
/// [`Error::Blank`], [`Error::InvalidHexadecimal`] or [`Error::TooLong`].
pub fn read_id_number_from(wiegand26_in_hexadecimal: &str) -> Result<u16, Error> {
    read_payload(wiegand26_in_hexadecimal).map(|payload| payload as u16)
}

/// Reads the 24-bit payload (parity bits excluded) from a hexadecimal Wiegand 26 value.
///
/// # Errors
///
/// [`Error::Blank`], [`Error::InvalidHexadecimal`] or [`Error::TooLong`].
pub fn read_decimal_payload(wiegand26_in_hexadecimal: &str) -> Result<u32, Error> {
    read_payload(wiegand26_in_hexadecimal)
}

/// Reads the facility code followed by the ID number left-padded to 5 digits.
///
/// # Errors
///
/// [`Error::Blank`], [`Error::InvalidHexadecimal`] or [`Error::TooLong`].
pub fn read_concatenation_of_facility_code_and_id_number(
    wiegand26_in_hexadecimal: &str,
) -> Result<u32, Error> {
    read_payload(wiegand26_in_hexadecimal)
        .map(|payload| concatenate((payload >> 16) as u8, payload as u16))
}

fn read_payload(wiegand26_in_hexadecimal: &str) -> Result<u32, Error> {
    let trimmed = wiegand26_in_hexadecimal.trim();
    if trimmed.is_empty() {
        return Err(Error::Blank);
    }
    let value = hexadecimal::parse(trimmed, FORMAT)?;
    if value >= 1 << WIEGAND26_LENGTH {
        return Err(Error::TooLong { format: FORMAT });
    }
    Ok(((value >> 1) & 0xFF_FFFF) as u32)
}

fn concatenate(facility_code: u8, id_number: u16) -> u32 {
    u32::from(facility_code) * 100_000 + u32::from(id_number)
}

fn add_parity_bits(bits: u32) -> u32 {
    set_odd_parity_bit(set_even_parity_bit(bits))
}

fn set_even_parity_bit(bits: u32) -> u32 {
    if (bits & EVEN_PARITY_FIELD).count_ones() % 2 != 0 {
        bits | EVEN_PARITY_MASK
    } else {
        bits
    }
}

fn set_odd_parity_bit(bits: u32) -> u32 {
    if (bits & ODD_PARITY_FIELD).count_ones() % 2 == 0 {
        bits | ODD_PARITY_MASK
    } else {
        bits
    }
}
