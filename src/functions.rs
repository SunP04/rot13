use crate::{caesar_cipher::CaesarCipher, Cipher, Rot13};

/// Utilitary function for encoding using a Caesar Cipher.
/// With this function, it's not needed to create a new CaesarCipher instance.
///
/// ## Example:
/// ```rust
/// use rot13::functions::encode_caesar;
/// fn main() {
///    let word = String::from("Hello");
///    let steps = 13;
///    let encoded_word = encode_caesar(word, steps);
///    assert_eq!("Uryyb", encoded_word);
/// }
/// ```
pub fn encode_caesar(word: String, steps: usize) -> String {
    let caesar = CaesarCipher::new(word, steps);

    caesar.encode()
}

/// Utilitary function for decoding strings using a Caesar Cipher.
/// With this function, it's not needed to create a new CaesarCipher instance.
///
/// ## Example:
/// ```rust
/// use rot13::functions::decode_caesar;
/// fn main() {
///    let word = String::from("Uryyb");
///    let steps = 13;
///    let decoded_word = decode_caesar(word, steps);
///    assert_eq!("Hello", decoded_word);
/// }
/// ```
pub fn decode_caesar(word: String, steps: usize) -> String {
    let caesar = CaesarCipher::new(word, steps);

    caesar.decode()
}

/// Utilitary function for encoding using a Rot13 Cipher.
/// With this function, it's not needed to create a new Rot13Cipher instance.
///
/// ## Example:
/// ```rust
/// use rot13::functions::encode_rot13;
/// fn main() {
///    let word = String::from("Hello");
///    let encoded = encode_rot13(word);
///    assert_eq!("Uryyb", encoded);
/// }
/// ```
pub fn encode_rot13(word: String) -> String {
    let caesar = Rot13::new(word);

    caesar.decode()
}

/// Utilitary function for decoding using a Rot13 Cipher.
/// With this function, it's not needed to create a new Rot13Cipher instance.
///
/// ## Example:
/// ```rust
/// use rot13::functions::decode_rot13;
/// fn main() {
///    let word = String::from("Uryyb");
///    let decoded = decode_rot13(word);
///    assert_eq!("Hello", decoded);
/// }
/// ```
pub fn decode_rot13(word: String) -> String {
    let caesar = Rot13::new(word);

    caesar.decode()
}
