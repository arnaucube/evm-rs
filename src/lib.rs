#![allow(dead_code)]

use num_bigint::BigUint;
use std::collections::HashMap;
pub mod opcodes;

#[derive(Default)]
pub struct Stack {
    pub pc: usize,
    pub calldata_i: usize,
    pub stack: Vec<[u8; 32]>,
    pub mem: Vec<u8>,
    pub gas: u64,
    pub opcodes: HashMap<u8, opcodes::Opcode>,
}

impl Stack {
    pub fn new() -> Stack {
        let mut s = Stack {
            pc: 0,
            calldata_i: 0,
            stack: Vec::new(),
            mem: Vec::new(),
            gas: 10000000000,
            opcodes: HashMap::new(),
        };
        s.opcodes = opcodes::new_opcodes();
        s
    }
    pub fn print_stack(&self) {
        for i in (0..self.stack.len()).rev() {
            println!("{:x?}", &self.stack[i][28..]);
        }
    }
    pub fn push(&mut self, b: [u8; 32]) {
        self.stack.push(b);
    }
    // push_arbitrary performs a push, but first converting the arbitrary-length
    // input into a 32 byte array
    pub fn push_arbitrary(&mut self, b: &[u8]) {
        // TODO if b.len()>32 return error
        let mut d: [u8; 32] = [0; 32];
        d[32 - b.len()..].copy_from_slice(b);
        self.stack.push(d);
    }
    // put_arbitrary puts in the last element of the stack the value
    pub fn put_arbitrary(&mut self, b: &[u8]) {
        // TODO if b.len()>32 return error
        let mut d: [u8; 32] = [0; 32];
        d[32 - b.len()..].copy_from_slice(b);
        let l = self.stack.len();
        self.stack[l - 1] = d;
    }
    pub fn pop(&mut self) -> [u8; 32] {
        match self.stack.pop() {
            Some(x) => x,
            None => panic!("err"),
        }
    }

    pub fn execute(&mut self, code: &[u8], calldata: &[u8], debug: bool) -> Vec<u8> {
        self.pc = 0;
        self.calldata_i = 0;
        let l = code.len();

        while self.pc < l {
            let opcode = code[self.pc];
            if !self.opcodes.contains_key(&opcode) {
                panic!("invalid opcode {:x}", opcode);
            }

            if debug {
                println!(
                    "{:?} (0x{:x}): pc={:?} gas={:?}\nstack:",
                    self.opcodes.get(&opcode).unwrap().name,
                    opcode,
                    self.pc,
                    self.gas,
                );
                self.print_stack();
                println!();
            }
            match opcode & 0xf0 {
                0x00 => {
                    // arithmetic
                    match opcode {
                        0x00 => {
                            return Vec::new();
                        }
                        0x01 => self.add(),
                        0x02 => self.mul(),
                        0x03 => self.sub(),
                        0x04 => self.div(),
                        0x05 => self.sdiv(),
                        0x06 => self.modulus(),
                        0x07 => self.smod(),
                        0x08 => self.add_mod(),
                        0x09 => self.mul_mod(),
                        0x0a => self.exp(),
                        // 0x0b => self.sign_extend(),
                        _ => panic!("unimplemented {:x}", opcode),
                    }
                    self.pc += 1;
                }
                0x30 => {
                    match opcode {
                        0x35 => {
                            self.calldata_load(&calldata);
                        }
                        _ => panic!("unimplemented {:x}", opcode),
                    }
                    self.pc += 1;
                }
                0x50 => {
                    self.pc += 1;
                    match opcode {
                        0x51 => self.mload(),
                        0x52 => self.mstore(),
                        0x56 => self.jump(),
                        0x57 => self.jump_i(),
                        0x5b => self.jump_dest(),
                        _ => panic!("unimplemented {:x}", opcode),
                    }
                }
                0x60 | 0x70 => {
                    // push
                    let n = (opcode - 0x5f) as usize;
                    self.push_arbitrary(&code[self.pc + 1..self.pc + 1 + n]);
                    self.pc += 1 + n;
                }
                0x80 => {
                    // 0x8x dup
                    let l = self.stack.len();
                    if opcode > 0x7f {
                        self.stack.push(self.stack[l - (opcode - 0x7f) as usize]);
                    } else {
                        self.stack.push(self.stack[(0x7f - opcode) as usize]);
                    }
                    self.pc += 1;
                }
                0x90 => {
                    // 0x9x swap
                    let l = self.stack.len();
                    let pos;
                    if opcode > 0x8e {
                        pos = l - (opcode - 0x8e) as usize;
                    } else {
                        pos = (0x8e - opcode) as usize;
                    }
                    self.stack.swap(pos, l - 1);
                    self.pc += 1;
                }
                0xf0 => {
                    if opcode == 0xf3 {
                        let pos_to_return = u256_to_u64(self.pop()) as usize;
                        let len_to_return = u256_to_u64(self.pop()) as usize;
                        return self.mem[pos_to_return..pos_to_return + len_to_return].to_vec();
                    }
                }
                _ => {
                    panic!("unimplemented {:x}", opcode);
                }
            }
            self.gas -= self.opcodes.get(&opcode).unwrap().gas;
        }
        Vec::new()
    }
}

pub fn u256_to_u64(a: [u8; 32]) -> u64 {
    let mut b8: [u8; 8] = [0; 8];
    b8.copy_from_slice(&a[32 - 8..32]);
    u64::from_be_bytes(b8)
}
pub fn str_to_u256(s: &str) -> [u8; 32] {
    let bi = s.parse::<BigUint>().unwrap().to_bytes_be();
    let mut r: [u8; 32] = [0; 32];
    r[32 - bi.len()..].copy_from_slice(&bi[..]);
    r
}
