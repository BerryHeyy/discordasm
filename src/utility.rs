use std::iter::IntoIterator;

pub fn read_upper_16(word: u16) -> u8 {
    word as u8
}

pub fn read_lower_16(word: u16) -> u8 {
    (word >> 8) as u8
}

pub fn set_lower_16(word: &mut u16, byte: u8) {
    *word = (*word & 0xFF00) | (byte as u16);
}

pub fn set_upper_16(word: &mut u16, byte: u8) {
    *word = (*word & 0xFF) | ((byte as u16) << 8)
}

pub fn map_count<K: std::cmp::Eq, V>(map: &[(K, V)], check: K) -> u32 {
    let mut n = 0;

    for key in IntoIterator::into_iter(map) {
        if key.0 == check { n += 1; }
    };

    n
}

pub fn valid_hex_str(toCheck: &str) -> bool {
    (toCheck[0..3] == *"0x") && (toCheck.len() > 2) && (toCheck.chars().all(|x| "0123456789abcdefABCDEF".find(x) != None))
}

pub fn get_register_size(encoding: u8) -> u8 {
    if encoding <= 0x07 { 8 }
    else if encoding <= 0x0F { 16 }
    else { 64 }
}