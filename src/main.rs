#![feature(plugin)]
#![plugin(phf_macros)]
#![allow(unused)]

extern crate phf;
extern crate argonaut;

mod maps;
mod dero;

use std::io::{self, Write};
use std::env;
use std::process::{self, Command, Stdio};
use std::collections::{HashMap};
use argonaut::{Parser, Arg, StructuredArgument, generate_help};
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
    for i in 0..position {
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

fn start_interactive(copy: bool) {
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
    use argonaut::StructuredArgument::*;
    const USAGE: &'static str = "Usage: dero [--help | OPTIONS]";

    let arg_vec: Vec<_> = env::args().skip(1).collect();
    let mut args: Vec<&str> = Vec::new();
    for arg in arg_vec.iter() {
        args.push(arg);
    }

    let a_text = Arg::named_and_short("text", 't').single()
        .add_help("A single text string to deromanize.");
    let a_lookup = Arg::named_and_short("look-up", 'l').one_or_more()
        .add_help("One or more text strings to deromanize and look up with the \
        OSX dictionary. The strings are joined with a space before being looked \
        up");
    let a_pipe_mode = Arg::named("pipe-mode").interrupt()
        .add_help("Start the program in pipe mode, where it reads from stdin \
        and prints the output to stdout.");
    let a_version = Arg::named("version").interrupt()
        .add_help("Show the version of this tool.");
    let a_help = Arg::named_and_short("help", 'h').interrupt()
        .add_help("Show this help message.");

    let mut parser = Parser::new();
    parser.define(&[a_text, a_lookup, a_pipe_mode, a_version, a_help]).unwrap();

    let mut parse = parser.parse(&args);
    for item in parse {
        match item {
            Err(err) => {
                println!("Parse error: {:?}", err);
                println!("{}", USAGE);
                return;
            },
            Ok(Single { name: "text", parameter }) => {
                if deromanize_single(parameter) {
                    return;
                } else {
                    process::exit(1);
                }
            },
            Ok(Multiple { name: "look-up", parameters }) => {
                let mut text = String::new();
                let last = parameters.len() - 1;
                for i in 0..parameters.len() {
                    text.push_str(parameters[i]);
                    if i != last {
                        text.push(' ');
                    }
                }
                if deromanize_and_look_up(&text) {
                    return;
                } else {
                    process::exit(1);
                }
            },
            Ok(Interrupt { name: "pipe-mode" }) => {
                return println!("Reading stuff from stdin...");
            },
            Ok(Interrupt { name: "help" }) => {
                return println!("{}\n\n{}", USAGE, generate_help(&parser));
            },
            Ok(Interrupt { name: "version" }) => {
                return println!("{}", env!("CARGO_PKG_VERSION"));
            },
            _ => unreachable!(),
        }
    }

    start_interactive(true);
}
