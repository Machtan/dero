//! Module for interactive terminal user interface functionality.

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use std::collections::VecDeque;
use std::io::{stdin, stdout, Write};

use super::deromanize_escaped;

pub struct History {
    max_len: usize,
    inputs: VecDeque<String>,
}

impl History {
    #[inline]
    pub fn new(max_len: usize) -> History {
        History {
            max_len: max_len,
            inputs: VecDeque::new(),
        }
    }
    
    #[inline]
    pub fn len(&self) -> usize {
        self.inputs.len()
    }
    
    pub fn push(&mut self, input: &str) {
        if input.trim() == "" {
            return;
        }
        
        let push = {
            let len = self.len();
            if len == 0 {
                true
            } else {
                let last = self.get(len - 1).unwrap();
                last.as_str() != input.trim()
            }
        };
        if push {
            if self.len() == self.max_len {
                self.inputs.pop_front();
            }
            self.inputs.push_back(input.trim().to_string());
        }
    }
    
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.inputs.is_empty()
    }
    
    pub fn get(&self, index: usize) -> Option<&String> {
        self.inputs.get(index)
    }
}

const DEFAULT_HISTORY_SIZE: usize = 64;

pub fn start_interactive_loop<F: FnMut(&str)>(on_deromanize: &mut F) {
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
    let mut history = History::new(DEFAULT_HISTORY_SIZE);
    let mut history_index = 0;
    
    
    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('\n') => {
                if text == "" {
                    continue;
                }
                
                let hangeul = deromanize_escaped(&text);
                
                on_deromanize(&hangeul);
                
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
            deromanize_escaped(&text),
        ).unwrap();
        
        stdout.flush().unwrap();
    }

    write!(stdout, "{}",
        termion::cursor::Show,
    ).unwrap();
}
