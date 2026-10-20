# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2026-10-20

* Stable API.
* Publish to crates.io through GitHub Actions, with Trusted Publishing.

## [0.1.0] - 2026-10-20

* First Rust implementation, ported from [anpr-wiegand](https://github.com/looorent/anpr-wiegand) (Java) and [anpr-wiegand-js](https://github.com/looorent/anpr-wiegand-js) (TypeScript).
* Wiegand 26-bit: `encode`, `decode`, `read_facility_code_from`, `read_id_number_from`, `read_decimal_payload`, `read_concatenation_of_facility_code_and_id_number`.
* Wiegand 64-bit: `encode`, `decode`.
* `no_std` (with `alloc`), zero dependencies.
