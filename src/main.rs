use clap::{Parser, Subcommand};
mod logger;


#[derive(Parser, Debug)]
#[clap(author = "vitoraqdev", version, about)]
/// A Very simple Encrypt and Decrypt program
struct Arguments {
    #[clap(short, long, parse(from_occurrences))]
    verbosity: usize,
    
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Encrypt {
        #[clap(subcommand)]
        command: Methods,
    },
    Decrypt {
        #[clap(subcommand)]
        command: Methods,
    },
}

#[derive(Subcommand, Debug)]
enum Methods {
    Vigenere {
        key: String,
    },
    Vernam {
        key: String,
    },
    Caesar {
        shift: u8,
    },   
}

struct Vigenere;
struct Vernam;
struct Caesar;
trait Cryptography<T> {
    fn encrypt(text: &str, key: T) -> String;
    fn decrypt(text: &str, key: Option<T>) -> String;
}

impl Cryptography<&str> for Vigenere {
    /// Encrypt a text using the Vigenere cipher using the given key
    fn encrypt(text: &str, key: &str) -> String {
        let mut cipher = String::new();
        
        return cipher;
    }

    /// Decrypt a text using the Vigenere cipher using the given key
    /// If not given the key, it will brute force it
    fn decrypt(text: &str, key: Option<&str>) -> String {
        let mut cipher = String::new();
        
        return cipher;
    }
}

impl Cryptography<&str> for Vernam {
    /// Encrypt a text using the Vernam cipher using the given key
    fn encrypt(text: &str, key: &str) -> String {
        let mut cipher = String::new();
        
        return cipher;
    }

    fn decrypt(text: &str, key: Option<&str>) -> String {
        todo!()
    }
    
}

impl Cryptography<u8> for Caesar {
    /// Encrypt a text using the Caesar cipher using the given key
    fn encrypt(text: &str, key: u8) -> String {
        let mut cipher = String::new();
        
        return cipher;
    }

    fn decrypt(text: &str, key: Option<u8>) -> String {
        todo!()
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

fn main() {
    let args = Arguments::parse();

    match args.command {
        Commands::Encrypt { command } => {
            println!("Encrypting...");
            match command {
                Methods::Vigenere { key } => {
                    println!("{}", Vigenere::encrypt("text", &key));
                }
                Methods::Vernam { key } => {
                    todo!()
                }
                Methods::Caesar { shift } => {
                    todo!()
                }
            }
        },
        Commands::Decrypt { command } => {
            println!("Decrypting...");
            match command {
                Methods::Vigenere { key } => {
                    println!("{}", Vigenere::decrypt("text", Some(&key)));
                }
                Methods::Vernam { key } => {
                    todo!()
                }
                Methods::Caesar { shift } => {
                    todo!()
                }
            }
        },
    }

}

#[cfg(test)]
mod tests {
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