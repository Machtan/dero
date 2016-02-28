
use std::char;
use maps::{CONSONANTS, VOWELS, FINAL_MAP, FINAL_COMBINATION_MAP};


const BLOCK_START: u32          = 44032;
const CONSONANT_COUNT: u32      = 30;
const VOWEL_COUNT: u32          = 21;
const FINAL_COUNT: u32          = 28;
const ITEMS_PER_INITIAL: u32    = 588;
const VOWEL_OFFSET: u32         = 28;
const CONSONANT_IEUNG: u32      = 11;

#[derive(Debug)]
pub enum DeromanizeError {
    InvalidConsonant { letter: char, position: usize },
    InvalidVowel { letter: char, position: usize },
    InvalidLetter { letter: char, position: usize },
    MissingFinalVowel { position: usize },
}

fn read_consonant(text: &str, i: usize, indices: &[usize])
        -> Result<(u32, usize), DeromanizeError> {
    use self::DeromanizeError::*;
    for &len in [2, 1].iter() {
        if i + len > indices.len() { // Skip if there aren't enough text left
            continue;
        }
        let part = if i == indices.len() - len { // Handle
            &text[indices[i]..]
        } else {
            &text[indices[i] .. indices[i + len]]
        };
        if let Some(index) = CONSONANTS.get(part) {
            return Ok((*index, len));
        }
    }
    return Err(InvalidConsonant { letter: text.chars().nth(i).unwrap(), position: i });
}

fn read_vowel(text: &str, i: usize, indices: &[usize])
        -> Result<(u32, usize), DeromanizeError> {
    use self::DeromanizeError::*;
    for &len in [3, 2, 1].iter() {
        if i + len > indices.len() { // Skip if there aren't enough text left
            continue;
        }
        let part = if i == indices.len() - len { // Handle
            &text[indices[i]..]
        } else {
            &text[indices[i] .. indices[i + len]]
        };
        if let Some(index) = VOWELS.get(part) {
            return Ok((*index, len));
        }
    }
    return Err(InvalidVowel { letter: text.chars().nth(i).unwrap(), position: i });
}

fn push_block(text: &mut String, initial: u32, vowel: u32, first_final: Option<u32>,
        last_final: Option<u32>) {
    let mut value = BLOCK_START;
    value += initial * ITEMS_PER_INITIAL;
    value += vowel * FINAL_COUNT;
    let mut _final = 0;
    if let Some(ff) = first_final {
        _final += *FINAL_MAP.get(&ff).unwrap();
        if let Some(lf) = last_final {
            _final += *FINAL_COMBINATION_MAP.get(&ff).unwrap().get(&lf).unwrap();
        }
    }
    value += _final;
    //println!("Value: {}, Char: {}", value, char::from_u32(value).unwrap());
    text.push(char::from_u32(value).unwrap());
}

fn deromanize_part(part: &str, start_index: usize) -> Result<String, DeromanizeError> {
    use self::DeromanizeError::*;
    match deromanize_validated(part) {
        Ok(deromanized) => {
            Ok(deromanized)
        },
        Err(InvalidConsonant { letter, position }) => {
            Err(InvalidConsonant { 
                letter: letter, 
                position: start_index + position,
            })
        },
        Err(InvalidVowel { letter, position }) => {
            Err(InvalidVowel { 
                letter: letter, 
                position: start_index + position,
            })
        },
        Err(InvalidLetter { letter, position }) => {
            Err(InvalidLetter { 
                letter: letter, 
                position: start_index + position,
            })
        },
        Err(MissingFinalVowel { position }) => {
            Err(MissingFinalVowel { position: start_index + position })
        }
    }
}

pub fn deromanize<F>(text: &str, allow_char: F)
        -> Result<String, DeromanizeError>
        where F: Fn(char) -> bool {
    use self::DeromanizeError::*;
    let mut output = String::new();
    let mut start = 0;
    let mut start_char_index = 0;
    let indices: Vec<_> = text.char_indices().collect();
    for (n, &(i, ch)) in indices.iter().enumerate() {
        if allow_char(ch) { // Check if it is time to break a block
            //println!("Valid break char at {}: '{}'", n, ch);
            if i != start { // Earlier stuff
                let part = &text[start .. i];
                let res = try!(deromanize_part(part, start_char_index));
                output.push_str(&res);
            }
            output.push(ch);
            start_char_index = n + 1;
            if n != indices.len() - 1 {
                start = indices[n + 1].0;
            }
        }
    }
    if start_char_index != indices.len() {
        let part = &text[start ..];
        //println!("Handling remainder...: part: ({}..): '{}'", start, part);
        let res = try!(deromanize_part(part, start_char_index));
        output.push_str(&res);
    }
    Ok(output)
}

pub fn deromanize_validated(text: &str) -> Result<String, DeromanizeError> {
    use self::DeromanizeError::*;
    let mut output = String::new();

    let mut initial: u32 = 0;
    let mut vowel: u32 = 0;
    let mut first_final: u32 = 0;
    let mut last_final: u32 = 0;

    let mut pos = 0;
    let indices: Vec<_> = text.char_indices().map(|(i, _)| i).collect();
    let mut i = 0;
    while i < indices.len() {
        //println!("{}: ({}: {})", i, pos, text.chars().nth(i).unwrap());
        match pos {
            0 => { // Read initial
                // Allow vowels with no consonant in the beginning of a block sequence
                if i == 0 {
                    if let Ok((index, len)) = read_vowel(text, i, &indices) {
                        initial = CONSONANT_IEUNG;
                        vowel = index;
                        pos = 2;
                        i += len;
                        continue;
                    }
                }
                let (index, len) = try!(read_consonant(text, i, &indices));
                initial = index;
                pos += 1;
                i += len;
            },
            1 => { // Read Vowel
                let (index, len) = try!(read_vowel(text, i, &indices));
                vowel = index;
                pos += 1;
                i += len;
            },
            2 => { // Read consonant (or the beginning of next block)
                // Read a consonant
                if let Ok((index, len)) = read_consonant(text, i, &indices) {
                    if FINAL_MAP.get(&index).is_some() { // If it cannot be a final, goto 1
                        first_final = index;
                        pos += 1;
                    } else {
                        push_block(&mut output, initial, vowel, None, None);
                        initial = index;
                        pos = 1;
                    }
                    i += len;
                // Allow two vowels in a row and just prefix ieung ('ㅇ')
                } else if let Ok((index, len)) = read_vowel(text, i, &indices) {
                    push_block(&mut output, initial, vowel, None, None);
                    initial = CONSONANT_IEUNG;
                    vowel = index;
                    pos = 2;
                    i += len;
                } else {
                    return Err(InvalidLetter { letter: text.chars().nth(i).unwrap(), position: i });
                }
            },
            3 => { // If this is a vowel: goto 2 | otherwise final consonant or next block
                if let Ok((index, len)) = read_consonant(text, i, &indices) {
                    let mapped = *FINAL_MAP.get(&first_final).unwrap();
                    // Can anything follow the other final?
                    if let Some(map) = FINAL_COMBINATION_MAP.get(&mapped) {
                        // Can this consonant follow the other final?
                        if map.get(&index).is_some() {
                            last_final = index;
                            pos += 1;
                        } else { // goto 1
                            push_block(&mut output, initial, vowel, Some(first_final), None);
                            initial = index;
                            pos = 1;
                        }
                    } else { // goto 1
                        push_block(&mut output, initial, vowel, Some(first_final), None);
                        initial = index;
                        pos = 1;
                    }
                    i += len;

                } else if let Ok((index, len)) = read_vowel(text, i, &indices) {
                    push_block(&mut output, initial, vowel, None, None);
                    initial = first_final;
                    vowel = index;
                    i += len;
                    pos = 2;
                } else {
                    return Err(InvalidLetter { letter: text.chars().nth(i).unwrap(), position: i });
                }
            },
            4 => { // full block. If this is a consonant: goto 1, if vowel: goto 2
                if let Ok((index, len)) = read_consonant(text, i, &indices) {
                    push_block(&mut output, initial, vowel, Some(first_final), Some(last_final));
                    initial = index;
                    i += len;
                    pos = 1;
                } else if let Ok((index, len)) = read_vowel(text, i, &indices) {
                    push_block(&mut output, initial, vowel, Some(first_final), None);
                    initial = last_final;
                    vowel = index;
                    i += len;
                    pos = 2;
                } else {
                    return Err(InvalidLetter { letter: text.chars().nth(i).unwrap(), position: i });
                }
            }
            _ => unreachable!(),
        }
    }

    match pos {
        1 => {
            return Err(MissingFinalVowel { position: indices.len() });
        },
        2 => {
            push_block(&mut output, initial, vowel, None, None);
        },
        3 => {
            push_block(&mut output, initial, vowel, Some(first_final), None);
        },
        4 => {
            push_block(&mut output, initial, vowel, Some(first_final), Some(last_final));
        },
        _ => {}
    }
    Ok(output)
}
