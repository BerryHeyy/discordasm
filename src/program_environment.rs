pub struct ProgramEnvironment {
    program_code: String,
    dump_memory: bool, dump_full: bool,

    // MEM
    memory: [u8; 0xFFFF],

    // CPU

    // Registers - 16 bit
    pc: u16, ax: u16, rx: u16, cx: u16, dx: u16, si: u16, di: u16, sp: u16, bp: u16,

    // 64 bit
    r8: u64, r9: u64, r10: u64, r11: u64, r12: u64, r13: u64, r14: u64, r15: u64,

    // Flags
    cf: bool
}

fn read_upper_16(word: u16) -> u8 {
    word as u8
}

fn read_lower_16(word: u16) -> u8 {
    (word >> 8) as u8
}

fn set_lower_16(word: &mut u16, byte: u8) {
    *word = (*word & 0xFF00) | (byte as u16);
}

fn set_upper_16(word: &mut u16, byte: u8) {
    *word = (*word & 0xFF) | ((byte as u16) << 8)
}