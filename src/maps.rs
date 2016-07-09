extern crate phf;

// Maps from an "initial" consonant to a final consonant
pub const FINAL_MAP: phf::Map<u32, u32> = phf_map! {
    // 0, //
    0u32 => 1, // ㄱ
    1u32 => 2, // ㄲ
    // 3, // ㄱㅅ
    2u32 => 4, // ㄴ
    // 5, // ㄴㅈ
    // 6, // ㄴㅎ
    3u32 => 7, // ㄷ
    5u32 => 8, // ㄹ
    // 9, // ㄹㄱ
    // 10, // ㄹㅁ
    // 11, // ㄹㅂ
    // 12, // ㄹㅅ
    // 13, // ㄹㅌ
    // 14, // ㄹㅍ
    // 15, // ㄹㅎ
    6u32 => 16, // ㅁ
    7u32 => 17, // ㅂ
    // 18, // ㅂㅅ
    9u32 => 19, // ㅅ
    10u32 => 20, // ㅆ
    11u32 => 21, // ㅇ
    12u32 => 22, // ㅈ
    14u32 => 23, // ㅊ
    15u32 => 24, // ㅋ
    16u32 => 25, // ㅌ
    17u32 => 26, // ㅍ
    18u32 => 27, // ㅎ
};
// FINALS = " ㄱ ㄲ ㄱㅅ ㄴ ㄴㅈ ㄴㅎ ㄷ ㄹ ㄹㄱ ㄹㅁ ㄹㅂ ㄹㅅ ㄹㅌ ㄹㅍ ㄹㅎ ㅁ ㅂ ㅂㅅ ㅅ ㅆ ㅇ ㅈ ㅊ ㅋ ㅌ ㅍ ㅎ".split(" ")

// maps from a MAPPED first final consonant
// to valid INITIAL consonants that may follow it
// (and how much they offset the value of the first consonant)
pub const FINAL_COMBINATION_MAP: phf::Map<u32, phf::Map<u32, u32>> = phf_map! {
    1u32 => phf_map! {
        9u32 => 1, // ㄱㅅ
    },
    4u32 => phf_map! {
        12u32 => 1, // ㄴㅈ
        18u32 => 2, // ㄴㅎ
    },
    5u32 => phf_map! {
        0u32 => 1, // ㄹㄱ
        6u32 => 2, // ㄹㅁ
        7u32 => 3, // ㄹㅂ
        9u32 => 4, // ㄹㅅ
        16u32 => 5, // ㄹㅌ
        17u32 => 6, // ㄹㅍ
        18u32 => 7, // ㄹㅎ
    },
    7u32 => phf_map! {
        9u32 => 1, // ㅂㅅ
    }
};

pub enum PhfTrie<T: 'static> {
    Leaf(T),
    Node(T, phf::Map<char, PhfTrie<T>>),
}

use self::PhfTrie::*;

pub const VOWELS: phf::Map<char, PhfTrie<Option<u32>>> = phf_map! {
    'a' => Node(Some(0), phf_map! { // 아
        'e' => Leaf(Some(1)), // 애
    }),
    'e' => Node(Some(5), phf_map! { // 에
        'o' => Leaf(Some(4)), // 어
    }),
    'i' => Leaf(Some(20)), // 이
    'o' => Node(Some(8), phf_map! { // 오
        'e' => Leaf(Some(11)), // 외
    }),
    'u' => Leaf(Some(13)), // 우
    'w' => Node(None, phf_map! {
        'a' => Node(Some(9), phf_map! { // 와
            'e' => Leaf(Some(10)), // 왜
        }),
        'e' => Node(Some(15), phf_map! { // 웨
            'o' => Leaf(Some(14)), // 워
        }),
        'i' => Leaf(Some(16)), // 위
    }),
    'y' => Node(Some(18), phf_map! { // ㅡ
        'a' => Node(Some(2), phf_map! { // 야
            'e' => Leaf(Some(3)), // 얘
        }),
        'e' => Node(Some(7), phf_map! { // 예
            'o' => Leaf(Some(6)), // 여
        }),
        'i' => Leaf(Some(19)), // 의
        'u' => Leaf(Some(17)), // 유
        'o' => Leaf(Some(12)), // 요
    }),
};

pub const CONSONANTS: phf::Map<char, PhfTrie<Option<u32>>> = phf_map! {
    'g' => Leaf(Some(0)), // ㄱ
    'G' => Leaf(Some(1)), // ㄲ
    'n' => Leaf(Some(2)), // ㄴ
    'd' => Leaf(Some(3)), // ㄷ
    'D' => Leaf(Some(4)), // ㄸ
    'r' => Leaf(Some(5)), // ㄹ
    'l' => Leaf(Some(5)), // ㄹ
    'm' => Leaf(Some(6)), // ㅁ
    'b' => Leaf(Some(7)), // ㅂ
    'B' => Leaf(Some(8)), // ㅃ
    's' => Leaf(Some(9)), // ㅅ
    'S' => Leaf(Some(10)), // ㅆ
    'x' => Leaf(Some(11)), // ㅇ
    'j' => Leaf(Some(12)), // ㅈ
    'J' => Leaf(Some(13)), // ㅉ
    'c' => Node(None, phf_map! {
        'h' => Leaf(Some(14)),
    }),
    'k' => Leaf(Some(15)), // ㅋ
    't' => Leaf(Some(16)), // ㅌ
    'p' => Leaf(Some(17)), // ㅍ
    'h' => Leaf(Some(18)), // ㅎ
};

pub const VALID_LETTERS: phf::Set<char> = phf_set! {
    'g',
    'G',
    'n',
    'd',
    'D',
    'r',
    'l',
    'm',
    'b',
    'B',
    's',
    'S',
    'x',
    'j',
    'J',
    'c',
    'k',
    't',
    'p',
    'h',
    
    'a',
    'e',
    'i',
    'o',
    'u',
    'w',
    'y'
};

