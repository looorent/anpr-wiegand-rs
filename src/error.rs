use alloc::string::String;
use core::fmt;

/// Errors returned when encoding or decoding a Wiegand value.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    /// The sanitized license plate exceeds the maximum number of characters supported by the format.
    TooManyCharacters {
        /// Name of the Wiegand format (`"Wiegand26"` or `"Wiegand64"`).
        format: &'static str,
        /// Maximum number of characters supported by the format.
        max: usize,
        /// The sanitized license plate that was rejected.
        sanitized: String,
    },
    /// A Wiegand 26 value was required but the input is empty or blank.
    Blank,
    /// The input contains a character that is not a hexadecimal digit.
    InvalidHexadecimal,
    /// The hexadecimal value has more bits than the format (26 or 64).
    TooLong {
        /// Name of the Wiegand format (`"Wiegand26"` or `"Wiegand64"`).
        format: &'static str,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::TooManyCharacters {
                format,
                max,
                sanitized,
            } => write!(
                f,
                "{format} does not support license plate containing more than {max} characters: {sanitized}"
            ),
            Error::Blank => f.write_str("A Wiegand26 cannot be empty or blank"),
            Error::InvalidHexadecimal => f.write_str("Invalid hexadecimal value"),
            Error::TooLong { format } => write!(f, "{format} is too long."),
        }
    }
}

impl core::error::Error for Error {}
