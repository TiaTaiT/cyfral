use crate::constants::{KEY_BUFFER_CAPACITY, KEY_CODONS_AMOUNT, LEVEL_BUFFER_CAPACITY, LOGIC_BUFFER_CAPACITY};

const START: [bool; 4] = [false, false, false, true];

const VALID_WORDS: [[bool; 4]; 4] = [
    [true, true, true, false],
    [true, true, false, true],
    [true, false, true, true],
    [false, true, true, true],
];

pub fn decode(level_buf: [bool; LEVEL_BUFFER_CAPACITY]) -> [bool; LOGIC_BUFFER_CAPACITY] {
    let mut result = [false; LOGIC_BUFFER_CAPACITY];
    let mut out_idx = 0;
    let mut segment_start = 0;
    for i in 1..LEVEL_BUFFER_CAPACITY {
        if level_buf[i - 1] && !level_buf[i] {
            // Process segment [segment_start .. i)
            let mut true_count = 0;
            for j in segment_start..i {
                if level_buf[j] {
                    true_count += 1;
                }
            }
            let seg_len = i - segment_start;
            let false_count = seg_len - true_count;
            if out_idx < LOGIC_BUFFER_CAPACITY {
                result[out_idx] = true_count >= false_count;
                out_idx += 1;
            }
            segment_start = i;
        }
    }
    // Last segment
    if segment_start < LEVEL_BUFFER_CAPACITY {
        let mut true_count = 0;
        for j in segment_start..LEVEL_BUFFER_CAPACITY {
            if level_buf[j] {
                true_count += 1;
            }
        }
        let seg_len = LEVEL_BUFFER_CAPACITY - segment_start;
        let false_count = seg_len - true_count;
        if out_idx < LOGIC_BUFFER_CAPACITY {
            result[out_idx] = true_count >= false_count;
            out_idx += 1;
        }
    }
    result
}

pub fn get_key(logic_buf: [bool; LOGIC_BUFFER_CAPACITY]) -> Option<[bool; KEY_BUFFER_CAPACITY]> {
    let mut result = [false; KEY_BUFFER_CAPACITY];

    // Find the first occurrence of the start marker sequence [false, false, false, true]
    let mut marker_pos = None;
    for i in 0..LOGIC_BUFFER_CAPACITY.saturating_sub(3) {
        if logic_buf[i] == START[0]
            && logic_buf[i + 1] == START[1]
            && logic_buf[i + 2] == START[2]
            && logic_buf[i + 3] == START[3]
        {
            marker_pos = Some(i);
            break;
        }
    }

    let start = match marker_pos {
        Some(pos) => pos + 4,
        None => return None,
    };

    // We need exactly 8 words (32 bits) after the marker
    if start + 32 > LOGIC_BUFFER_CAPACITY {
        return None;
    }

    let mut out_idx = 0;
    for word_idx in 0..8 {
        let base = start + word_idx * 4;
        let word = [
            logic_buf[base],
            logic_buf[base + 1],
            logic_buf[base + 2],
            logic_buf[base + 3],
        ];

        let mut valid = false;
        for &valid_word in &VALID_WORDS {
            if word == valid_word {
                valid = true;
                break;
            }
        }

        if !valid {
            return None;
        }

        // Copy the 4 bits into result
        for j in 0..4 {
            result[out_idx] = word[j];
            out_idx += 1;
        }
    }

    Some(result)
}

pub fn get_digit_key(key_buf: [bool; KEY_BUFFER_CAPACITY]) -> [u8; KEY_CODONS_AMOUNT] {
    let mut result = [0u8; KEY_CODONS_AMOUNT];

    for i in 0..KEY_CODONS_AMOUNT {
        let base = i * 4;
        let word = [
            key_buf[base],
            key_buf[base + 1],
            key_buf[base + 2],
            key_buf[base + 3],
        ];

        result[i] = match word {
            [true, true, true, false] => 0,   // 1110
            [true, true, false, true] => 1,   // 1101
            [true, false, true, true] => 2,   // 1011
            [false, true, true, true] => 3,   // 0111
            _ => 0, // fallback (should not happen if validated by get_key)
        };
    }

    result
}

