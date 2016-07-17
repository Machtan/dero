#![feature(plugin)]
#![plugin(phf_macros)]

extern crate argonaut;
extern crate dero;
extern crate phf;

use std::env;
use std::io::{self, Write};
use std::process::{self, Command, Stdio};

use argonaut::{Parse, ArgDef};

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

#[cfg(target_os = "macos")]
fn look_up_word(text: &str) {
    let url = format!("dict://{}", &text);
    Command::new("open")
        .arg(&url)
        .status()
        .expect("Could not open dictionary app");
}

#[cfg(not(target_os = "macos"))]
fn copy_to_clipboard(text: &str) {}

#[cfg(not(target_os = "macos"))]
fn look_up_word(text: &str) {}

fn convert_single(text: &str, copy: bool, look_up: bool) -> bool {
    match dero::convert(text) {
        Ok(output) => {
            println!("{}", &output);
            if copy {
                copy_to_clipboard(&output);
            }
            if look_up {
                look_up_word(&output);
            }
            true
        }
        Err(error) => {
            error.print_explanation(text).expect("Could not print error");
            false
        }
    }
}

fn start_interactive(copy: bool, look_up: bool) {
    println!("Welcome to the deromanization tool.");
    println!("Write romaja to convert it to 한글.");
    println!("( Press Ctrl + C to quit )");
    let mut input = String::new();
    loop {
        input.clear();
        print!("> ");
        io::stdout().flush().expect("Could not flush stdout");
        if io::stdin().read_line(&mut input).expect("Could not read from stdin") == 0 {
            // End of file reached.
            println!("");
            return;
        }
        match dero::convert(&input) {
            Ok(output) => {
                let trimmed = output.trim_right();
                if copy {
                    copy_to_clipboard(trimmed);
                }
                if look_up {
                    look_up_word(trimmed);
                }
                println!("=> {}", trimmed);
            }
            Err(error) => {
                error.print_explanation(&input).expect("Could not print error");
            }
        }
    }
}

const USAGE: &'static str = "Usage: dero [--help | OPTIONS]";

const HELP: &'static str = r#"Optional arguments:
  --look-up | -l TEXT   Deromanize TEXT and look up the result in the OS X
                        dictionary.
  --version             Show the version of dero.
  --help | -h           Show this help message.
  --no-copy             Do not copy the results to clipboard."#;

fn main() {
    use argonaut::Arg::*;

    let a_text_parts = ArgDef::optional_trail();
    let a_lookup = ArgDef::named_and_short("look-up", 'l').switch();
    let a_no_copy = ArgDef::named("no-copy").switch();
    let a_version = ArgDef::named("version").switch();
    let a_help = ArgDef::named_and_short("help", 'h').switch();
    let expected = &[a_text_parts, a_lookup, a_version, a_help, a_no_copy];

    let args: Vec<_> = env::args().skip(1).collect();
    let parse = Parse::new(expected, &args).expect("Invalid definitions");

    let mut parts = Vec::new();
    let mut copy_text = true;
    let mut look_up = false;

    for item in parse {
        match item {
            Err(err) => {
                // TODO: Do not use Debug print of the error.
                let msg = format!("Parse error: {:?}\n{}\n{}\n",
                                  err,
                                  USAGE,
                                  "Try --help for more information.");
                io::stderr().write(msg.as_bytes()).expect("Could not print error");
                process::exit(2);
            }
            Ok(TrailPart(value)) => {
                parts.push(value);
            }
            Ok(Switch("look-up")) => look_up = true,
            Ok(Switch("no-copy")) => {
                copy_text = false;
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

    if parts.is_empty() {
        start_interactive(copy_text, look_up);
        return;
    }

    for part in parts {
        if convert_single(part, copy_text, look_up) != true {
            process::exit(1);
        }
    }
}
