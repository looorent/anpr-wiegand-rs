use anpr_wiegand::{
    Error, Wiegand26,
    wiegand26::{
        decode, encode, read_concatenation_of_facility_code_and_id_number, read_decimal_payload,
        read_facility_code_from, read_id_number_from,
    },
};

fn assert_encodes_to(cases: &[(&str, &str)]) {
    for &(plate, expected) in cases {
        let hexadecimal = encode(plate).unwrap().unwrap().hexadecimal;
        assert_eq!(hexadecimal, expected, "encode({plate:?})");
        assert_eq!(hexadecimal.len(), 7, "encode({plate:?})");
    }
}

#[test]
fn encode_returns_none_for_blank_input() {
    for blank in ["", "   ", " ", "-- ."] {
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
                    format: "Wiegand26",
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
        ("1WFV385", "1A98B4B"),
        ("1SDF534", "2521053"),
        ("ADF543", "226C607"),
        ("DE56G", "31C234A"),
        ("FF23DDFDF5", "090BF4D"),
        ("AZERTYUIOI", "006D2EC"),
        ("1FRDEERE", "1A7622D"),
        ("2ZZD456", "2866DD8"),
    ]);
}

#[test]
fn encode_short_plates() {
    assert_encodes_to(&[
        ("85", "0A6D5C5"),
        ("34", "3FE96B2"),
        ("3", "0DD0777"),
        ("DFDF5", "1F628B4"),
        ("YUIOI", "3A9BABB"),
        ("ERE", "08F4AEC"),
        ("56", "3EB746E"),
    ]);
}

#[test]
fn encode_plates_with_spaces_and_special_characters() {
    assert_encodes_to(&[
        ("HK 55 EVB", "3019E2A"),
        ("HK-55-EVB", "3019E2A"),
        ("HK-55-evb", "3019E2A"),
        (" HK-55€evb ", "3019E2A"),
        ("1wfv385", "1A98B4B"),
        ("1S--DF534", "2521053"),
        ("A__DF543", "226C607"),
        ("de56g", "31C234A"),
        ("FF23DDFDF5", "090BF4D"),
        ("azer)TYUIOI", "006D2EC"),
        ("1FR//DEERE", "1A7622D"),
        ("2ZZD4;;..56", "2866DD8"),
    ]);
}

#[test]
fn encode_returns_the_same_fields_as_decode() {
    let encoded = encode("1WFV385").unwrap().unwrap();
    assert_eq!(Some(encoded.clone()), decode(&encoded.hexadecimal).unwrap());
}

#[test]
fn decode_returns_none_for_blank_input() {
    for blank in ["", "   "] {
        assert_eq!(decode(blank), Ok(None), "decode({blank:?})");
    }
}

#[test]
fn decode_returns_all_fields() {
    assert_eq!(
        decode("1A98B4B"),
        Ok(Some(Wiegand26 {
            hexadecimal: "1A98B4B".into(),
            decimal: 13944229,
            facility_code: 212,
            id_number: 50597,
            facility_code_and_id_number: 21250597,
        }))
    );
    assert_eq!(
        decode("31C234A"),
        Ok(Some(Wiegand26 {
            hexadecimal: "31C234A".into(),
            decimal: 9310629,
            facility_code: 142,
            id_number: 4517,
            facility_code_and_id_number: 14204517,
        }))
    );
}

#[test]
fn decode_rejects_invalid_hexadecimal() {
    for input in ["1A98B4G", "+1A98B4", "0x1A98B4", "1A 98B4"] {
        assert_eq!(
            decode(input),
            Err(Error::InvalidHexadecimal),
            "decode({input:?})"
        );
    }
}

#[test]
fn decode_rejects_values_wider_than_26_bits() {
    for input in ["4000000", "FFFFFFFFFFFFFFFFFF"] {
        assert_eq!(
            decode(input),
            Err(Error::TooLong {
                format: "Wiegand26"
            }),
            "decode({input:?})"
        );
    }
}

#[test]
fn read_functions_reject_blank_input() {
    for blank in ["    ", ""] {
        assert_eq!(read_facility_code_from(blank), Err(Error::Blank));
        assert_eq!(read_id_number_from(blank), Err(Error::Blank));
        assert_eq!(read_decimal_payload(blank), Err(Error::Blank));
        assert_eq!(
            read_concatenation_of_facility_code_and_id_number(blank),
            Err(Error::Blank)
        );
    }
}

#[test]
fn read_facility_code_from_rejects_values_wider_than_26_bits() {
    assert_eq!(
        read_facility_code_from("4000000"),
        Err(Error::TooLong {
            format: "Wiegand26"
        })
    );
    assert_eq!(
        read_facility_code_from("4000000").unwrap_err().to_string(),
        "Wiegand26 is too long."
    );
}

#[test]
fn read_facility_code_from_valid_values() {
    for (input, expected) in [
        ("3019E2A", 128),
        ("2521053", 41),
        ("1A7622D", 211),
        ("31C234A", 142),
        ("1A98B4B", 212),
    ] {
        assert_eq!(read_facility_code_from(input), Ok(expected), "{input}");
    }
}

#[test]
fn read_id_number_from_valid_values() {
    for (input, expected) in [
        ("3019E2A", 53013),
        ("2521053", 2089),
        ("1A7622D", 45334),
        ("31C234A", 4517),
        ("1A98B4B", 50597),
    ] {
        assert_eq!(read_id_number_from(input), Ok(expected), "{input}");
    }
}

#[test]
fn read_decimal_payload_valid_values() {
    for (input, expected) in [
        ("3019E2A", 8441621),
        ("2521053", 2689065),
        ("1A7622D", 13873430),
        ("31C234A", 9310629),
        ("1A98B4B", 13944229),
    ] {
        assert_eq!(read_decimal_payload(input), Ok(expected), "{input}");
    }
}

#[test]
fn read_concatenation_of_facility_code_and_id_number_valid_values() {
    for (input, expected) in [
        ("3019E2A", 12853013),
        ("2521053", 4102089),
        ("1A7622D", 21145334),
        ("31C234A", 14204517),
        ("1A98B4B", 21250597),
    ] {
        assert_eq!(
            read_concatenation_of_facility_code_and_id_number(input),
            Ok(expected),
            "{input}"
        );
    }
}
