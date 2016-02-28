#![feature(plugin)]
#![plugin(phf_macros)]
#![allow(unused)]

extern crate phf;
extern crate argonaut;

mod maps;
mod dero;

use std::io;
use std::env;
use std::process::Command;
use std::collections::{HashMap};
use argonaut::{Parser, Arg, StructuredArgument, generate_help};
use dero::deromanize;

fn copy_to_clipboard(text: &str) {
    println!("Copying '{}' to the clipboard...", text);
    //panic!("Needs to pipe it to stdin!");
    /*Command::new("pbcopy").arg(text)
        .output().unwrap_or_else(|e| {
            panic!("Could not copy text with pbcopy: {}", e)
        }
    );*/
}

static PUNCTUATION: phf::Set<char> = phf_set! {
    '.', ',', '\'', '"', '/', '\\', '?', '!', '#', '%', '-', '+', '(', ')', '[', ']', '{', '}',
    '@', '*', '&', ':', ';', '_', '^', '`', '~', '$', '|'
};
fn allow_punctuation(ch: char) -> bool {
    ch.is_whitespace() || PUNCTUATION.contains(&ch)
}

fn deromanize_single(text: &str) {
    use dero::DeromanizeError::*;
    match deromanize(text, allow_punctuation) {
        Ok(output) => {
            copy_to_clipboard(&output);
        },
        Err(error) => {
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
                    println!("Expected a valid consonant or vowel at position {}, found {}:",
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
    }
}

fn start_interactive(copy: bool) {
    println!("Welcome to the deromanization tool.");
    println!("Write romaja to convert it to hangeul.");
    let mut input = String::new();
    loop {
        print!("$ ");
        io::stdin().read_line(&mut input).unwrap();
        deromanize_single(&input);
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
    let a_pipe_mode = Arg::named("pipe-mode").interrupt()
        .add_help("Start the program in pipe mode, where it reads from stdin and prints the output to stdout.");
    let a_version = Arg::named("version").interrupt()
        .add_help("Show the version of this tool.");
    let a_help = Arg::named_and_short("help", 'h').interrupt()
        .add_help("Show this help message.");

    let mut parser = Parser::new();
    parser.define(&[a_text, a_pipe_mode, a_version, a_help]).unwrap();

    let mut parse = parser.parse(&args);
    for item in parse {
        match item {
            Err(err) => {
                println!("Parse error: {:?}", err);
                println!("{}", USAGE);
                return;
            },
            Ok(Single { name: "text", parameter }) => {
                return deromanize_single(parameter);
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
