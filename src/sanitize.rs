use alloc::string::String;

/// Trims and uppercases `license_plate`, then keeps only `[A-Z0-9]`.
///
/// Uppercasing is Unicode-aware before filtering, so characters such as `ı` (dotless i)
/// become `I`, matching the Java and JavaScript implementations.
pub(crate) fn sanitize(license_plate: &str) -> String {
    license_plate
        .trim()
        .chars()
        .flat_map(char::to_uppercase)
        .filter(char::is_ascii_alphanumeric)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::sanitize;

    #[test]
    fn removes_special_characters_and_uppercases() {
        assert_eq!(sanitize(" HK-55€evb "), "HK55EVB");
        assert_eq!(sanitize("2ZZD4;;..56"), "2ZZD456");
    }

    #[test]
    fn uppercases_non_ascii_letters_into_ascii() {
        assert_eq!(sanitize("ıx"), "IX");
    }

    #[test]
    fn returns_empty_for_blank_input() {
        assert_eq!(sanitize(""), "");
        assert_eq!(sanitize("   "), "");
        assert_eq!(sanitize("--"), "");
    }
}
