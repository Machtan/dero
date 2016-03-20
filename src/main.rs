#![feature(plugin)]
#![plugin(phf_macros)]

extern crate phf;
extern crate argonaut;

mod maps;
mod dero;

use std::env;
use std::io::{self, Write};
use std::process::{self, Command, Stdio};

use argonaut::{Parse, ArgDef};

use dero::{DeromanizeError, deromanize};

const MIN_BLOCK_VALUE: char = '\u{44032}';
const MAX_BLOCK_VALUE: char = '\u{55203}';

#[cfg(target_os = "macos")]
fn copy_to_clipboard(text: &str) {
    // println!("Copying '{}' to the clipboard...", text);
    let mut child = Command::new("/usr/bin/pbcopy")
                        .arg(text)
                        .stdin(Stdio::piped())
                        .spawn()
                        .expect("Could not run pbcopy");
    if let Some(ref mut stdin) = child.stdin {
        stdin.write_all(text.as_bytes())
             .expect("Could not write to pbcopy");
    } else {
        unreachable!();
    }
    child.wait().expect("Error while running pbcopy");
}

#[cfg(not(target_os = "macos"))]
fn copy_to_clipboard(text: &str) {}

static PUNCTUATION: phf::Set<char> = phf_set! {
    '.', ',', '\'', '"', '/', '\\', '?', '!', '#', '%', '-', '+',
    '(', ')', '[', ']', '{', '}',
    '@', '*', '&', ':', ';', '_', '^', '`', '~', '$', '|'
};

#[inline]
fn is_punctuation(ch: char) -> bool {
    PUNCTUATION.contains(&ch)
}

#[inline]
fn is_korean(ch: char) -> bool {
    MIN_BLOCK_VALUE <= ch && ch <= MAX_BLOCK_VALUE
}

#[inline]
fn is_whitespace(ch: char) -> bool {
    ch.is_whitespace()
}

#[inline]
fn is_common(ch: char) -> bool {
    is_whitespace(ch) || is_punctuation(ch) || is_korean(ch)
}

fn print_error(error: DeromanizeError, text: &str) -> io::Result<()> {
    // Output '^' padded with '~' to the length given by argument 3.
    let msg = format!("{}\n{}{:~>3$}\n",
                      error,
                      text,
                      '^',
                      error.char_position() + 1);
    io::stderr().write_all(msg.as_bytes())
}

fn deromanize_and_look_up(text: &str) -> bool {
    match deromanize(text, is_common) {
        Ok(output) => {
            println!("{}", &output);
            let url = format!("dict://{}", &output);
            let status = Command::new("open")
                             .arg(&url)
                             .status()
                             .expect("Could not open dictionary app");
            status.success()
        }
        Err(error) => {
            print_error(error, text).expect("Could not print error");
            false
        }
    }
}

fn deromanize_single(text: &str) -> bool {
    match deromanize(text, is_common) {
        Ok(output) => {
            copy_to_clipboard(output.trim_right());
            println!("{}", &output);
            true
        }
        Err(error) => {
            print_error(error, text).expect("Could not print error");
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
        if io::stdin().read_line(&mut input).unwrap() == 0 {
            // End of file reached.
            println!("");
            return;
        }
        match deromanize(&input, is_common) {
            Ok(output) => {
                let trimmed = output.trim_right();
                if copy {
                    copy_to_clipboard(trimmed);
                }
                println!("=> {}", trimmed);
            }
            Err(error) => {
                print_error(error, &input).expect("Could not print error");
            }
        }
    }
}

fn main() {
    use argonaut::Arg::*;

    const USAGE: &'static str = "Usage: dero [--help | OPTIONS]";

    const HELP: &'static str = "Optional arguments:
  --text | -t TEXT    Deromanize TEXT.
  \
                                --look-up | -l TEXT Deromanize and then look TEXT up in the OS X \
                                dictionary.
  --version           Show the version of this tool.
  \
                                --help | -h         Show this help message.
  --no-copy           \
                                Do not copy the results to clipboard.";

    let a_text = ArgDef::named_and_short("text", 't').option();
    let a_lookup = ArgDef::named_and_short("look-up", 'l').option();
    let a_version = ArgDef::named("version").switch();
    let a_help = ArgDef::named_and_short("help", 'h').switch();
    let a_no_copy = ArgDef::named("no-copy").switch();
    let expected = &[a_text, a_lookup, a_version, a_help, a_no_copy];

    let args: Vec<_> = env::args().skip(1).collect();
    let mut parse = Parse::new(expected, &args).expect("Invalid definitions");

    let mut modes = Vec::new();
    let mut interactive_copy = true;

    for item in &mut parse {
        match item {
            Err(err) => {
                // TODO: Do not use Debug print of the error.
                let msg = format!("Parse error: {:?}\n{}\n", err, USAGE);
                io::stderr().write(msg.as_bytes()).expect("Could not print error");
                process::exit(2);
            }
            Ok(Option("text", value)) => {
                // look_up = false
                modes.push((false, value));
            }
            Ok(Option("look-up", value)) => {
                // look_up = true
                modes.push((true, value));
            }
            Ok(Switch("no-copy")) => {
                interactive_copy = false;
            }
            Ok(Switch("help")) => {
                println!("{}\n\n{}", USAGE, HELP);
                return;
            }
            Ok(Switch("version")) => {
                println!("dero {}", env!("CARGO_PKG_VERSION"));
                return;
            }
            _ => unreachable!(),
        }
    }

    if modes.is_empty() {
        start_interactive(interactive_copy);
        return;
    }

    for (look_up, value) in modes {
        let ok = if look_up {
            deromanize_single(&value)
        } else {
            deromanize_and_look_up(&value)
        };

        if !ok {
            process::exit(1);
        }
    }
}
