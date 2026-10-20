use anpr_wiegand::{
    Error,
    wiegand64::{decode, encode},
};

fn assert_encodes_to(cases: &[(&str, &str)]) {
    for &(plate, expected) in cases {
        let hexadecimal = encode(plate).unwrap().unwrap();
        assert_eq!(hexadecimal, expected, "encode({plate:?})");
        assert_eq!(hexadecimal.len(), 16, "encode({plate:?})");
    }
}

#[test]
fn encode_returns_none_for_blank_input() {
    for blank in ["", "   ", " ", "#"] {
        assert_eq!(encode(blank), Ok(None), "encode({blank:?})");
    }
}

#[test]
fn encode_rejects_plates_longer_than_ten_characters() {
    for plate in ["azertyuiop0987", "1234567899879825", "nnnnnnnnnnnnnnnn"] {
        assert!(
            matches!(
                encode(plate),
                Err(Error::TooManyCharacters {
                    format: "Wiegand64",
                    max: 10,
                    ..
                })
            ),
            "encode({plate:?})"
        );
    }
}

#[test]
fn encode_plates() {
    assert_encodes_to(&[
        ("AZERTYUIOP", "66B37ABB72BA2A29"),
        ("Z", "6000000000000033"),
        ("1WFV385", "6000011C1FBD3615"),
        ("1SDF534", "6000011B1D7D54D4"),
        ("ADF543", "600000069D7D5513"),
        ("DE56G", "600000001D7955A0"),
        ("FF23DDFDF5", "67DF49375D7DD7D5"),
        ("AZERTYUIOI", "66B37ABB72BA2A22"),
        ("1FRDEERE", "600045FADD79EADE"),
        ("2ZZD456", "6000012CF3754556"),
        ("1T1234", "600000046D4524D4"),
    ]);
}

#[test]
fn encode_short_plates() {
    assert_encodes_to(&[
        ("A", "600000000000001A"),
        ("EE", "600000000000079E"),
        ("012", "6000000000010452"),
        ("1T", "600000000000046D"),
        ("789", "6000000000017619"),
        ("42", "6000000000000512"),
        ("1337", "60000000004534D7"),
        ("CB", "600000000000071B"),
        ("ZIP", "60000000000338A9"),
        ("WAP", "60000000000306A9"),
        ("IU", "60000000000008AE"),
        ("PL", "6000000000000A65"),
    ]);
}

#[test]
fn encode_plates_with_spaces_and_special_characters() {
    assert_encodes_to(&[
        ("HK 55 EVB", "600002191555EBDB"),
        ("VR46#T", "600000002FAD45AD"),
        (" VR46#T   ", "600000002FAD45AD"),
        ("hk-55-evb", "600002191555EBDB"),
    ]);
}

#[test]
fn decode_returns_none_for_blank_input() {
    for blank in ["", "   ", " "] {
        assert_eq!(decode(blank), Ok(None), "decode({blank:?})");
    }
}

#[test]
fn decode_values() {
    for (hexadecimal, expected) in [
        ("66B37ABB72BA2A29", "AZERTYUIOP"),
        ("6000000000000033", "Z"),
        ("6000011C1FBD3615", "1WFV385"),
        ("600002191555EBDB", "HK55EVB"),
        ("6FFFFFFFFFFFFFFF", "??????????"),
        ("600000000000003F", "?"),
        ("600000000000003f", "?"),
    ] {
        assert_eq!(
            decode(hexadecimal).unwrap().as_deref(),
            Some(expected),
            "decode({hexadecimal:?})"
        );
    }
}

#[test]
fn decode_reverses_encode() {
    for plate in ["AZERTYUIOP", "Z", "1T1234", "HK55EVB", "0000000000"] {
        let encoded = encode(plate).unwrap().unwrap();
        assert_eq!(decode(&encoded).unwrap().as_deref(), Some(plate), "{plate}");
    }
}

#[test]
fn decode_rejects_invalid_hexadecimal() {
    for input in [
        "6000011C1FBD361Z",
        "+600000000000001A",
        "0x600000000000001A",
    ] {
        assert_eq!(
            decode(input),
            Err(Error::InvalidHexadecimal),
            "decode({input:?})"
        );
    }
}

#[test]
fn decode_rejects_values_wider_than_64_bits() {
    assert_eq!(
        decode("16000011C1FBD3615"),
        Err(Error::TooLong {
            format: "Wiegand64"
        })
    );
}
