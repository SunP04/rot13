use super::cipher::Cipher;
use super::constants as ctt;

/// Defines a base of a CeasarCipher.
/// Can be created manually or by calling CaesarCipher::new
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct CaesarCipher {
    /// The word that will be used in the cipher.
    pub word: String,
    /// The amount of steps the cipher will have to take.
    pub move_amount: usize,
}

impl CaesarCipher {
    /// Creates a new instance of a CaesarCipher.
    /// Alternative for creating a new Cipher manually.
    /// ```rs
    /// use rot13::CaesarCipher;
    /// fn main() {
    ///    let cipher = CaesarCipher::new("Hello".into(), 13);
    ///    assert_eq!("Hello", cipher.word);
    ///    assert_eq!(13, cipher.move_amount);
    /// }
    /// ```
    pub fn new(word: String, move_amount: usize) -> Self {
        Self { word, move_amount }
    }

    /// Runs the encription for the given string.
    /// User may call it by using the rot13::Cipher trait.
    fn run_encryption(&self) -> String {
        self.word
            .chars()
            .map(|letter| match letter {
                'A'..='Z' => self.add_encoded_to_string(
                    letter,
                    ctt::ASCII_UPPERCASE_Z,
                    ctt::ASCII_UPPERCASE_A,
                ),
                'a'..='z' => self.add_decoded_to_string(
                    letter,
                    ctt::ASCII_LOWERCASE_Z,
                    ctt::ASCII_LOWERCASE_A,
                ),
                _ => letter,
            })
            .collect()
    }

    fn add_encoded_to_string(&self, value: char, max_value: usize, lowest_value: usize) -> char {
        let ascii_value = (value as u32) as usize;
        let ascii_value_added = ascii_value + self.move_amount;

        if ascii_value_added > max_value {
            let rest = ascii_value_added - max_value;
            ((lowest_value + rest - 1) as u8) as char
        } else {
            (ascii_value_added as u8) as char
        }
    }

    fn add_decoded_to_string(&self, value: char, max_value: usize, lowest_value: usize) -> char {
        let ascii_value = (value as u32) as usize;
        let ascii_value_lowered = ascii_value - self.move_amount;
        if ascii_value_lowered < lowest_value {
            let rest = lowest_value - ascii_value_lowered;
            (max_value - rest + 1) as u8 as char
        } else {
            (ascii_value_lowered as u8) as char
        }
    }
}

impl From<(String, usize)> for CaesarCipher {
    fn from((word, move_amount): (String, usize)) -> Self {
        Self::new(word, move_amount)
    }
}

impl Cipher for CaesarCipher {
    type Output = String;
    fn encode(&self) -> Self::Output {
        self.run_encryption()
    }

    fn decode(&self) -> Self::Output {
        self.run_encryption()
    }
}

impl std::ops::Add<usize> for CaesarCipher {
    type Output = CaesarCipher;

    fn add(self, rhs: usize) -> Self::Output {
        CaesarCipher::new(self.word, self.move_amount + rhs)
    }
}

impl std::ops::Sub<usize> for CaesarCipher {
    type Output = CaesarCipher;

    fn sub(self, rhs: usize) -> Self::Output {
        CaesarCipher::new(self.word, self.move_amount - rhs)
    }
}

impl std::ops::Mul<usize> for CaesarCipher {
    type Output = CaesarCipher;

    fn mul(self, rhs: usize) -> Self::Output {
        CaesarCipher::new(self.word, self.move_amount * rhs)
    }
}

impl std::ops::Div<usize> for CaesarCipher {
    type Output = CaesarCipher;

    fn div(self, rhs: usize) -> Self::Output {
        CaesarCipher::new(self.word, self.move_amount / rhs)
    }
}
