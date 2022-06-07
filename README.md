# mirage
Simple encryption tool to encrypt and decrypt simple methods
such as Ciphers, Vernam and Vigenere.


## Installation

To install mirage, clone the repository and run the following command:\

`$ cargo build --release`\

Then to run mirage, run the following command:\

`$ cargo run -- [ARGS]`

## Usage
```
$ cargo run -- encrypt <caesar | vernam | vigenere> <key> <message>
$ cargo run -- decrypt <caesar | vernam | vigenere> <key> <message>
```
```
$ cargo run -- --help
mirage 0.1.0
vitoraqdev
A Very simple Encrypt and Decrypt program

USAGE:
    mirage.exe [OPTIONS] <SUBCOMMAND>

OPTIONS:
    -h, --help         Print help information
    -v, --verbosity
    -V, --version      Print version information

SUBCOMMANDS:
    decrypt
    encrypt
    help       Print this message or the help of the given subcommand(s)
```

## Examples
This uses the caesar cipher to encrypt the message "Hello World!" rotating by 3.

```
$ cargo run -- encrypt caesar "Hello world!" 3
Khoor zruog!
```

You can also decrypt if you have the key:
```
$ cargo run -- decrypt caesar "Khoor zruog!" 3
Hello world!
```

## Preview in Repl.it
<iframe frameborder="0" width="100%" height="500px" src="https://replit.com/@vitoraqdev/mirage?lite=true"></iframe>