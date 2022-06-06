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

impl Vigenere {
    fn encrypt(text: &str, key: &str) -> String {
        let mut cipher = String::new();
        
        return cipher;
    }

    fn decrypt(text: &str, key: &str) -> String {
        let mut cipher = String::new();
        
        return cipher;
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
                    println!("{}", Vigenere::decrypt("text", &key));
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