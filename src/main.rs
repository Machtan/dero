#![feature(plugin)]
#![plugin(phf_macros)]

extern crate phf;
extern crate argonaut;

mod maps;
mod dero;

use std::io::{self, Write};
use std::borrow::Borrow;
use std::env;
use std::process::{self, Command, Stdio};
use argonaut::{Parse, ArgDef};
use dero::{deromanize, DeromanizeError};

const MIN_BLOCK_VALUE: u32 = 44032;
const MAX_BLOCK_VALUE: u32 = 55203;

fn copy_to_clipboard(text: &str) {
    if cfg!(target_os = "macos") {
        //println!("Copying '{}' to the clipboard...", text);
        let mut child = Command::new("pbcopy").arg(text).stdin(Stdio::piped()).spawn()
            .expect("Could not run pbcopy");
        if let Some(ref mut stdin) = child.stdin {
            stdin.write_all(text.as_bytes())
                .expect("Could not write to pbcopy");
        } else { 
            unreachable!();
        }
        child.wait().expect("Error while running pbcopy");
    } else {
        println!("{}", text);
    }
}

static PUNCTUATION: phf::Set<char> = phf_set! {
    '.', ',', '\'', '"', '/', '\\', '?', '!', '#', '%', '-', '+', 
    '(', ')', '[', ']', '{', '}',
    '@', '*', '&', ':', ';', '_', '^', '`', '~', '$', '|'
};

#[inline(always)]
fn is_punctuation(ch: char) -> bool {
    PUNCTUATION.contains(&ch)
}

#[inline(always)]
fn is_korean(ch: char) -> bool {
    MIN_BLOCK_VALUE <= (ch as u32) && (ch as u32) <= MAX_BLOCK_VALUE
}

#[inline(always)]
fn is_whitespace(ch: char) -> bool {
    ch.is_whitespace()
}

#[inline(always)]
fn is_common(ch: char) -> bool {
    is_whitespace(ch) || is_punctuation(ch) || is_korean(ch)
}

fn print_error(error: DeromanizeError, text: &str) {
    use dero::DeromanizeError::*;
    let position = match error {
        InvalidConsonant { letter, position } => {
            println!("Expected a valid consonant at position {}, found {}:",
                position + 1, letter);
            position
        },
        InvalidVowel { letter, position } => {
            println!("Expected a valid vowel at position {}, found {}:",
                position + 1, letter);
            position
        },
        InvalidLetter { letter, position } => {
            println!("Expected a valid consonant or vowel at position \
                {}, found {}:",
                position + 1, letter);
            position
        },
        MissingFinalVowel { position } => {
            println!("Expected a vowel at position {}", position + 1);
            position
        }
    };
    //let mut example: String = text.chars().take(position+1).collect();
    //println!("{}", example);
    println!("{}", text);
    let mut pointer = String::new();
    for _ in 0..position {
        pointer.push('~');
    }
    pointer.push('^');
    println!("{}", pointer);
}

fn deromanize_and_look_up(text: &str) -> bool {
    match deromanize(text, is_common) {
        Ok(output) => {
            println!("{}", &output);
            let url = format!("dict://{}", &output);
            let status = Command::new("open").arg(&url).status()
                .expect("Could not open dictionary app");
            status.success()
        },
        Err(error) => {
            print_error(error, text);
            false
        }
    }
}

fn deromanize_single(text: &str) -> bool {
    match deromanize(text, is_common) {
        Ok(output) => {
            copy_to_clipboard(&output);
            println!("{}", &output);
            true
        },
        Err(error) => {
            print_error(error, text);
            false
        }
    }
}

fn start_interactive(_copy: bool) {
    println!("Welcome to the deromanization tool.");
    println!("Write romaja to convert it to 한글.");
    println!("( Press Ctrl + C to quit )");
    let mut input = String::new();
    loop {
        input.clear();
        print!("> ");
        io::stdout().flush().expect("Could not flush stdout");
        io::stdin().read_line(&mut input).unwrap();
        match deromanize(&input, is_common) {
            Ok(output) => {
                copy_to_clipboard(&output);
                print!("=> {}", output);
            },
            Err(error) => {
                print_error(error, &input);
            }
        }
    }
}

fn main() {
    use argonaut::Arg::*;
    const USAGE: &'static str = "Usage: dero [--help | OPTIONS]";
    
    const HELP: &'static str = "\
Optional arguments:
--text | -t     A single text string to deromanize.
--look-up | -l  A text string to deromanize and look up with the OSX dictionary.
--version       Show the version of this tool.
--help | -h     Show this help message.\
    ";

    let a_text = ArgDef::named_and_short("text", 't').option();
    let a_lookup = ArgDef::named_and_short("look-up", 'l').option();
    let a_version = ArgDef::named("version").switch();
    let a_help = ArgDef::named_and_short("help", 'h').switch();
    let expected = &[a_text, a_lookup, a_version, a_help];
    
    let args: Vec<_> = env::args().skip(1).collect();
    let mut parse = Parse::new(expected, &args).expect("Invalid definitions");
    while let Some(item) = parse.next() {
        match item {
            Err(err) => {
                println!("Parse error: {:?}", err);
                println!("{}", USAGE);
                return;
            },
            Ok(Option ("text", value)) => {
                if deromanize_single(value.borrow()) {
                    return;
                } else {
                    process::exit(1);
                }
            },
            Ok(Option("look-up", value)) => {
                if deromanize_and_look_up(value.borrow()) {
                    return;
                } else {
                    process::exit(1);
                }
            },
            Ok(Switch("help")) => {
                return println!("{}\n\n{}", USAGE, HELP);
            },
            Ok(Switch("version")) => {
                return println!("{}", env!("CARGO_PKG_VERSION"));
            },
            _ => unreachable!(),
        }
    }

    start_interactive(true);
}
