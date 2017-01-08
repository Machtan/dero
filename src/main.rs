extern crate hangeul2;

use hangeul2::{Initial, Vowel, Final};


#[derive(Debug)]
enum DeroState {
    Empty,
    AfterInitial {
        initial: Initial,
    },
    AfterVowel {
        initial: Initial,
        vowel: Vowel,
    },
    AfterFirstFinal {
        initial: Initial,
        vowel: Vowel,
        final_: Final,
    },
    AfterSecondFinal {
        initial: Initial,
        vowel: Vowel,
        final_: Final,
    },
}

/// Returns a vowel and the number of chars read.
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
        (S, _) => (Final::N, 0),
        (Ss, _) => (Final::N, 0),
        (Ieung, _) => (Final::N, 0),
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

pub fn deromanize(text: &str) -> String {
    use self::DeroState::*;
    let mut s = String::new();
    let mut state = String::new();
    let mut state = Empty;
    let mut i = 0;
    while i < text.len() {
        match state {
            Empty => {

            }
            _ => unimplemented!(),
        }
    } 
    s
}



pub fn main() {
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
}