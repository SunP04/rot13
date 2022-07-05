use super::constants as ctt;

#[derive(Debug)]
pub struct CaesarCipher {
    pub word: String,
    move_amount: usize,
}

impl CaesarCipher {
    pub fn new(word: String, move_amount: usize) -> Self {
        Self { word, move_amount }
    }

    pub fn encode(&self) -> String {
        let mut out = String::new();
        self.run_encryption(&mut out, |a, b, c| self.add_encoded_to_string(a, b, c));

        out
    }

    pub fn decode(&self) -> String {
        let mut out = String::new();
        self.run_encryption(&mut out, |x, y, z| self.add_decoded_to_string(x, y, z));
        out
    }

    fn run_encryption<F>(&self, out: &mut String, func: F)
    where
        F: Fn(char, usize, usize) -> char,
    {
        let value: String = self
            .word
            .chars()
            .map(|letter| match letter {
                'A'..='Z' => func(letter, ctt::ASCII_UPPERCASE_Z, ctt::ASCII_UPPERCASE_A),
                'a'..='z' => func(letter, ctt::ASCII_LOWERCASE_Z, ctt::ASCII_LOWERCASE_A),
                _ => letter,
            })
            .collect();

        out.push_str(&value);
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
