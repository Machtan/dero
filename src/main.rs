extern crate dero;
extern crate argonaut;

use argonaut::{Parse, ArgDef};

use std::io::{self, Write};
use std::process::{self, Command, Stdio};
use std::env;
use std::fs::OpenOptions;
use std::path::Path;
use std::error::Error;

#[cfg(target_os = "macos")]
fn copy_to_clipboard(text: &str) {
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
fn activate_anki() {
    Command::new("osascript")
        .arg("-e").arg("tell application \"Anki\"")
        .arg("-e").arg("Activate")
        .arg("-e").arg("end tell")
        .status().expect("Could not run AppleScript");
}

#[cfg(not(target_os = "macos"))]
fn activate_anki() {}

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

fn convert_single(text: &str, copy: bool, look_up: bool, append_file: Option<String>, anki: bool) -> bool {
    let output = dero::deromanize_escaped(text);
    println!("{}", &output);
    if copy {
        copy_to_clipboard(&output);
    }
    
    if look_up {
        look_up_word(&output);
    }
    
    if anki {
        activate_anki();
    }
    
    if let Some(ref file) = append_file {
        let path = Path::new(file);
        append_to_file(path, &output);
    }
    true
}

fn append_to_file(file: &Path, text: &str) {
    match OpenOptions::new().create(true).append(true).open(file) {
        Ok(mut file) => {
            match writeln!(file, "{}", text) {
                Ok(_) => {}
                Err(err) => {
                    eprintln!("Could not write to file: {}", err.description());
                }
            }
        }
        Err(err) => {
            eprintln!("Could not open file: {}", err.description());
        }
    }
}

const USAGE: &'static str = "Usage: dero [--help | OPTIONS]";

const HELP: &'static str = r#"Optional arguments:
  --look-up | -l TEXT   Deromanize TEXT and look up the result in the OS X
                        dictionary.
  --anki | -n           Activate ANKI after converting the text.
  --version             Show the version of dero.
  --help | -h           Show this help message.
  --no-copy             Do not copy the results to clipboard."#;

fn main() {
    use argonaut::Arg::*;

    let a_text_parts = ArgDef::optional_trail();
    let a_lookup = ArgDef::named_and_short("look-up", 'l').switch();
    let a_anki = ArgDef::named_and_short("anki", 'n').switch();
    let a_no_copy = ArgDef::named("no-copy").switch();
    let a_version = ArgDef::named("version").switch();
    let a_append = ArgDef::named_and_short("append-to-file", 'a').option();
    let a_help = ArgDef::named_and_short("help", 'h').switch();
    let expected = &[a_text_parts, a_append, a_anki, a_lookup, a_version, a_help, a_no_copy];

    let args: Vec<_> = env::args().skip(1).collect();
    let parse = Parse::new(expected, &args).expect("Invalid definitions");

    let mut parts = Vec::new();
    let mut copy_text = true;
    let mut look_up = false;
    let mut append_file = None;
    let mut anki = false;

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
            Ok(Switch("anki")) => anki = true,
            Ok(Switch("look-up")) => look_up = true,
            Ok(Switch("no-copy")) => {
                copy_text = false;
            }
            Ok(Option("append-to-file", value)) => {
                append_file = Some(value.to_string());
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
        dero::start_interactive_loop(&mut |hangeul| {
            if copy_text {
                copy_to_clipboard(&hangeul);
            }
            
            if look_up {
                look_up_word(&hangeul);
            }
            
            if anki {
                activate_anki();
            }
            
            if let Some(ref file) = append_file {
                let path = Path::new(file);
                append_to_file(path, &hangeul);
            }
        });
        return;
    } else {
        for part in parts {
            if convert_single(part, copy_text, look_up, append_file.clone(), anki) != true {
                process::exit(1);
            }
        }
    }
}

