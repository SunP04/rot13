//! Basic crate for using a Caesar Cipher.
#![warn(missing_docs)]

use caesar_cipher::CaesarCipher;

/// Defines a basic Caesar Cipher.
pub mod caesar_cipher;
/// Base module for defining what a Cipher is.
pub mod cipher;
/// Basic constants. Should not be used outside of crate.
pub(crate) mod constants;
/// Utility functions to encode/decode strings.
pub mod functions;
/// Module that extends the caesar_cipher module, creating Rot13 objects instead.
pub mod rot13;

pub use caesar_cipher::*;
pub use cipher::*;
pub use functions::*;
pub use rot13::*;
