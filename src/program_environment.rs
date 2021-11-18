use std::collections::HashMap;
use std::vec::Vec;

use crate::utility;

pub static INSTRUCTION_OP_CODES: &[(&str, &[(AddressingMode, u8)])] = &[
    ("mov", &[
        (AddressingMode::Absolute, 0x01),
        (AddressingMode::Immediate, 0x02),
        (AddressingMode::Registry, 0x03),
        (AddressingMode::RegistryPointer, 0x04)
    ]),
    ("cmp", &[
        (AddressingMode::Immediate, 0x05)
    ]),
    ("jne", &[
        (AddressingMode::Absolute, 0x06)
    ]),
    ("pusha", &[
        (AddressingMode::Implicit, 0x07)
    ]),
    ("popa", &[
        (AddressingMode::Implicit, 0x00)
    ]),
    ("jmp", &[
        (AddressingMode::Absolute, 0x09)
    ]),
    ("call", &[
        (AddressingMode::Absolute, 0x0A)
    ]),
    ("ret", &[
        (AddressingMode::Implicit, 0x0B)
    ]),
    ("prtc", &[
        (AddressingMode::Immediate, 0x0C),
        (AddressingMode::Registry, 0x0D),
        (AddressingMode::RegistryPointer, 0x0E)
    ]),
    ("add", &[
        (AddressingMode::Immediate, 0x0F)
    ])
];

pub static REGISTER_ENCODING: &[(&str, u8)] = &[
    ( "al", 0x00 ), ( "bl", 0x01 ), ( "cl", 0x02 ), ( "dl", 0x03 ), ( "ah", 0x04 ), ( "bh", 0x05 ),
    ( "ch", 0x06 ), ( "dh", 0x07 ), // 8-bit registers

    ( "ax", 0x08 ), ( "bx", 0x09 ), ( "cx", 0x0A ), ( "dx", 0x0B ), ( "si", 0x0C ), ( "di", 0x0D ),
    ( "sp", 0x0E ), ( "bp", 0x0F ), // 16-bit registers

    ( "r8", 0x10 ), ( "r9", 0x11 ), ( "r10", 0x12 ), ( "r11", 0x13 ), ( "r12", 0x14 ),
    ( "r13", 0x15 ), ( "r14", 0x16 ), ( "r15", 0x17 ) // 64-bit registers
]; // I check for sizes with these assigned encoding values in the assembler. 
   // THIS IS VERY ERROR PRONE IF I CHANGE THIS MAP IN ANY WAY.

enum AddressingMode { Implicit, Immediate, Absolute, Registry, RegistryPointer }

pub struct ProgramEnvironment {
    program_code: String,
    dump_memory: bool, dump_full: bool,

    // MEM
    memory: [u8; 0xFFFF],

    // CPU

    // Registers - 16 bit
    pc: u16, ax: u16, bx: u16, cx: u16, dx: u16, si: u16, di: u16, sp: u16, bp: u16,

    // 64 bit
    r8: u64, r9: u64, r10: u64, r11: u64, r12: u64, r13: u64, r14: u64, r15: u64,

    // Flags
    cf: bool
}

impl ProgramEnvironment {
    pub fn new(program_code: String, dump_memory: bool, dump_full: bool) -> Self {
        ProgramEnvironment {
            program_code, dump_memory, dump_full,

            memory: [0; 0xFFFF],

            pc: 200, ax: 0, bx: 0, cx: 0, dx: 0, si: 0, di: 0, sp: 0, bp: 0,
            r8: 0, r9: 0, r10: 0, r11: 0, r12: 0, r13: 0, r14: 0, r15: 0,

            cf: false
        }
    }

    pub fn get_register_value(&self, byte: u8) -> u64 {
        match byte {
            0x00 => utility::read_lower_16(self.ax) as u64,
            0x01 => utility::read_lower_16(self.bx) as u64,
            0x02 => utility::read_lower_16(self.cx) as u64,
            0x03 => utility::read_lower_16(self.dx) as u64,
            0x04 => utility::read_upper_16(self.ax) as u64,
            0x05 => utility::read_upper_16(self.bx) as u64,
            0x06 => utility::read_upper_16(self.cx) as u64,
            0x07 => utility::read_upper_16(self.dx) as u64,
    
            0x08 => self.ax as u64,
            0x09 => self.bx as u64,
            0x0A => self.cx as u64,
            0x0B => self.dx as u64,
            0x0C => self.si as u64,
            0x0D => self.di as u64,
            0x0E => self.sp as u64,
            0x0F => self.bp as u64,
    
            0x10 => self.r8,
            0x11 => self.r9,
            0x12 => self.r10,
            0x13 => self.r11,
            0x14 => self.r12,
            0x15 => self.r13,
            0x16 => self.r14,
            0x17 => self.r15,
            _ => panic!("Indexed non existing register")
        }
    }

    pub fn set_register_value(&mut self, byte: u8, value: u64) {
        match byte {
            0x00 => utility::set_lower_16(&mut self.ax, value as u8),
            0x01 => utility::set_lower_16(&mut self.bx, value as u8),
            0x02 => utility::set_lower_16(&mut self.cx, value as u8),
            0x03 => utility::set_lower_16(&mut self.dx, value as u8),
            0x04 => utility::set_upper_16(&mut self.ax, value as u8),
            0x05 => utility::set_upper_16(&mut self.bx, value as u8),
            0x06 => utility::set_upper_16(&mut self.cx, value as u8),
            0x07 => utility::set_upper_16(&mut self.dx, value as u8),
    
            0x08 => self.ax = value as u16,
            0x09 => self.bx = value as u16,
            0x0A => self.cx = value as u16,
            0x0B => self.dx = value as u16,
            0x0C => self.si = value as u16,
            0x0D => self.di = value as u16,
            0x0E => self.sp = value as u16,
            0x0F => self.bp = value as u16,
    
            0x10 => self.r8 = value,
            0x11 => self.r9 = value,
            0x12 => self.r10 = value,
            0x13 => self.r11 = value,
            0x14 => self.r12 = value,
            0x15 => self.r13 = value,
            0x16 => self.r14 = value,
            0x17 => self.r15 = value,
            _ => panic!("Indexed non existing register")
        }
    }

    pub fn compile(&self) -> bool {
        let labels: HashMap<String, u16> = HashMap::new();
        let labelReferences: HashMap<String, u16> = HashMap::new();
    
        let mut compilerPointer: u16 = 0x200;

        let mut lines: Vec<&str> = self.program_code.lines().collect();

        for line in 0..=lines.len() {
            if lines.len() == 0 { continue; } // Empty line

            loop { // In case of labels with inline instructions



            }
        };
    }
}