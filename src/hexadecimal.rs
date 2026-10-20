use crate::error::Error;

/// Parses `value` as unsigned hexadecimal digits only (no sign, prefix or whitespace).
///
/// # Errors
///
/// [`Error::InvalidHexadecimal`] when `value` is empty or contains a non-hexadecimal character,
/// [`Error::TooLong`] (attributed to `format`) when it overflows `u64`.
pub(crate) fn parse(value: &str, format: &'static str) -> Result<u64, Error> {
    if value.is_empty() || !value.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(Error::InvalidHexadecimal);
    }
    u64::from_str_radix(value, 16).map_err(|_| Error::TooLong { format })
}
