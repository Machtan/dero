use std::char;
use std::iter::Peekable;
use maps::{FINAL_MAP, FINAL_COMBINATION_MAP};

const BLOCK_START: u32 = 44032;
const FINAL_COUNT: u32 = 28;
const ITEMS_PER_INITIAL: u32 = 588;
const CONSONANT_IEUNG: u32 = 11;

#[derive(Debug, Copy, Clone)]
pub enum DeromanizeError {
    InvalidConsonant {
        letter: char,
        position: usize,
    },
    InvalidVowel {
        letter: char,
        position: usize,
    },
    InvalidLetter {
        letter: char,
        position: usize,
    },
    MissingFinalVowel {
        position: usize,
    },
}

impl DeromanizeError {
    pub fn position(self) -> usize {
        use self::DeromanizeError::*;

        match self {
            InvalidConsonant { position, .. } => position,
            InvalidVowel { position, .. } => position,
            InvalidLetter { position, .. } => position,
            MissingFinalVowel { position } => position,
        }
    }

    pub fn offset(self, offset: usize) -> DeromanizeError {
        use self::DeromanizeError::*;

        match self {
            InvalidConsonant { position, letter } => {
                InvalidConsonant {
                    position: position + offset,
                    letter: letter,
                }
            }
            InvalidVowel { position, letter } => {
                InvalidVowel {
                    position: position + offset,
                    letter: letter,
                }
            }
            InvalidLetter { position, letter } => {
                InvalidLetter {
                    position: position + offset,
                    letter: letter,
                }
            }
            MissingFinalVowel { position } => MissingFinalVowel { position: position + offset },
        }
    }
}

fn read_consonant<I>(it: &mut I) -> Result<u32, DeromanizeError>
    where I: Iterator<Item = (usize, char)>
{
    use self::DeromanizeError::*;

    let (i, ch) = it.next().expect("read_consonant with empty input");
    let err = InvalidConsonant {
        letter: ch,
        position: i,
    };
    let res = match ch {
        'g' => 0, // ㄱ
        'G' => 1, // ㄲ
        'n' => 2, // ㄴ
        'd' => 3, // ㄷ
        'D' => 4, // ㄸ
        'r' => 5, // ㄹ
        'l' => 5, // ㄹ
        'm' => 6, // ㅁ
        'b' => 7, // ㅂ
        'B' => 8, // ㅃ
        's' => 9, // ㅅ
        'S' => 10, // ㅆ
        'x' => 11, // ㅇ
        'j' => 12, // ㅈ
        'J' => 13, // ㅉ
        'c' => {
            match it.next() {
                Some((_, 'h')) => 14,
                _ => {
                    return Err(err);
                }
            }
        }
        'k' => 15, // ㅋ
        't' => 16, // ㅌ
        'p' => 17, // ㅍ
        'h' => 18, // ㅎ
        _ => {
            return Err(err);
        }
    };
    Ok(res)
}

fn read_vowel<I>(it: &mut Peekable<I>) -> Result<u32, DeromanizeError>
    where I: Iterator<Item = (usize, char)>
{
    use self::DeromanizeError::*;

    let mut state = Err(0);
    let (i, ch) = *it.peek().expect("read_vowel with empty input");
    let err = InvalidVowel {
        letter: ch,
        position: i,
    };

    while let Some(&(_, ch)) = it.peek() {
        let next_state = match (state, ch) {
            (Err(0), 'a') => Ok(0), // a 아
            (Ok(0), 'e') => Ok(1), // ae 애

            (Err(0), 'e') => Ok(5), // e 에
            (Ok(5), 'o') => Ok(4), // eo 어

            (Err(0), 'i') => Ok(20), // i 이

            (Err(0), 'o') => Ok(8), // o 오
            (Ok(8), 'e') => Ok(11), // oe 외

            (Err(0), 'u') => Ok(13), // u 우

            (Err(0), 'w') => Err(1), // w
            (Err(1), 'a') => Ok(9), // wa 와
            (Ok(9), 'e') => Ok(10), // wae 왜
            (Err(1), 'e') => Ok(15), // we 웨
            (Ok(15), 'o') => Ok(14), // weo 워
            (Err(1), 'i') => Ok(16), // wi 위

            (Err(0), 'y') => Ok(18), // y ㅡ
            (Ok(18), 'a') => Ok(2), // ya 야
            (Ok(2), 'e') => Ok(3), // yae 얘
            (Ok(18), 'e') => Ok(7), // ye 예
            (Ok(7), 'o') => Ok(6), // yeo 여
            (Ok(18), 'i') => Ok(19), // yi 의
            (Ok(18), 'u') => Ok(17), // yu 유
            (Ok(18), 'o') => Ok(12), // yo 요

            _ => {
                break;
            }
        };
        it.next();
        state = next_state;
    }

    state.map_err(|_| err)
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
            mapped_final += *FINAL_COMBINATION_MAP.get(&mapped_final)
                                                  .expect("Invalid first final after mapping")
                                                  .get(&sf)
                                                  .expect("Invalid second final");
        }
    }
    value += mapped_final;
    // println!("Value: {}, Char: {}", value, char::from_u32(value).unwrap());
    text.push(char::from_u32(value).expect("Invalid UTF-8 value created from block"));
}

pub fn deromanize<F>(text: &str, is_break_char: F) -> Result<String, DeromanizeError>
    where F: Fn(char) -> bool
{
    let mut output = String::new();
    let mut it = text.char_indices();
    let mut start = None;

    for (i, ch) in &mut it {
        if is_break_char(ch) {
            if let Some(start) = start {
                let part = &text[start..i];
                let res = try!(deromanize_validated(part).map_err(|e| e.offset(start)));
                output.push_str(&res);
            }
            output.push(ch);
            start = None;
        } else if start.is_none() {
            start = Some(i);
        }
    }
    if let Some(start) = start {
        let part = &text[start..];
        let res = try!(deromanize_validated(part).map_err(|e| e.offset(start)));
        output.push_str(&res);
    }

    Ok(output)
}

#[derive(Debug)]
enum DeroState {
    Initial,
    AfterInitial {
        initial: u32,
    },
    AfterMissingConsonant,
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

pub fn deromanize_validated(text: &str) -> Result<String, DeromanizeError> {
    use self::DeromanizeError::*;
    use self::DeroState::*;
    let mut output = String::new();

    let mut state = Initial;

    let mut it = text.char_indices().peekable();

    while it.peek().is_some() {
        match state {
            Initial => {
                let mut copy = it.clone();
                if let Ok(code) = read_consonant(&mut copy) {
                    it = copy;
                    state = AfterInitial { initial: code };
                    continue;
                }
                state = AfterMissingConsonant;
            }
            AfterInitial { initial } => {
                let code = try!(read_vowel(&mut it));
                state = AfterVowel {
                    initial: initial,
                    vowel: code,
                };
            }
            AfterMissingConsonant => {
                let (i, ch) = *it.peek().expect("unreachable; while condition asserts is_some()");
                let code = match read_vowel(&mut it) {
                    Ok(code) => code,
                    Err(_) => {
                        return Err(InvalidLetter {
                            position: i,
                            letter: ch,
                        });
                    }
                };
                state = AfterVowel {
                    initial: CONSONANT_IEUNG,
                    vowel: code,
                }
            }
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
                        push_block(&mut output, initial, vowel, None, None);
                        AfterInitial { initial: code }
                    };
                    continue;
                }

                push_block(&mut output, initial, vowel, None, None);

                state = AfterMissingConsonant;
            }
            AfterFirstFinal { initial, vowel, first_final } => {
                // If this is a vowel: goto 2 | otherwise final consonant or next block
                // We clone the iterator so that read_vowel does not advance `it`.
                let mut copy = it.clone();
                if let Ok(code) = read_consonant(&mut copy) {
                    // Commit the consumed characters.
                    it = copy;
                    let mapped = *FINAL_MAP.get(&first_final)
                                           .expect("invariant broken; first_final must be in \
                                                    FINAL_MAP");
                    // Can anything follow the other final?

                    let can_follow = FINAL_COMBINATION_MAP.get(&mapped)
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
                        push_block(&mut output, initial, vowel, Some(first_final), None);
                        AfterInitial { initial: code }
                    };
                    continue;
                }

                push_block(&mut output, initial, vowel, None, None);

                state = AfterMissingConsonant;
            }
            AfterSecondFinal { initial, vowel, first_final, second_final } => {
                // full block. If this is a consonant: goto 1, if vowel: goto 2
                // We clone the iterator so that read_vowel does not advance `it`.
                let mut copy = it.clone();
                if let Ok(code) = read_consonant(&mut copy) {
                    // Commit the consumed characters.
                    it = copy;
                    push_block(&mut output,
                               initial,
                               vowel,
                               Some(first_final),
                               Some(second_final));
                    state = AfterInitial { initial: code };
                    continue;
                }

                push_block(&mut output, initial, vowel, Some(first_final), None);

                state = AfterMissingConsonant;
            }
        }
    }

    match state {
        AfterInitial { .. } => {
            return Err(MissingFinalVowel { position: text.len() });
        }
        AfterVowel { initial, vowel } => {
            push_block(&mut output, initial, vowel, None, None);
        }
        AfterFirstFinal { initial, vowel, first_final } => {
            push_block(&mut output, initial, vowel, Some(first_final), None);
        }
        AfterSecondFinal { initial, vowel, first_final, second_final } => {
            push_block(&mut output,
                       initial,
                       vowel,
                       Some(first_final),
                       Some(second_final));
        }
        Initial | AfterMissingConsonant => {}
    }
    Ok(output)
}
