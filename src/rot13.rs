use super::CaesarCipher;
use super::Cipher;

pub struct Rot13 {
    pub word: String,
    cipher: CaesarCipher,
}

impl Rot13 {
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
