pub struct Caesar;
pub struct Vernam;
pub struct Vigenere;
pub trait Cryptography<T> {
    fn encrypt(text: &str, key: T) -> String;
    fn decrypt(text: &str, key: T) -> String;
}

impl Cryptography<i8> for Caesar {
    /// Encrypt a text using the Caesar cipher using the given shift
    fn encrypt(text: &str, shift: i8) -> String {
        text.chars().map(|c| rotate_char(c, shift)).collect::<String>()
    }

    /// Decrypt a text using the Vigenere cipher using the given key
    fn decrypt(text: &str, shift: i8) -> String {
        Caesar::encrypt(text, -shift)
    }
    
}

impl Cryptography<&str> for Vernam {
    /// Encrypt a text using the Vernam cipher using the given key
    fn encrypt(text: &str, key: &str) -> String {
        let mut cipher = String::new();
        
        return cipher;
    }

    /// Decrypt a text using the Vernam cipher using the given key
    /// If not given the key, it will brute force it
    fn decrypt(text: &str, key: &str) -> String {
        todo!()
    }
    
}

impl Cryptography<&str> for Vigenere {
    /// Encrypt a text using the Vigenere cipher using the given key
    fn encrypt(text: &str, key: &str) -> String {
        let mut cipher = String::new();
        
        return cipher;
    }

    /// Decrypt a text using the Vigenere cipher using the given key
    /// If not given the key, it will brute force it
    fn decrypt(text: &str, key: &str) -> String {
        let mut cipher = String::new();
        
        return cipher;
    }
}

/// Rotates a character by a given amount,
/// if the character is not a letter, it will return the same character
fn rotate_char(c: char, shift: i8) -> char {
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