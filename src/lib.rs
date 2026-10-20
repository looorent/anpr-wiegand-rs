#![doc = include_str!("../README.md")]
#![no_std]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

extern crate alloc;

mod error;
mod hexadecimal;
mod sanitize;
mod sha1;
pub mod wiegand26;
pub mod wiegand64;

pub use error::Error;
pub use wiegand26::{Wiegand26, decode as decode26, encode as encode26};
pub use wiegand64::{decode as decode64, encode as encode64};
