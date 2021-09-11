#![allow(dead_code)]

use num_bigint::BigUint;
use std::collections::HashMap;

// Non-opcode gas prices
const GDEFAULT: usize = 1;
const GMEMORY: usize = 3;
const GQUADRATICMEMDENOM: usize = 512; // 1 gas per 512 quadwords
const GSTORAGEREFUND: usize = 15000;
const GSTORAGEKILL: usize = 5000;
const GSTORAGEMOD: usize = 5000;
const GSTORAGEADD: usize = 20000;
const GEXPONENTBYTE: usize = 10; // cost of EXP exponent per byte
const GCOPY: usize = 3; // cost to copy one 32 byte word
const GCONTRACTBYTE: usize = 200; // one byte of code in contract creation
const GCALLVALUETRANSFER: usize = 9000; // non-zero-valued call
const GLOGBYTE: usize = 8; // cost of a byte of logdata

const GTXCOST: usize = 21000; // TX BASE GAS COST
const GTXDATAZERO: usize = 4; // TX DATA ZERO BYTE GAS COST
const GTXDATANONZERO: usize = 68; // TX DATA NON ZERO BYTE GAS COST
const GSHA3WORD: usize = 6; // Cost of SHA3 per word
const GSHA256BASE: usize = 60; // Base c of SHA256
const GSHA256WORD: usize = 12; // Cost of SHA256 per word
const GRIPEMD160BASE: usize = 600; // Base cost of RIPEMD160
const GRIPEMD160WORD: usize = 120; // Cost of RIPEMD160 per word
const GIDENTITYBASE: usize = 15; // Base cost of indentity
const GIDENTITYWORD: usize = 3; // Cost of identity per word
const GECRECOVER: usize = 3000; // Cost of ecrecover op

const GSTIPEND: usize = 2300;

const GCALLNEWACCOUNT: usize = 25000;
const GSUICIDEREFUND: usize = 24000;

pub struct Stack {
    pc: usize,
    calldata_i: usize,
    stack: Vec<[u8; 32]>,
    mem: Vec<u8>,
    gas: u64,
    opcodes: HashMap<u8, Opcode>,
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
        s.opcodes = new_opcodes();
        s
    }
    fn push(&mut self, b: [u8; 32]) {
        self.stack.push(b);
    }
    // push_arbitrary performs a push, but first converting the arbitrary-length input into a 32
    // byte array
    fn push_arbitrary(&mut self, b: &[u8]) {
        // TODO if b.len()>32 return error
        let mut d: [u8; 32] = [0; 32];
        d[32 - b.len()..].copy_from_slice(&b[..]);
        self.stack.push(d);
    }
    fn pop(&mut self) -> [u8; 32] {
        match self.stack.pop() {
            Some(x) => return x,
            None => panic!("err"),
        }
    }
    fn execute(&mut self, code: &[u8], calldata: &[u8], debug: bool) -> Vec<u8> {
        self.pc = 0;
        self.calldata_i = 0;
        let l = code.len();

        while self.pc < l {
            let opcode = code[self.pc];
            if !self.opcodes.contains_key(&opcode) {
                panic!("invalid opcode {:x}", opcode);
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
                0x50 => {
                    self.pc += 1;
                    match opcode {
                        0x52 => self.mstore(),
                        _ => panic!("unimplemented {:x}", opcode),
                    }
                }
                0x60 | 0x70 => {
                    // push
                    let n = (opcode - 0x5f) as usize;
                    self.push_arbitrary(&code[self.pc + 1..self.pc + 1 + n]);
                    self.pc += 1 + n;
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
        return Vec::new();
    }

    // arithmetic
    // TODO instead of [u8;32] converted to BigUint, use custom type uint256 that implements all
    // the arithmetic
    fn add(&mut self) {
        let b0 = BigUint::from_bytes_be(&self.pop()[..]);
        let b1 = BigUint::from_bytes_be(&self.pop()[..]);
        self.push_arbitrary(&(b0 + b1).to_bytes_be());
    }
    fn mul(&mut self) {
        let b0 = BigUint::from_bytes_be(&self.pop()[..]);
        let b1 = BigUint::from_bytes_be(&self.pop()[..]);
        self.push_arbitrary(&(b0 * b1).to_bytes_be());
    }
    fn sub(&mut self) {
        let b0 = BigUint::from_bytes_be(&self.pop()[..]);
        let b1 = BigUint::from_bytes_be(&self.pop()[..]);
        if b0 >= b1 {
            self.push_arbitrary(&(b0 - b1).to_bytes_be());
        } else {
            // 2**256
            let max =
                "115792089237316195423570985008687907853269984665640564039457584007913129639936"
                    .parse::<BigUint>()
                    .unwrap();
            self.push_arbitrary(&(max + b0 - b1).to_bytes_be());
        }
    }
    fn div(&mut self) {
        let b0 = BigUint::from_bytes_be(&self.pop()[..]);
        let b1 = BigUint::from_bytes_be(&self.pop()[..]);
        self.push_arbitrary(&(b0 / b1).to_bytes_be());
    }
    fn sdiv(&mut self) {
        panic!("unimplemented");
    }
    fn modulus(&mut self) {
        let b0 = BigUint::from_bytes_be(&self.pop()[..]);
        let b1 = BigUint::from_bytes_be(&self.pop()[..]);
        self.push_arbitrary(&(b0 % b1).to_bytes_be());
    }
    fn smod(&mut self) {
        panic!("unimplemented");
    }
    fn add_mod(&mut self) {
        let b0 = BigUint::from_bytes_be(&self.pop()[..]);
        let b1 = BigUint::from_bytes_be(&self.pop()[..]);
        let b2 = BigUint::from_bytes_be(&self.pop()[..]);
        self.push_arbitrary(&(b0 + b1 % b2).to_bytes_be());
    }
    fn mul_mod(&mut self) {
        let b0 = BigUint::from_bytes_be(&self.pop()[..]);
        let b1 = BigUint::from_bytes_be(&self.pop()[..]);
        let b2 = BigUint::from_bytes_be(&self.pop()[..]);
        self.push_arbitrary(&(b0 * b1 % b2).to_bytes_be());
    }
    fn exp(&mut self) {
        panic!("unimplemented");
        // let b0 = BigUint::from_bytes_be(&self.pop()[..]);
        // let b1 = BigUint::from_bytes_be(&self.pop()[..]);
        // self.push_arbitrary(&(pow(b0, b1)).to_bytes_be());
    }

    // boolean
    // crypto

    // contract context
    fn calldata_load(&mut self, calldata: &[u8]) {}

    // blockchain context

    // storage and execution
    fn extend_mem(&mut self, start: usize, size: usize) {
        if size <= self.mem.len() || start + size <= self.mem.len() {
            return;
        }
        let old_size = self.mem.len() / 32;
        let new_size = (start + size) / 32;
        let old_total_fee = old_size * GMEMORY + old_size.pow(2) / GQUADRATICMEMDENOM;
        let new_total_fee = new_size * GMEMORY + new_size.pow(2) / GQUADRATICMEMDENOM;
        let mem_fee = new_total_fee - old_total_fee;
        self.gas -= mem_fee as u64;
        let mut new_bytes: Vec<u8> = vec![0; size];
        self.mem.append(&mut new_bytes);
    }
    fn mstore(&mut self) {
        let pos = u256_to_u64(self.pop());
        let val = self.pop();
        self.extend_mem(pos as usize, 32);

        self.mem[pos as usize..].copy_from_slice(&val);
    }
}

fn u256_to_u64(a: [u8; 32]) -> u64 {
    let mut b8: [u8; 8] = [0; 8];
    b8.copy_from_slice(&a[32 - 8..32]);
    let pos = u64::from_be_bytes(b8);
    pos
}
fn str_to_u256(s: &str) -> [u8; 32] {
    let bi = s.parse::<BigUint>().unwrap().to_bytes_be();
    let mut r: [u8; 32] = [0; 32];
    r[32 - bi.len()..].copy_from_slice(&bi[..]);
    r
}

struct Opcode {
    name: String,
    ins: u32,
    outs: u32,
    gas: u64,
}

fn new_opcode(name: &str, ins: u32, outs: u32, gas: u64) -> Opcode {
    Opcode {
        name: name.to_string(),
        ins,
        outs,
        gas,
    }
}

fn new_opcodes() -> HashMap<u8, Opcode> {
    let mut opcodes: HashMap<u8, Opcode> = HashMap::new();

    // arithmetic
    opcodes.insert(0x00, new_opcode("STOP", 0, 0, 0));
    opcodes.insert(0x01, new_opcode("ADD", 2, 1, 3));
    opcodes.insert(0x02, new_opcode("MUL", 2, 1, 5));
    opcodes.insert(0x03, new_opcode("SUB", 2, 1, 3));
    opcodes.insert(0x04, new_opcode("DIV", 2, 1, 5));
    opcodes.insert(0x05, new_opcode("SDIV", 2, 1, 5));
    opcodes.insert(0x06, new_opcode("MOD", 2, 1, 5));
    opcodes.insert(0x07, new_opcode("SMOD", 2, 1, 5));
    opcodes.insert(0x08, new_opcode("ADDMOD", 3, 1, 8));
    opcodes.insert(0x09, new_opcode("MULMOD", 3, 1, 8));
    opcodes.insert(0x0a, new_opcode("EXP", 2, 1, 10));
    opcodes.insert(0x0b, new_opcode("SIGNEXTEND", 2, 1, 5));

    // boolean
    opcodes.insert(0x10, new_opcode("LT", 2, 1, 3));
    opcodes.insert(0x11, new_opcode("GT", 2, 1, 3));
    opcodes.insert(0x12, new_opcode("SLT", 2, 1, 3));
    opcodes.insert(0x13, new_opcode("SGT", 2, 1, 3));
    opcodes.insert(0x14, new_opcode("EQ", 2, 1, 3));
    opcodes.insert(0x15, new_opcode("ISZERO", 1, 1, 3));
    opcodes.insert(0x16, new_opcode("AND", 2, 1, 3));
    opcodes.insert(0x17, new_opcode("OR", 2, 1, 3));
    opcodes.insert(0x18, new_opcode("XOR", 2, 1, 3));
    opcodes.insert(0x19, new_opcode("NOT", 1, 1, 3));
    opcodes.insert(0x1a, new_opcode("BYTE", 2, 1, 3));

    // crypto
    opcodes.insert(0x20, new_opcode("SHA3", 2, 1, 30));

    // contract context
    opcodes.insert(0x30, new_opcode("ADDRESS", 0, 1, 2));
    opcodes.insert(0x31, new_opcode("BALANCE", 1, 1, 20));
    opcodes.insert(0x32, new_opcode("ORIGIN", 0, 1, 2));
    opcodes.insert(0x33, new_opcode("CALLER", 0, 1, 2));
    opcodes.insert(0x34, new_opcode("CALLVALUE", 0, 1, 2));
    opcodes.insert(0x35, new_opcode("CALLDATALOAD", 1, 1, 3));
    opcodes.insert(0x36, new_opcode("CALLDATASIZE", 0, 1, 2));
    opcodes.insert(0x37, new_opcode("CALLDATACOPY", 3, 0, 3));
    opcodes.insert(0x38, new_opcode("CODESIZE", 0, 1, 2));
    opcodes.insert(0x39, new_opcode("CODECOPY", 3, 0, 3));
    opcodes.insert(0x3a, new_opcode("GASPRICE", 0, 1, 2));
    opcodes.insert(0x3b, new_opcode("EXTCODESIZE", 1, 1, 20));
    opcodes.insert(0x3c, new_opcode("EXTCODECOPY", 4, 0, 20));

    // blockchain context
    opcodes.insert(0x40, new_opcode("BLOCKHASH", 1, 1, 20));
    opcodes.insert(0x41, new_opcode("COINBASE", 0, 1, 2));
    opcodes.insert(0x42, new_opcode("TIMESTAMP", 0, 1, 2));
    opcodes.insert(0x43, new_opcode("NUMBER", 0, 1, 2));
    opcodes.insert(0x44, new_opcode("DIFFICULTY", 0, 1, 2));
    opcodes.insert(0x45, new_opcode("GASLIMIT", 0, 1, 2));

    // storage and execution
    opcodes.insert(0x50, new_opcode("POP", 1, 0, 2));
    opcodes.insert(0x51, new_opcode("MLOAD", 1, 1, 3));
    opcodes.insert(0x52, new_opcode("MSTORE", 2, 0, 3));
    opcodes.insert(0x53, new_opcode("MSTORE8", 2, 0, 3));
    opcodes.insert(0x54, new_opcode("SLOAD", 1, 1, 50));
    opcodes.insert(0x55, new_opcode("SSTORE", 2, 0, 0));
    opcodes.insert(0x56, new_opcode("JUMP", 1, 0, 8));
    opcodes.insert(0x57, new_opcode("JUMPI", 2, 0, 10));
    opcodes.insert(0x58, new_opcode("PC", 0, 1, 2));
    opcodes.insert(0x59, new_opcode("MSIZE", 0, 1, 2));
    opcodes.insert(0x5a, new_opcode("GAS", 0, 1, 2));
    opcodes.insert(0x5b, new_opcode("JUMPDEST", 0, 0, 1));

    // logging
    opcodes.insert(0xa0, new_opcode("LOG0", 2, 0, 375));
    opcodes.insert(0xa1, new_opcode("LOG1", 3, 0, 750));
    opcodes.insert(0xa2, new_opcode("LOG2", 4, 0, 1125));
    opcodes.insert(0xa3, new_opcode("LOG3", 5, 0, 1500));
    opcodes.insert(0xa4, new_opcode("LOG4", 6, 0, 1875));

    // closures
    opcodes.insert(0xf0, new_opcode("CREATE", 3, 1, 32000));
    opcodes.insert(0xf1, new_opcode("CALL", 7, 1, 40));
    opcodes.insert(0xf2, new_opcode("CALLCODE", 7, 1, 40));
    opcodes.insert(0xf3, new_opcode("RETURN", 2, 0, 0));
    opcodes.insert(0xf4, new_opcode("DELEGATECALL", 6, 0, 40));
    opcodes.insert(0xff, new_opcode("SUICIDE", 1, 0, 0));

    for i in 1..33 {
        let name = format!("PUSH{}", i);
        opcodes.insert(0x5f + i, new_opcode(&name, 0, 1, 3));
    }

    for i in 1..17 {
        let name = format!("DUP{}", i);
        opcodes.insert(0x7f + i, new_opcode(&name, i as u32, i as u32 + 1, 3));

        let name = format!("SWAP{}", i);
        opcodes.insert(0x8f + i, new_opcode(&name, i as u32 + 1, i as u32 + 1, 3));
    }

    opcodes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stack_simple_push_pop() {
        let mut s = Stack::new();
        s.push(str_to_u256("1"));
        s.push(str_to_u256("2"));
        s.push(str_to_u256("3"));
        assert_eq!(s.pop(), str_to_u256("3"));
        assert_eq!(s.pop(), str_to_u256("2"));
        assert_eq!(s.pop(), str_to_u256("1"));
        // assert_eq!(s.pop(), str_to_u256("1"));
        // assert_eq!(s.pop(), error); // TODO expect error as stack is empty
    }

    // arithmetic
    #[test]
    fn execute_opcodes_0() {
        let code = hex::decode("6005600c01").unwrap(); // 5+12
        let calldata = vec![];

        let mut s = Stack::new();
        s.execute(&code, &calldata, false);
        assert_eq!(s.pop(), str_to_u256("17"));
        assert_eq!(s.gas, 9999999991);
        assert_eq!(s.pc, 5);
    }

    #[test]
    fn execute_opcodes_1() {
        let code = hex::decode("60056004016000526001601ff3").unwrap();
        let calldata = vec![];

        let mut s = Stack::new();
        let out = s.execute(&code, &calldata, false);

        assert_eq!(out[0], 0x09);
        assert_eq!(s.gas, 9999999976);
        assert_eq!(s.pc, 12);
        // assert_eq!(s.pop(), err); // TODO expect error as stack is empty
    }

    #[test]
    fn execute_opcodes_2() {
        let code = hex::decode("61010161010201").unwrap();
        let calldata = vec![];

        let mut s = Stack::new();
        s.execute(&code, &calldata, false);

        // assert_eq!(out[0], 0x09);
        assert_eq!(s.gas, 9999999991);
        assert_eq!(s.pc, 7);
        assert_eq!(s.pop(), str_to_u256("515"));
    }
}
