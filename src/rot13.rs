use super::CaesarCipher;
use super::Cipher;

/// Struct for encoding and decoding strings using the Rot13 cipher.
/// This struct can be created manually or by calling Rot13::new
pub struct Rot13 {
    /// This string will be the one encoded or decoded.
    pub word: String,
    cipher: CaesarCipher,
}

impl Rot13 {
    /// Creates a new instance of a Rot13 Cipher.
    /// Alternative for creating a new Cipher manually.
    /// ```rs
    /// use rot13::Rot13;
    /// fn main() {
    ///    let cipher = Rot13::new("Hello".into());
    ///    assert_eq!("Hello", cipher.word);
    /// }
    /// ```
    pub fn new(word: String) -> Self {
        let copied_word = &word;
        Self {
            word: copied_word.to_string(),
            cipher: CaesarCipher::new(copied_word.to_owned(), 13),
        }
    }
}

impl Default for Rot13 {
    fn default() -> Self {
        Self::new(String::from(""))
    }
}

impl Cipher for Rot13 {
    type Output = String;
    fn encode(&self) -> Self::Output {
        self.cipher.encode()
    }

    fn decode(&self) -> Self::Output {
        self.cipher.decode()
    }
}
