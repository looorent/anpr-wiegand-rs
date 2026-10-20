# anpr-wiegand

[![Crates.io](https://img.shields.io/crates/v/anpr-wiegand.svg)](https://crates.io/crates/anpr-wiegand)
[![Docs.rs](https://docs.rs/anpr-wiegand/badge.svg)](https://docs.rs/anpr-wiegand)
[![Test](https://github.com/looorent/anpr-wiegand-rs/actions/workflows/test.yml/badge.svg)](https://github.com/looorent/anpr-wiegand-rs/actions/workflows/test.yml)
[![License](https://img.shields.io/crates/l/anpr-wiegand.svg)](https://github.com/looorent/anpr-wiegand-rs/blob/main/LICENSE)

A Rust library to format license plates into Wiegand 26-bit and 64-bit formats, according to the [Nedap Wiegand Interface Module](https://www.nedapidentification.com/products/anpr/wiegand-interface-module/). This is commonly used in **Automatic Number Plate Recognition (ANPR)** systems to integrate with access control panels (Wiegand controllers).

This crate is a port of [looorent/anpr-wiegand](https://github.com/looorent/anpr-wiegand) (Java) and [looorent/anpr-wiegand-js](https://github.com/looorent/anpr-wiegand-js) (TypeScript), and produces the same outputs.

## What is Wiegand?

Wiegand is a standard interface used in contact-less card readers and access control systems. Since license plates are alphanumeric and Wiegand payloads are typically numeric, they must be encoded:

- **Wiegand 26-bit:** 1 even parity bit, 8 bits of facility code, 16 bits of ID number and 1 odd parity bit. A license plate does not fit on 24 bits, so a numeric ID is derived from the **SHA-1** of the plate.
- **Wiegand 64-bit:** a proprietary but common ANPR format encoding up to 10 alphanumeric characters directly, with a 6-bit mapping per character.

## Installation

```sh
cargo add anpr-wiegand
```

## Usage

```rust
use anpr_wiegand::{decode26, decode64, encode26, encode64};

// --- Wiegand 26-bit ---
let wiegand26 = encode26("1wfv-385")?.expect("plate is not blank");
assert_eq!(wiegand26.hexadecimal, "1A98B4B");

let decoded = decode26("1A98B4B")?.expect("input is not blank");
assert_eq!(decoded.facility_code, 212);
assert_eq!(decoded.id_number, 50597);
assert_eq!(decoded.decimal, 13944229);
assert_eq!(decoded.facility_code_and_id_number, 21250597);

// --- Wiegand 64-bit ---
assert_eq!(encode64("1WFV385")?.as_deref(), Some("6000011C1FBD3615"));
assert_eq!(decode64("6000011C1FBD3615")?.as_deref(), Some("1WFV385"));

// Blank inputs are not an error
assert_eq!(encode64("  ")?, None);
# Ok::<(), anpr_wiegand::Error>(())
```

## API

### Wiegand 26-bit (`anpr_wiegand::wiegand26`)

| Function | Returns |
|---|---|
| `encode(license_plate)` (re-exported as `encode26`) | `Result<Option<Wiegand26>, Error>` |
| `decode(hexadecimal)` (re-exported as `decode26`) | `Result<Option<Wiegand26>, Error>` |
| `read_facility_code_from(hexadecimal)` | `Result<u8, Error>` |
| `read_id_number_from(hexadecimal)` | `Result<u16, Error>` |
| `read_decimal_payload(hexadecimal)` | `Result<u32, Error>` |
| `read_concatenation_of_facility_code_and_id_number(hexadecimal)` | `Result<u32, Error>` |

```rust,ignore
pub struct Wiegand26 {
    pub hexadecimal: String,              // e.g. "1A98B4B"
    pub decimal: u32,                     // 24-bit payload, parity excluded
    pub facility_code: u8,                // 8-bit facility code
    pub id_number: u16,                   // 16-bit ID number
    pub facility_code_and_id_number: u32, // FC followed by ID padded to 5 digits, e.g. 21250597
}
```

### Wiegand 64-bit (`anpr_wiegand::wiegand64`)

| Function | Returns |
|---|---|
| `encode(license_plate)` (re-exported as `encode64`) | `Result<Option<String>, Error>` — 16 uppercase hexadecimal digits |
| `decode(hexadecimal)` (re-exported as `decode64`) | `Result<Option<String>, Error>` — unknown codes decoded as `?` |

### Behavior

- **Sanitization:** plates are trimmed, uppercased and stripped of every character outside `[A-Z0-9]`, so `"hk-55 evb"` and `"HK55EVB"` give the same result.
- **Blank input:** `encode*` / `decode*` return `Ok(None)`; `read_*` return `Err(Error::Blank)`.
- **Length:** more than 10 characters after sanitization returns `Err(Error::TooManyCharacters { .. })`.
- **Strict hexadecimal:** decoding rejects non-hexadecimal characters (`Error::InvalidHexadecimal`) and values wider than the format (`Error::TooLong { .. }`).

## Features

- **`no_std`:** only requires `alloc`, usable on embedded targets.
- **Zero dependencies:** SHA-1 is implemented internally (it is used as a hash function, not for security).
- **No `unsafe`.**

Minimum supported Rust version: **1.85**.

## Differences with the other implementations

- Wiegand 64 sanitization follows the JavaScript implementation (2.x): special characters are removed. The Java implementation only removes spaces and encodes other characters as `0b111111` (`"VR46#T"` gives `6000000BEB516FED` in Java, `600000002FAD45AD` here and in JavaScript).
- `null` / `undefined` inputs do not exist in Rust: an empty `&str` plays that role.

## Publishing

Releases are published to crates.io by the `publish.yml` GitHub Actions workflow, using [Trusted Publishing](https://crates.io/docs/trusted-publishing) (no long-lived token).

1. Update `version` in `Cargo.toml` and add an entry to `CHANGELOG.md`.
2. Commit and push to `main`.
3. Create and push a matching version tag:
   ```sh
   git tag v0.1.0
   git push --tags
   ```
4. The workflow checks that the tag matches `Cargo.toml`, runs the checks, publishes the crate and creates a GitHub release.

## License

[MIT](https://github.com/looorent/anpr-wiegand-rs/blob/main/LICENSE)
