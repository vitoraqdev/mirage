use clap::{Parser, Subcommand};
use mirage::Caesar;
use mirage::Vernam;
use mirage::Vigenere;
use mirage::Cryptography;
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
        message: String,
        key: String,
    },
    Vernam {
        message: String,
        key: String,
    },
    Caesar {
        message: String,
        shift: i8,
    },   
}


fn main() {
    let args = Arguments::parse();
    match args.command {
        Commands::Encrypt { command } => {
            println!("Encrypting...");
            match command {
                Methods::Vigenere { message, key } => {
                    println!("{}", Vigenere::encrypt(&message, &key));
                }
                Methods::Vernam { message, key } => {
                    println!("{}", Vernam::encrypt(&message, &key));
                }
                Methods::Caesar { message, shift } => {
                    println!("{}", Caesar::encrypt(&message, shift));
                }
            }
        },
        Commands::Decrypt { command } => {
            println!("Decrypting...");
            match command {
                Methods::Vigenere { message, key } => {
                    println!("{}", Vigenere::decrypt(&message, &key));
                }
                Methods::Vernam { message, key } => {
                    
                }
                Methods::Caesar { message, shift } => {
                    println!("{}", Caesar::decrypt(&message, shift));
                }
            }
        },
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    
}  