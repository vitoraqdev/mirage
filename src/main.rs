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