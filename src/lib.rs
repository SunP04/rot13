use caesar_cipher::CaesarCipher;

pub mod caesar_cipher;
pub mod constants;

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

    pub fn encode(&self) -> String {
        self.cipher.encode()
    }

    pub fn decode(&self) -> String {
        self.cipher.decode()
    }
}

impl Default for Rot13 {
    fn default() -> Self {
        Self::new(String::from(""))
    }
}