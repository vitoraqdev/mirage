# mirage
Simple encryption tool written in Rust to encrypt and decrypt using simple methods
such as Ciphers, Vernam and Vigenere. It should be able to handle all characters, lower and uppercase, emojis,
special characters and etc. Mirage also has a logger function, based on the verbosity argument.


## Installation

To install mirage, clone the repository and run the following command:

`$ cargo build --release`

Then to run mirage, run the following command:

`$ cargo run -- [ARGS]`

## Usage
```
$ cargo run -- [-v] encrypt <caesar | vernam | vigenere> <message> <key>
$ cargo run -- [-v] decrypt <caesar | vernam | vigenere> <message> <key>
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

## Verbosity
There are 3 modes of verbosity: quiet, info, and debug. The program runs quietly by default, to run with info, type `-v` before the code, and to debug write `-vv`.
Using another example, by encrypting with Vigenere the message "Lorem" with key "ipsum" adding the verbosity flag

```
$ cargo run -- -v encrypt vernam Lorem ipsum  
INFO - Attempting to encrypt using Vernam: Lorem
INFO - With the key: ipsum
Tdjyy
```

```
$ cargo run -- -vv encrypt vigenere "This is a cool project!" "It sure is" 
INFO - Attempting to encrypt using Vigenere: This is a cool project!
INFO - With the key: It sure is
DEBUG - Key without space: itsureis
Baam zw i uwhd jisrwkm!
```
## Preview in Repl.it
<iframe frameborder="0" width="100%" height="500px" src="https://replit.com/@vitoraqdev/mirage?lite=true"></iframe>
