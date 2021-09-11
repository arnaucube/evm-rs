#![allow(dead_code)]

// use num::pow::pow;
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
    fn pop(&mut self) -> [u8; 32] {
        match self.stack.pop() {
            Some(x) => return x,
            None => panic!("err"),
        }
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
}
