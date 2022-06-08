use clap::{Parser, Subcommand};
use mirage::Caesar;
use mirage::Vernam;
use mirage::Vigenere;
use mirage::Method;
use mirage::logger::DummyLogger;


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
        shift: isize,
    },   
}


fn main() {
    let args = Arguments::parse();
    let logger = DummyLogger::new(args.verbosity);
    match args.command {
        Commands::Encrypt { command } => {
            match command {
                Methods::Vigenere { message, key } => {
                    println!("{}", Vigenere::new(logger).encrypt(&message, &key));
                }
                Methods::Vernam { message, key } => {
                    println!("{}", Vernam::new(logger).encrypt(&message, &key));
                }
                Methods::Caesar { message, shift } => {
                    println!("{}", Caesar::new(logger).encrypt(&message, shift.to_string().as_str()));
                }
            }
        },
        Commands::Decrypt { command } => {
            match command {
                Methods::Vigenere { message, key } => {
                    println!("{}", Vigenere::new(logger).decrypt(&message, &key));
                }
                Methods::Vernam { message, key } => {
                    println!("{}", Vernam::new(logger).decrypt(&message, &key));
                }
                Methods::Caesar { message, shift } => {
                    println!("{}", Caesar::new(logger).decrypt(&message, shift.to_string().as_str()));
                }
            }
        },
    }

}
