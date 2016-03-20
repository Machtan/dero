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
