pub struct Caesar;
pub struct Vernam;
pub struct Vigenere;
pub trait Cryptography<T> {
    fn encrypt(text: &str, key: T) -> String;
    fn decrypt(text: &str, key: T) -> String;
}

impl Cryptography<isize> for Caesar {
    /// Encrypt a text using the Caesar cipher using the given shift
    fn encrypt(text: &str, shift: isize) -> String {
        text.chars().map(|c| rotate_char(c, shift)).collect::<String>()
    }

    /// Decrypt a text using the Vigenere cipher using the given key
    fn decrypt(text: &str, shift: isize) -> String {
        Caesar::encrypt(text, -shift)
    }
    
}

impl Cryptography<&str> for Vernam {
    /// Encrypt a text using the Vernam cipher using the given key
    fn encrypt(text: &str, key: &str) -> String {
        // If text and key are not the same size, panic!
        if text.split_ascii_whitespace().collect::<String>().len()
        != key.split_ascii_whitespace().collect::<String>().len() {
            panic!("Text and key must be the same size");
        }
        
        let mut key = String::from(key);
        // Add whitespaces to the key at the same place as the text
        text.chars().enumerate().for_each(
            |(i, c)| {
                if c.is_whitespace() && !key.chars().nth(i).unwrap().is_whitespace() {
                    key.insert(i, ' ');
                }
            }
        );
        
        // Rotate each character based on the ith key
        text.chars().zip(key.chars()).map(|(c, k)| {
            rotate_char(c, k.to_ascii_lowercase() as isize - 'a' as isize)
        }).collect::<String>()
    }

    /// Decrypt a text using the Vernam cipher using the given key
    fn decrypt(text: &str, key: &str) -> String {
        println!("old key: {}", key);
        println!("reduction {}", 26 - 2*(key.chars().nth(0).unwrap().to_ascii_lowercase() as isize - 'a' as isize));
        // Calculate the negative rotation of the key
        let key = key.chars().inspect(|c| println!("{} = {}", c, c.to_ascii_lowercase() as isize - 'a' as isize)).map(|c| 
            rotate_char(c, 26 - 2*(c.to_ascii_lowercase() as isize - 'a' as isize)))
            .collect::<String>();
        println!("inverted key: {}", key);
        Vernam::encrypt(text, &key)
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