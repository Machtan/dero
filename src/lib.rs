//! # Conversion format
//! The following are the conversion rules for the 'romaja' understood by this
//! library.
//! 
//! The format is a mix of the revised romanization and my understanding of the
//! pronunciation of the characters. The reason why 'x' is used instead of 'ng' 
//! is to ensure that parsing is unambiguous (eg `hangyl` => `한글` and never 
//! `항을`), and because it is only pronounced this way in the final position.
//! 
//! # Consonants
//! | Sequence  | g | G | n | d | D | r / l | m | b | B | s | S | x | j | J | ch | k | t | p | h |
//! |-----------|---|---|---|---|---|-------|---|---|---|---|---|---|---|---|----|---|---|---|---|
//! | Consonant | ㄱ | ㄲ | ㄴ | ㄷ | ㄸ | ㄹ   | ㅁ | ㅂ | ㅃ | ㅅ | ㅆ | ㅇ | ㅈ | ㅉ | ㅊ | ㅋ | ㅌ | ㅍ | ㅎ |
//!
//! # Vowels
//! | Sequence  | a | eo | o | u | y | i | ae | e | ya | yeo | yo | yu | yae | ye | wa | wae | weo | we | oe | wi | yi |
//! |-----------|---|----|---|---|---|---|----|---|----|-----|----|----|-----|----|----|-----|-----|----|----|----|----|
//! | Vowel     | ㅏ | ㅓ | ㅗ | ㅜ | ㅡ | ㅣ | ㅐ | ㅔ | ㅑ | ㅕ | ㅛ  | ㅠ  | ㅒ  | ㅖ  | ㅘ  | ㅙ  | ㅝ   | ㅞ | ㅚ  | ㅟ  | ㅢ |
//!
//! # Escaping
//! Text can be left untouched by leaving it inside square brackets, and using
//! the 'deromanize_escaped' function.
//! 
//! Ex: `annyeox ha[se]yo` => `안녕 하se요`
//! 
//! # Rules
//! Aside from those conversions, the following rules hold:
//!
//! - Initial vowels do not need an explicit ieung ('x')
//!   
//!   Ex: `arayo` => `아라요`
//! - A vowel following a vowel does not need explicit ieungs either
//!   
//!   Ex: `igeo mweoyeyo?` => `이거 뭐예요?`
//! - As a note to the former: As `eo` => `어`, ieung can still be used to 
//!   separate those:
//!   
//!   Ex: `rexonSi` => `레온씨`
//! - Initials not followed by a vowel will be left as standalone characters
//! 
//!   Ex: `jinJa, kkkkkkk` => `진짜, ㅋㅋㅋㅋㅋㅋㅋ`
extern crate hangeul2;

use hangeul2::{Initial, Vowel, Final, Block};


#[derive(Debug)]
enum DeroState {
    Empty,
    AfterInitial(Initial),
    AfterVowel(Initial, Vowel),
    AfterFinal(Initial, Vowel, Final),
}

/// Reads a vowel from the given romaja and the number of chars read.
/// This also corresponds to bytes, as the characters must be ASCII chars.
pub fn read_vowel(text: &str) -> Option<(Vowel, usize)> {
    use hangeul2::Vowel::*;
    let mut chars = text.chars();
    match chars.next() {
        Some('i') => Some((I, 1)),
        Some('a') => match chars.next() {
            Some('e') => Some((Ae, 2)),
            _ => Some((A, 1)),
        },
        Some('e') => match chars.next() {
            Some('o') => Some((Eo, 2)),
            _ => Some((E, 1)),
        },
        Some('o') => match chars.next() {
            Some('e') => Some((Oe, 2)),
            _ => Some((O, 1)),
        },
        Some('u') => Some((U, 1)),
        Some('y') => match chars.next() {
            Some('a') => match chars.next() {
                Some('e') => Some((Yae, 3)),
                _ => Some((Ya, 2)),
            },
            Some('e') => match chars.next() {
                Some('o') => Some((Yeo, 3)),
                _ => Some((Ye, 2)),
            },
            Some('o') => Some((Yo, 2)),
            Some('u') => Some((Yu, 2)),
            Some('i') => Some((Yi, 2)),
            _ => Some((Y, 1)),
        },
        Some('w') => match chars.next() {
            Some('a') => match chars.next() {
                Some('e') => Some((Wae, 3)),
                _ => Some((Wa, 2)),
            },
            Some('e') => match chars.next() {
                Some('o') => Some((Weo, 3)),
                _ => Some((We, 2)),
            },
            Some('i') => Some((Wi, 2)),
            _ => None,
        },
        _ => None,
    }
}

/// Reads an initial consonant from the given romaja and returns the number of 
/// characters read.
/// This also corresponds to bytes, as the characters must be ASCII chars.
pub fn read_initial(text: &str) -> Option<(Initial, usize)> {
    use hangeul2::Initial::*;
    let mut chars = text.chars();
    match chars.next() {
        Some('g') => Some((G, 1)),
        Some('G') => Some((Gg, 1)),
        Some('n') => Some((N, 1)),
        Some('d') => Some((D, 1)),
        Some('D') => Some((Dd, 1)),
        Some('r') => Some((R, 1)),
        Some('l') => Some((R, 1)),
        Some('m') => Some((M, 1)),
        Some('b') => Some((B, 1)),
        Some('B') => Some((Bb, 1)),
        Some('s') => Some((S, 1)),
        Some('S') => Some((Ss, 1)),
        Some('x') => Some((Ieung, 1)),
        Some('j') => Some((J, 1)),
        Some('J') => Some((Jj, 1)),
        Some('c') => match chars.next() {
            Some('h') => Some((Ch, 2)),
            _ => None
        },
        Some('k') => Some((K, 1)),
        Some('t') => Some((T, 1)),
        Some('p') => Some((P, 1)),
        Some('h') => Some((H, 1)),
        _ => None,
    }
}

/// Reads a final consonant combination from the given romaja and returns the 
/// number of characters read.
/// This also corresponds to bytes, as the characters must be ASCII chars.
pub fn read_final(mut text: &str) -> Option<(Final, usize)> {
    use hangeul2::Initial::*;
    let (first, flen) = if let Some((ini, len)) = read_initial(text) {
        text = &text[len..];
        (ini, len)
    } else {
        return None;
    };
    let mut chars = text.chars();
    let (fin, len) = match (first, chars.next()) {
        (G, Some('s')) => (Final::Gs, 1),
        (G, _) => (Final::G, 0),
        (Gg, _) => (Final::Gg, 0),
        (N, Some('j')) => (Final::Nj, 1),
        (N, Some('h')) => (Final::Nh, 1),
        (N, _) => (Final::N, 0),
        (D, _) => (Final::D, 0),
        (Dd, _) => return None,
        (R, Some('g')) => (Final::Lg, 1),
        (R, Some('m')) => (Final::Lm, 1),
        (R, Some('b')) => (Final::Lb, 1),
        (R, Some('s')) => (Final::Ls, 1),
        (R, Some('t')) => (Final::Lt, 1),
        (R, Some('p')) => (Final::Lp, 1),
        (R, Some('h')) => (Final::Lh, 1),
        (R, _) => (Final::L, 0),
        (M, _) => (Final::M, 0),
        (B, Some('s')) => (Final::Bs, 1),
        (B, _) => (Final::B, 0),
        (Bb, _) => return None,
        (S, _) => (Final::S, 0),
        (Ss, _) => (Final::Ss, 0),
        (Ieung, _) => (Final::Ieung, 0),
        (J, _) => (Final::J, 0),
        (Jj, _) => return None,
        (Ch, _) => (Final::Ch, 0),
        (K, _) => (Final::K, 0),
        (T, _) => (Final::T, 0),
        (P, _) => (Final::P, 0),
        (H, _) => (Final::H, 0),
    };
    Some((fin, flen + len))
}

#[inline]
fn skip<I>(iter: &mut I, n: usize) where I: Iterator {
    for _ in 0..n {
        iter.next();
    }
}

/// Converts as much of the given romaja-containing string to 한글 as possible.
pub fn deromanize(text: &str) -> String {
    let mut s = String::new();
    deromanize_into(text, &mut s);
    s
}

/// Converts as much of the given romaja-containing string to 한글 as possible.
pub fn deromanize_into(text: &str, s: &mut String) {
    use self::DeroState::*;
    let mut state = Empty;
    let mut chars = text.char_indices().peekable();
    while let Some(&(i, ch)) = chars.peek() {
        let rem = &text[i..];
        //println!("State: {:?}", state);
        state = match state {
            Empty => {
                if let Some((ini, len)) = read_initial(rem) {
                    skip(&mut chars, len);
                    AfterInitial(ini)
                } else if let Some((vow, len)) = read_vowel(rem) {
                    skip(&mut chars, len);
                    AfterVowel(Initial::Ieung, vow)
                } else {
                    s.push(ch);
                    chars.next();
                    Empty
                }
            }
            AfterInitial(ini) => {
                if let Some((nvow, len)) = read_vowel(rem) {
                    skip(&mut chars, len);
                    AfterVowel(ini, nvow)
                } else {
                    s.push(ini.as_char());
                    Empty
                }
            }
            AfterVowel(ini, vow) => {
                if let Some((nfin, len)) = read_final(rem) {
                    skip(&mut chars, len);
                    AfterFinal(ini, vow, nfin)
                } else if let Some((nvow, len)) = read_vowel(rem) {
                    skip(&mut chars, len);
                    s.push(Block::from_parts(ini, vow, Final::Empty).combine());
                    AfterVowel(Initial::Ieung, nvow)
                // Consonants invalid in final position, ie: Bb
                } else if let Some((nini, len)) = read_initial(rem) {
                    skip(&mut chars, len);
                    s.push(Block::from_parts(ini, vow, Final::Empty).combine());
                    AfterInitial(nini)
                } else {
                    s.push(Block::from_parts(ini, vow, Final::Empty).combine());
                    s.push(ch);
                    chars.next();
                    Empty
                }
            }
            AfterFinal(ini, vow, fin) => {
                if let Some((nvow, len)) = read_vowel(rem) {
                    use hangeul2::Final::*;
                    skip(&mut chars, len);
                    let (fin, nini) = match fin {
                        G => (Empty, Initial::G),
                        Gg => (Empty, Initial::Gg),
                        Gs => (G, Initial::S),
                        N => (Empty, Initial::N),
                        Nj => (N, Initial::J),
                        Nh => (N, Initial::H),
                        D => (Empty, Initial::D),
                        L => (Empty, Initial::R),
                        Lg => (L, Initial::G),
                        Lm => (L, Initial::M),
                        Lb => (L, Initial::B),
                        Ls => (L, Initial::S),
                        Lt => (L, Initial::T),
                        Lp => (L, Initial::P),
                        Lh => (L, Initial::H),
                        M => (Empty, Initial::M),
                        B => (Empty, Initial::B),
                        Bs => (B, Initial::S),
                        S => (Empty, Initial::S),
                        Ss => (Empty, Initial::Ss),
                        Ieung => (Empty, Initial::Ieung),
                        J => (Empty, Initial::J),
                        Ch => (Empty, Initial::Ch),
                        K => (Empty, Initial::K),
                        T => (Empty, Initial::T),
                        P => (Empty, Initial::P),
                        H => (Empty, Initial::H),
                        Empty => unreachable!(),
                    };
                    s.push(Block::from_parts(ini, vow, fin).combine());
                    AfterVowel(nini, nvow)
                } else {
                    s.push(Block::from_parts(ini, vow, fin).combine());
                    Empty
                }
            }
        }
    }
    match state {
        Empty => {}
        AfterInitial(ini) => s.push(ini.as_char()),
        AfterVowel(ini, vow) => s.push(Block::from_parts(ini, vow, Final::Empty).combine()),
        AfterFinal(ini, vow, fin) => s.push(Block::from_parts(ini, vow, fin).combine()),
    }
}

/// Converts as much of the given romaja-containing string to 한글 as possible.
/// The conversion ignores all text between square brackets.
pub fn deromanize_escaped(text: &str) -> String {
    const ESCAPE_START: char = '[';
    const ESCAPE_END: char = ']';
    let mut s = String::new();
    let mut i = 0;
    while i < text.len() {
        //println!("i: {}", i);
        let rem = &text[i..];
        if let Some(start) = rem.find(ESCAPE_START) {
            deromanize_into(&rem[..start], &mut s);
            if let Some(end) = rem.find(ESCAPE_END) {
                s.push_str(&rem[start + ESCAPE_START.len_utf8() .. end]);
                i += end + ESCAPE_END.len_utf8();
            } else {
                s.push_str(&rem[start + ESCAPE_START.len_utf8() ..]);
                break;
            }
        } else {
            deromanize_into(rem, &mut s);
            break;
        }
    }
    s
}

#[cfg(test)]
mod tests {
    use super::hangeul2::{Initial, Vowel, Final, Block};
    use super::{deromanize_escaped, deromanize, read_initial, read_vowel, read_final};
    #[test]
    fn test_everything() {
        println!("Hello Dero!");
        let vowels = "a ya ae yae eo yeo e ye o wa wae oe yo u weo we wi yu y yi";
        let initials = "g G n d D r l m b B s S x j J ch k p t h";
        let finals = "g G gs n nj nh d l lg lm lb ls lt lp lh m b bs s S x j ch k t p h";
        for vowtext in vowels.split_whitespace() {
            let (vow, len) = read_vowel(vowtext).expect(&format!("Could not read vowel '{}'", vowtext));
            println!("Vowel: '{}' => {:?} |{}|", vowtext, vow, len);
            assert!(len == vowtext.len());
        }
        for initext in initials.split_whitespace() {
            let (ini, len) = read_initial(initext).expect(&format!("Could not read initial '{}'", initext));
            println!("Initial: '{}' => {:?} |{}|", initext, ini, len);
            assert!(len == initext.len());
        }
        for fintext in finals.split_whitespace() {
            let (fin, len) = read_final(fintext).expect(&format!("Could not read final '{}'", fintext));
            println!("Final: '{}' => {:?} |{}|", fintext, fin, len);
            assert!(len == fintext.len());
        }
        let derovow = deromanize(vowels);
        println!("Vowels: {}", derovow);
        let deroini = deromanize(initials);
        println!("Initials: {}", deroini);
        let examples = "manhda eobsxeoyo balgda masxiSxeoSxeoyo masyeoSxeoyo yeByn yeoja ijxeobeoryeoSxeoyo";
        for ex in examples.split_whitespace() {
            let dero = deromanize(ex);
            println!("{} => {}", ex, dero);
        }
        // Error: mashyeoSxeoyo => 만현어요
        // Error: masyeoSxeoyo => 마년어요
        let syeo = Block::from_parts(Initial::S, Vowel::Yeo, Final::Ss).combine();
        println!("Syeo: {}", syeo);
    
        let garbage = "astioeangpdurfw:qfujwpk:xlbcmv/2102398473925^9asheynttujfujt";
        println!("Garbage: {}", deromanize(garbage));
        // Error: p -> ㅌ
        // Error: t -> ㅍ
        let uj = Block::from_parts(Initial::Ieung, Vowel::U, Final::J).combine();
        println!("Uj: {}", uj);
        let uch = Block::from_parts(Initial::Ieung, Vowel::U, Final::Ch).combine();
        println!("Uch: {}", uch);
    
        let escaped = deromanize_escaped("annyeoxhaseyo, [Jakob]Si!");
        println!("Escaped: {}", escaped);
    
        let escaped_garbage = "qdp:[rwufa]eonbcmev/[arp]dft[]sa[][][[nhon]]etydrnt";
        println!("Escaped garbage: {}", deromanize_escaped(escaped_garbage));
    }
}