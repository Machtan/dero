extern crate termion;
extern crate dero;
extern crate argonaut;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use argonaut::{Parse, ArgDef};

use std::io::{self, Write, stdout, stdin};
use std::process::{self, Command, Stdio};
use std::env;

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
    let output = dero::deromanize_escaped(text);
    println!("{}", &output);
    if copy {
        copy_to_clipboard(&output);
    }
    if look_up {
        look_up_word(&output);
    }
    true
}


pub struct History {
    inputs: Vec<String>,
}

impl History {
    pub fn new() -> History {
        History {
            inputs: Vec::new()
        }
    }
    
    pub fn len(&self) -> usize {
        self.inputs.len()
    }
    
    pub fn push(&mut self, input: &str) {
        if input.trim() == "" {
            return;
        }
        let push = if let Some(ref inp) = self.inputs.last() {
            inp.as_str() != input.trim()
        }  else {
            true
        };
        if push {
            self.inputs.push(input.trim().to_string());
        }
    }
    
    pub fn is_empty(&self) -> bool {
        self.inputs.len() == 0
    }
    
    pub fn get(&self, index: usize) -> Option<&String> {
        self.inputs.get(index)
    }
}



fn start_interactive(copy: bool, lookup: bool) {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(stdout,
           "{}{}Welcome to dero. Use Ctrl-C to quit.",
           termion::clear::All,
           termion::cursor::Goto(1, 1),
           // {} termion::cursor::Hide
    ).unwrap();
    
    write!(stdout,
        "{}{}Write romaja to convert it to 한글.",
        termion::cursor::Goto(1,2),
        termion::clear::CurrentLine,
    ).unwrap();
    
    write!(stdout,
        "{}{}dero: ",
        termion::cursor::Goto(1, 3),
        termion::clear::AfterCursor,
    ).unwrap();
    
    stdout.flush().unwrap();
    
    let mut text = String::new();
    let mut pos = 0;
    let mut history = History::new();
    let mut history_index = 0;
    
    
    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('\n') => {
                if text == "" {
                    continue;
                }
                
                let hangeul = dero::deromanize_escaped(&text);
                
                if copy {
                    copy_to_clipboard(&hangeul);
                }
                
                if lookup {
                    look_up_word(&hangeul);
                }
                
                history.push(&text);
                history_index = history.len();
                
                text.clear();
                pos = text.len();
            }
            Key::Char(ch) => {
                if pos == text.len() {
                    text.push(ch);
                    pos += ch.len_utf8();
                }
            },
            Key::Ctrl(c) => {
                if c == 'c' {
                    break;
                }
            }
            Key::Esc => {
                // clear?
            },
            // Cursor movement
            Key::Left => {
                // move back one translated character
            },
            Key::Right => {
                
            },
            Key::Up => {
                if history_index > 0 {
                    history_index -= 1;
                    text = history.get(history_index).unwrap().to_string();
                    pos = text.len();
                }
            },
            Key::Down => {
                if history_index + 1 >= history.len() {
                    text.clear();
                    pos = text.len();
                    if history_index < history.len() {
                        history_index = history.len();
                    }
                } else {
                    history_index += 1;
                    text = history.get(history_index).unwrap().to_string();
                    pos = text.len();
                }
            },
            // Delete back one source character
            Key::Backspace => {
                if ! text.is_empty() {
                    if pos == text.len() {
                        text.pop();
                        pos -= 1;
                    } else {
                        // TODO    
                    }
                }
            },
            _ => {}
        }
        
        write!(stdout,
            "{}{}dero: {}",
            termion::cursor::Goto(1, 3),
            termion::clear::AfterCursor,
            dero::deromanize_escaped(&text),
        ).unwrap();
        
        stdout.flush().unwrap();
    }

    write!(stdout, "{}",
        termion::cursor::Show,
    ).unwrap();
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

