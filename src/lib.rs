#![feature(plugin)]
#![plugin(phf_macros)]

extern crate phf;

mod maps;

use std::char;
use std::iter::Peekable;
use maps::{PhfTrie, CONSONANTS, VOWELS, FINAL_MAP, FINAL_COMBINATION_MAP};

const BLOCK_START: u32 = 44032;
const FINAL_COUNT: u32 = 28;
const ITEMS_PER_INITIAL: u32 = 588;
const CONSONANT_IEUNG: u32 = 11;

#[derive(Debug, Copy, Clone)]
pub struct Error {
    pub position: usize,
    pub kind: ErrorKind,
}

impl Error {
    pub fn offset(self, offset: isize) -> Error {
        Error {
            position: ((self.position as isize) + offset) as usize,
            kind: self.kind,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ErrorKind {
    InvalidConsonant(char),
    InvalidVowel(char),
    InvalidLetter(char),
    MissingFinalVowel,
}

fn read_consonant<I>(it: &mut Peekable<I>) -> Result<u32, Error>
    where I: Iterator<Item = (usize, char)>
{
    let (i, ch) = *it.peek().expect("read_consonant with empty input");
    let err = Error {
        position: i,
        kind: ErrorKind::InvalidConsonant(ch),
    };

    let mut map = &CONSONANTS;
    let mut res = None;

    while let Some(&(_, ch)) = it.peek() {
        match map.get(&ch) {
            Some(&PhfTrie::Leaf(value)) => {
                it.next();
                return value.ok_or(err);
            }
            Some(&PhfTrie::Node(value, ref children)) => {
                it.next();
                res = value;
                map = children;
            }
            None => {
                break;
            }
        }
    }

    res.ok_or(err)
}

fn read_vowel<I>(it: &mut Peekable<I>) -> Result<u32, Error>
    where I: Iterator<Item = (usize, char)>
{
    let (i, ch) = *it.peek().expect("read_vowel with empty input");
    let err = Error {
        position: i,
        kind: ErrorKind::InvalidVowel(ch),
    };

    let mut map = &VOWELS;
    let mut res = None;

    while let Some(&(_, ch)) = it.peek() {
        match map.get(&ch) {
            Some(&PhfTrie::Leaf(value)) => {
                it.next();
                return value.ok_or(err);
            }
            Some(&PhfTrie::Node(value, ref children)) => {
                it.next();
                res = value;
                map = children;
            }
            None => {
                break;
            }
        }
    }

    res.ok_or(err)
}

fn push_block(text: &mut String,
              initial: u32,
              vowel: u32,
              first_final: Option<u32>,
              second_final: Option<u32>) {
    let mut value = BLOCK_START;
    value += initial * ITEMS_PER_INITIAL;
    value += vowel * FINAL_COUNT;
    let mut mapped_final = 0;
    if let Some(ff) = first_final {
        mapped_final += *FINAL_MAP.get(&ff).expect("Invalid first final");
        if let Some(sf) = second_final {
            mapped_final += *FINAL_COMBINATION_MAP.get(&ff)
                                                  .expect("Invalid first final after mapping")
                                                  .get(&sf)
                                                  .expect("Invalid second final");
        }
    }
    value += mapped_final;
    // println!("Value: {}, Char: {}", value, char::from_u32(value).unwrap());
    text.push(char::from_u32(value).expect("Invalid UTF-8 value created from block"));
}

pub fn deromanize_words_into<F>(text: &str,
                                mut is_boundary: F,
                                output: &mut String)
                                -> Result<(), Error>
    where F: FnMut(char) -> bool
{
    let it = text.char_indices();
    let mut start = None;

    for (i, ch) in it {
        if is_boundary(ch) {
            if let Some(start) = start {
                let part = &text[start..i];
                try!(deromanize_into(part, output).map_err(|e| e.offset(start as isize)));
            }
            output.push(ch);
            start = None;
        } else if start.is_none() {
            start = Some(i);
        }
    }
    if let Some(start) = start {
        let part = &text[start..];
        try!(deromanize_into(part, output).map_err(|e| e.offset(start as isize)));
    }

    Ok(())
}

pub fn deromanize_words<F>(text: &str, is_boundary: F) -> Result<String, Error>
    where F: FnMut(char) -> bool
{
    let mut output = String::new();
    deromanize_words_into(text, is_boundary, &mut output).map(|_| output)
}

#[derive(Debug)]
enum DeroState {
    Initial,
    AfterInitial {
        initial: u32,
    },
    AfterVowel {
        initial: u32,
        vowel: u32,
    },
    AfterFirstFinal {
        initial: u32,
        vowel: u32,
        first_final: u32,
    },
    AfterSecondFinal {
        initial: u32,
        vowel: u32,
        first_final: u32,
        second_final: u32,
    },
}

pub fn deromanize_into(text: &str, output: &mut String) -> Result<(), Error> {
    use self::ErrorKind::*;
    use self::DeroState::*;

    let mut state = Initial;
    let mut it = text.char_indices().peekable();

    while it.peek().is_some() {
        //println!("State: {:?}", state);
        match state {
            Initial => {
                let mut copy = it.clone();
                if let Ok(code) = read_consonant(&mut copy) {
                    it = copy;
                    state = AfterInitial { initial: code };
                } else if let Ok(code) = read_vowel(&mut copy) {
                    it = copy;
                    state = AfterVowel { 
                        initial: CONSONANT_IEUNG, vowel: code
                    };
                } else {
                    let (i, ch) = *it.peek().expect("unreachable; while condition asserts is_some()");
                    return Err(Error {
                        position: i,
                        kind: InvalidLetter(ch),
                    });
                }
            }
            AfterInitial { initial } => {
                let code = try!(read_vowel(&mut it));
                state = AfterVowel {
                    initial: initial,
                    vowel: code,
                };
            },
            AfterVowel { initial, vowel } => {
                // Read consonant (or the beginning of next block)
                // We clone the iterator so that read_vowel does not advance `it`.
                let mut copy = it.clone();
                if let Ok(code) = read_consonant(&mut copy) {
                    // Commit the consumed characters.
                    it = copy;
                    state = if FINAL_MAP.contains_key(&code) {
                        AfterFirstFinal {
                            initial: initial,
                            vowel: vowel,
                            first_final: code,
                        }
                    } else {
                        push_block(output, initial, vowel, None, None);
                        AfterInitial { initial: code }
                    };
                } else if let Ok(code) = read_vowel(&mut copy) {
                    it = copy;
                    push_block(output, initial, vowel, None, None);
                    state = AfterVowel { 
                        initial: CONSONANT_IEUNG, vowel: code,
                    };
                } else {
                    let (i, ch) = *it.peek().expect("unreachable; while condition asserts is_some()");
                    return Err(Error {
                        position: i,
                        kind: InvalidLetter(ch),
                    });
                }
            }
            AfterFirstFinal { initial, vowel, first_final } => {
                // If this is a vowel: goto 2 | otherwise final consonant or next block
                // We clone the iterator so that read_vowel does not advance `it`.
                let mut copy = it.clone();
                if let Ok(code) = read_consonant(&mut copy) {
                    // Commit the consumed characters.
                    it = copy;
                    // let mapped = *FINAL_MAP.get(&first_final)
                    //                        .expect("invariant broken; first_final must be in \
                    //                                 FINAL_MAP");
                    // Can anything follow the other final?

                    let can_follow = FINAL_COMBINATION_MAP.get(&first_final)
                                                          .and_then(|map| map.get(&code))
                                                          .is_some();
                    state = if can_follow {
                        AfterSecondFinal {
                            initial: initial,
                            vowel: vowel,
                            first_final: first_final,
                            second_final: code,
                        }
                    } else {
                        push_block(output, initial, vowel, Some(first_final), None);
                        AfterInitial { initial: code }
                    };
                } else if let Ok(code) = read_vowel(&mut copy) {
                    it = copy;
                    push_block(output, initial, vowel, None, None);
                    state = AfterVowel {
                        initial: first_final, vowel: code
                    };
                } else {
                    let (i, ch) = *it.peek().expect("unreachable; while condition asserts is_some()");
                    return Err(Error {
                        position: i,
                        kind: InvalidLetter(ch),
                    });
                }
            }
            AfterSecondFinal { initial, vowel, first_final, second_final } => {
                // full block. If this is a consonant: goto 1, if vowel: goto 2
                // We clone the iterator so that read_vowel does not advance `it`.
                let mut copy = it.clone();
                if let Ok(code) = read_consonant(&mut copy) {
                    // Commit the consumed characters.
                    it = copy;
                    push_block(output,
                               initial,
                               vowel,
                               Some(first_final),
                               Some(second_final));
                    state = AfterInitial { initial: code };
                } else if let Ok(code) = read_vowel(&mut copy) {
                    it = copy;
                    push_block(output, initial, vowel, Some(first_final), None);
                    state = AfterVowel { initial: second_final, vowel: code };
                } else {
                    let (i, ch) = *it.peek().expect("unreachable; while condition asserts is_some()");
                    return Err(Error {
                        position: i,
                        kind: InvalidLetter(ch),
                    });
                }
            }
        }
    }

    match state {
        AfterInitial { .. } => {
            return Err(Error {
                position: text.len(),
                kind: MissingFinalVowel,
            });
        }
        AfterVowel { initial, vowel } => {
            push_block(output, initial, vowel, None, None);
        }
        AfterFirstFinal { initial, vowel, first_final } => {
            push_block(output, initial, vowel, Some(first_final), None);
        }
        AfterSecondFinal { initial, vowel, first_final, second_final } => {
            push_block(output,
                       initial,
                       vowel,
                       Some(first_final),
                       Some(second_final));
        }
        Initial => {}
    }
    Ok(())
}

pub fn deromanize(text: &str) -> Result<String, Error> {
    let mut output = String::new();
    deromanize_into(text, &mut output).map(|_| output)
}
