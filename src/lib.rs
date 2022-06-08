pub mod logger;
use logger::DummyLogger;

pub struct Caesar { logger: DummyLogger }
pub struct Vernam { logger: DummyLogger }
pub struct Vigenere { logger: DummyLogger }
pub trait Method {
    fn new(logger: logger::DummyLogger) -> Self;
    fn encrypt(&self, text: &str, key: &str) -> String;
    fn decrypt(&self, text: &str, key: &str) -> String;
}

impl Method for Caesar {
    fn new(logger: logger::DummyLogger) -> Self {
        Caesar { logger }
    }
    /// Encrypt a text using the Caesar cipher using the given shift
    fn encrypt(&self, text: &str, shift: &str) -> String {
        self.logger.info(format!("Attempting to encrypt using Caesar: {}", text));
        self.logger.info(format!("With the shift: {}", shift));

        let shift = shift.parse::<isize>().unwrap();

        text.chars().map(|c| rotate_char(c, shift)).collect::<String>()
    }

    /// Decrypt a text using the Vigenere cipher using the given key
    fn decrypt(&self, text: &str, shift: &str) -> String {
        self.logger.info(format!("Attempting to decrypt using Caesar: {}", text));
        self.logger.info(format!("With the shift: {}", shift));
        
        // Invert the sign of the shift
        let shift = shift.parse::<isize>().unwrap();
        let shift = (-shift).to_string();

        // self.logger.debug(format!("Shift is {}", shift));
        self.encrypt(text, shift.as_str())
    }
    
}

impl Method for Vernam {
    fn new(logger: logger::DummyLogger) -> Self {
        Vernam { logger }
    }
    /// Encrypt a text using the Vernam cipher using the given key
    fn encrypt(&self, text: &str, key: &str) -> String {
        self.logger.info(format!("Attempting to encrypt using Vernam: {}", text));
        self.logger.info(format!("With the key: {}", key));

        let text_length = text.chars().filter(|c| c.is_ascii_alphabetic()).collect::<String>().len();
        let key_length = key.chars().filter(|c| c.is_ascii_alphabetic()).collect::<String>().len();

        // If text and key are not the same size, panic!
        if text_length != key_length {
            panic!("Text and key must be the same size, \n\
                    text length: {}, \n\
                    key length: {}", text_length, key_length);
        }
        
        // Strip all whitespace from the key
        let key: String = key.to_lowercase().split_ascii_whitespace().collect();
        
        // If the key has length 0, panic!
        if key.len() == 0 {
            panic!("Key must not be empty");
        }

        // If the key has not alphabetic characters, panic!
        if key.chars().any(|c| !c.is_ascii_alphabetic()) {
            panic!("Key must only contain alphabetic characters");
        }

        let mut key_index = 0;
        let cipher = text
            .chars()
            .map(|c| {
                // If the character is a whitespace, return it
                if !c.is_alphabetic() { return c }
                // // Get the index of the character in the key
                // let i = key.chars().position(|k| k == c).unwrap();
                key_index += 1;
                // Rotate the character based on the key
                rotate_char(c, key.chars().collect::<Vec<char>>()[(key_index - 1) % key.len()] as isize - 'a' as isize)
            }).collect();

        // self.logger.debug(format!("Cipher: {}", cipher));
        cipher
    }

    /// Decrypt a text using the Vernam cipher using the given key
    fn decrypt(&self, text: &str, key: &str) -> String {
        self.logger.info(format!("Attempting to decrypt using Vernam: {}", text));
        self.logger.info(format!("With the key: {}", key));
        self.logger.debug(format!("Reduction {}", 26 - 2*(key.chars().nth(0).unwrap().to_ascii_lowercase() as isize - 'a' as isize)));
        
        // Calculate the negative rotation of the key
        let key = key
            .chars()
            .inspect(|c| self.logger.debug(
                format!("{} = {}", c, c.to_ascii_lowercase() as isize - 'a' as isize)))
            .map(|c| 
            rotate_char(c, 26 - 2*(c.to_ascii_lowercase() as isize - 'a' as isize)))
            .collect::<String>();

        // self.logger.debug(format!("Key: {}", key));

        self.encrypt(text, &key)
    }
    
}

impl Method for Vigenere {
    fn new(logger: logger::DummyLogger) -> Self {
        Vigenere { logger }
    }

    /// Encrypt a text using the Vigenere cipher using the given key
    fn encrypt(&self, text: &str, key: &str) -> String {
        self.logger.info(format!("Attempting to encrypt using Vigenere: {}", text));
        self.logger.info(format!("With the key: {}", key));

        // Strip all whitespace from the key
        let key: String = key.to_lowercase().split_ascii_whitespace().collect();
        self.logger.debug(format!("Key without space: {}", key));

        // If the key has length 0, panic!
        if key.len() == 0 {
            panic!("Key must not be empty");
        }

        // If the key has not alphabetic characters, panic!
        if key.chars().any(|c| !c.is_ascii_alphabetic()) {
            panic!("Key must only contain alphabetic characters");
        }
        
        let mut key_index = 0;
        let cipher = text
            .chars()
            .map(|c| {
                // If the character is a whitespace, return it
                if !c.is_alphabetic() { return c }
                // // Get the index of the character in the key
                // let i = key.chars().position(|k| k == c).unwrap();
                key_index += 1;
                // Rotate the character based on the key
                rotate_char(c, key.chars().collect::<Vec<char>>()[(key_index - 1) % key.len()] as isize - 'a' as isize)
            }).collect();

        // self.logger.debug(format!("Vigenere cipher: {}", cipher));
        cipher

    }

    /// Decrypt a text using the Vigenere cipher using the given key
    fn decrypt(&self, text: &str, key: &str) -> String {
        self.logger.info(format!("Attempting to decrypt using Vigenere: {}", text));
        self.logger.info(format!("With the key: {}", key));

        // Strip all whitespace from the key
        let key: String = key.to_lowercase().split_ascii_whitespace().collect();

        // If the key has length 0, panic!
        if key.len() == 0 {
            panic!("Key must not be empty");
        }

        // If the key has not alphabetic characters, panic!
        if key.chars().any(|c| !c.is_ascii_alphabetic()) {
            panic!("Key must only contain alphabetic characters");
        }
        
        self.logger.debug(format!("Reduction {}", 26 - 2*(key.chars().nth(0).unwrap().to_ascii_lowercase() as isize - 'a' as isize)));
        
        // Calculate the negative rotation of the key
        let key = key
            .chars()
            .inspect(|c| println!("{} = {}", c, c.to_ascii_lowercase() as isize - 'a' as isize))
            .map(|c| rotate_char(c, 26 - 2*(c.to_ascii_lowercase() as isize - 'a' as isize)))
            .collect::<String>();
        
        self.logger.debug(format!("Inverted key: {}", key));

        self.encrypt(text, &key)
    }
}

/// Rotates a character by a given amount,
/// if the character is not a letter, it will return the same character
fn rotate_char(c: char, shift: isize) -> char {
    // println!("Rotating the char: {}", c); // Change to debug
    
    // If the shift is negative then change it the congruent positive
    let shift = match shift {
        x if x < 0 => ((x % 26) + 26) as u8,
        x => (x % 26) as u8,
    };

    // println!("Shifting {} by {}", c, shift); // Change to debug
    match c {
        'a'..='z' => (((c as u8) - 97 + shift) % 26 + 97) as char,
        'A'..='Z' => (((c as u8) - 65 + shift) % 26 + 65) as char,
        character => character,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(test)]
    mod tests_rotate_char {
        use super::*;
        
        #[test]
        fn test_rotate_char_by_zero() {
            assert_eq!(rotate_char('a', 0), 'a');
        }
        #[test]
        fn test_rotate_char_by_one() {
            assert_eq!(rotate_char('a', 1), 'b');
        }
        #[test]
        fn test_rotate_char_by_twenty_six() {
            assert_eq!(rotate_char('a', 26), 'a');
        }
        #[test]
        fn test_rotate_char_by_twenty_seven() {
            assert_eq!(rotate_char('a', 27), 'b');
        }
        #[test]
        fn test_rotate_uppercase_char_by_one() {
            assert_eq!(rotate_char('A', 1), 'B');
        }
        #[test]
        fn test_rotate_uppercase_char_by_twenty_six() {
            assert_eq!(rotate_char('A', 26), 'A');
        }
        #[test]
        fn test_rotate_uppercase_char_by_twenty_seven() {
            assert_eq!(rotate_char('A', 27), 'B');
        }
        #[test]
        fn test_rotate_char_by_negative_one() {
            assert_eq!(rotate_char('a', -1), 'z');
        }
        #[test]
        fn test_rotate_char_by_negative_twenty_seven() {
            assert_eq!(rotate_char('a', -27), 'z');
        }
        #[test]
        fn test_rotate_not_a_letter() {
            assert_eq!(rotate_char('!', 1), '!');
        }
        #[test]
        fn test_rotate_not_a_letter_2() {
            assert_eq!(rotate_char('#', 1), '#');
        }
    }
}