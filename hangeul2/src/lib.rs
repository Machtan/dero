
use std::char;

const BLOCK_START: u32 = 0xAC00;
const CONSONANT_START: u32 = 0x3130;
const VOWEL_START: u32 = 0x314F;

const NUM_FINALS: u32 = 28;
const NUM_VOWELS: u32 = 21;

const CHARS_PER_INITIAL: u32 = NUM_VOWELS * NUM_FINALS;
const CHARS_PER_VOWEL: u32 = NUM_FINALS;

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum Initial {
    G,
    Gg,
    N,
    D,
    Dd,
    R,
    M,
    B,
    Bb,
    S,
    Ss,
    Ieung,
    J,
    Jj,
    Ch,
    K,
    P,
    T,
    H,
}
impl Initial {
    pub fn offset(&self) -> u32 {
        use self::Initial::*;
        match *self {
            G     => 0,
            Gg    => 1,
            N     => 2,
            D     => 3,
            Dd    => 4,
            R     => 5,
            M     => 6,
            B     => 7,
            Bb    => 8,
            S     => 9,
            Ss    => 10,
            Ieung => 11,
            J     => 12,
            Jj    => 13,
            Ch    => 14,
            K     => 15,
            T     => 16,
            P     => 17,
            H     => 18,
        }
    }
    
    fn char_offset(&self) -> u32 {
        use self::Initial::*;
        match *self {
            G     => 1,
            Gg    => 2,
            N     => 4,
            D     => 7,
            Dd    => 8,
            R     => 9,
            M     => 17,
            B     => 18,
            Bb    => 19,
            S     => 21,
            Ss    => 22,
            Ieung => 23,
            J     => 24,
            Jj    => 25,
            Ch    => 26,
            K     => 27,
            T     => 28,
            P     => 29,
            H     => 30,
        }
    }
    
    pub fn as_char(&self) -> char {
        let mut code = CONSONANT_START;
        code += self.char_offset();
        let res = char::from_u32(code)
            .expect("hangeul2 constructed an invalid hangeul character!");
        res
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum Vowel {
    A,
    Ae,
    Ya,
    Yae,
    Eo,
    E,
    Yeo,
    Ye,
    O,
    Wa,
    Wae,
    Oe,
    Yo,
    U,
    Weo,
    We,
    Wi,
    Yu,
    Y,
    Yi,
    I,
}
impl Vowel {
    pub fn offset(&self) -> u32 {
        use self::Vowel::*;
        match *self {
            A   => 0,
            Ae  => 1,
            Ya  => 2,
            Yae => 3,
            Eo  => 4,
            E   => 5,
            Yeo => 6,
            Ye  => 7,
            O   => 8,
            Wa  => 9,
            Wae => 10,
            Oe  => 11,
            Yo  => 12,
            U   => 13,
            Weo => 14,
            We  => 15,
            Wi  => 16,
            Yu  => 17,
            Y   => 18,
            Yi  => 19,
            I   => 20,
        }
    }
    
    pub fn as_char(&self) -> char {
        let mut code = VOWEL_START;
        code += self.offset();
        let res = char::from_u32(code)
            .expect("hangeul2 constructed an invalid hangeul character!");
        res
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum Final {
    Empty,
    G,
    Gg,
    Gs,
    N,
    Nj,
    Nh,
    D,
    L,
    Lg,
    Lm,
    Lb,
    Ls,
    Lt,
    Lp,
    Lh,
    M,
    B,
    Bs,
    S,
    Ss,
    Ieung,
    J,
    Ch,
    K,
    T,
    P,
    H,
}
impl Final {
    pub fn offset(&self) -> u32 {
        use self::Final::*;
        match *self {
            Empty => 0,
            G     => 1,
            Gg    => 2,
            Gs    => 3,
            N     => 4,
            Nj    => 5,
            Nh    => 6,
            D     => 7,
            L     => 8,
            Lg    => 9,
            Lm    => 10,
            Lb    => 11,
            Ls    => 12,
            Lt    => 13,
            Lp    => 14,
            Lh    => 15,
            M     => 16,
            B     => 17,
            Bs    => 18,
            S     => 19,
            Ss    => 20,
            Ieung => 21,
            J     => 22,
            Ch    => 23,
            K     => 24,
            T     => 25,
            P     => 26,
            H     => 27,
        }
    }
    
    
    
    fn char_offset(&self) -> u32 {
        use self::Final::*;
        match *self {
            Empty => 0,
            G     => 1,
            Gg    => 2,
            Gs    => 3,
            N     => 4,
            Nj    => 5,
            Nh    => 6,
            D     => 7,
            L     => 9,
            Lg    => 10,
            Lm    => 11,
            Lb    => 12,
            Ls    => 13,
            Lt    => 14,
            Lp    => 15,
            Lh    => 16,
            M     => 17,
            B     => 18,
            Bs    => 20,
            S     => 21,
            Ss    => 22,
            Ieung => 23,
            J     => 24,
            Ch    => 26,
            K     => 27,
            T     => 28,
            P     => 29,
            H     => 30,
        }
    }
    
    pub fn as_char(&self) -> char {
        let mut code = CONSONANT_START;
        code += self.char_offset();
        let res = char::from_u32(code)
            .expect("hangeul2 constructed an invalid hangeul character!");
        res
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub struct Block {
    pub initial: Initial,
    pub vowel: Vowel,
    pub fin: Final,
}

impl Block {
    pub fn from_parts(init: Initial, vowel: Vowel, fin: Final) -> Block {
        Block {
            initial: init, 
            vowel: vowel, 
            fin: fin,
        }
    }
    
    pub fn combine(&self) -> char {
        let mut code = BLOCK_START;
        code += self.initial.offset() * CHARS_PER_INITIAL;
        code += self.vowel.offset() * CHARS_PER_VOWEL;
        code += self.fin.offset();
        let res = char::from_u32(code)
            .expect("hangeul2 constructed an invalid hangeul character!");
        res
    }
}
