use super::*;

// Non-opcode gas prices
const GDEFAULT: usize = 1;
const GMEMORY: usize = 3;
const GQUADRATICMEMDENOM: usize = 512; // 1 gas per 512 quadwords
const GSTORAGEREFUND: usize = 15000;
const GSTORAGEKILL: usize = 5000;
const GSTORAGEMOD: usize = 5000;
const GSTORAGEADD: usize = 20000;
const GEXPONENTBYTE: usize = 10; // cost of EXP exponent per byte
const EXP_SUPPLEMENTAL_GAS: usize = 40;
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

pub struct Opcode {
    pub name: String,
    pub ins: u32,
    pub outs: u32,
    pub gas: u64,
    // operation: fn(),
}

pub fn new_opcode(name: &str, ins: u32, outs: u32, gas: u64) -> Opcode {
    Opcode {
        name: name.to_string(),
        ins,
        outs,
        gas,
    }
}

pub fn new_opcodes() -> HashMap<u8, Opcode> {
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

impl Stack {
    // arithmetic
    // TODO instead of [u8;32] converted to BigUint, use custom type uint256 that implements all
    // the arithmetic
    pub fn add(&mut self) -> Result<(), String> {
        let b0 = BigUint::from_bytes_be(&self.pop()?[..]);
        let b1 = BigUint::from_bytes_be(&self.pop()?[..]);
        self.push_arbitrary(&(b0 + b1).to_bytes_be());
        Ok(())
    }
    pub fn mul(&mut self) -> Result<(), String> {
        let b0 = BigUint::from_bytes_be(&self.pop()?[..]);
        let b1 = BigUint::from_bytes_be(&self.pop()?[..]);
        self.push_arbitrary(&(b0 * b1).to_bytes_be());
        Ok(())
    }
    pub fn sub(&mut self) -> Result<(), String> {
        let b0 = BigUint::from_bytes_be(&self.pop()?[..]);
        let b1 = BigUint::from_bytes_be(&self.pop()?[..]);
        if b0 >= b1 {
            self.push_arbitrary(&(b0 - b1).to_bytes_be());
        } else {
            // 2**256 TODO this will not be here hardcoded, there will be a custom type uint256
            let max =
                "115792089237316195423570985008687907853269984665640564039457584007913129639936"
                    .parse::<BigUint>()
                    .unwrap();
            self.push_arbitrary(&(max + b0 - b1).to_bytes_be());
        }
        Ok(())
    }
    pub fn div(&mut self) -> Result<(), String> {
        let b0 = BigUint::from_bytes_be(&self.pop()?[..]);
        let b1 = BigUint::from_bytes_be(&self.pop()?[..]);
        self.push_arbitrary(&(b0 / b1).to_bytes_be());
        Ok(())
    }
    pub fn sdiv(&mut self) -> Result<(), String> {
        Err(format!("unimplemented"))
    }
    pub fn modulus(&mut self) -> Result<(), String> {
        let b0 = BigUint::from_bytes_be(&self.pop()?[..]);
        let b1 = BigUint::from_bytes_be(&self.pop()?[..]);
        self.push_arbitrary(&(b0 % b1).to_bytes_be());
        Ok(())
    }
    pub fn smod(&mut self) -> Result<(), String> {
        Err(format!("unimplemented"))
    }
    pub fn add_mod(&mut self) -> Result<(), String> {
        let b0 = BigUint::from_bytes_be(&self.pop()?[..]);
        let b1 = BigUint::from_bytes_be(&self.pop()?[..]);
        let b2 = BigUint::from_bytes_be(&self.pop()?[..]);
        self.push_arbitrary(&(b0 + b1 % b2).to_bytes_be());
        Ok(())
    }
    pub fn mul_mod(&mut self) -> Result<(), String> {
        let b0 = BigUint::from_bytes_be(&self.pop()?[..]);
        let b1 = BigUint::from_bytes_be(&self.pop()?[..]);
        let b2 = BigUint::from_bytes_be(&self.pop()?[..]);
        self.push_arbitrary(&(b0 * b1 % b2).to_bytes_be());
        Ok(())
    }
    pub fn exp(&mut self) -> Result<(), String> {
        let b = BigUint::from_bytes_be(&self.pop()?[..]);
        let e = BigUint::from_bytes_be(&self.pop()?[..]);

        let mut r = "1".parse::<BigUint>().unwrap();
        let zero = "0".parse::<BigUint>().unwrap();
        let mut rem = e.clone();
        let mut exp = b;
        // 2**256 TODO this will not be here hardcoded, there will be a custom type uint256
        let field =
            "115792089237316195423570985008687907853269984665640564039457584007913129639936"
                .parse::<BigUint>()
                .unwrap();
        while rem != zero {
            if rem.bit(0) {
                // is odd
                r = r * exp.clone() % field.clone();
            }
            exp = exp.clone() * exp.clone();
            rem >>= 1;
        }
        self.push_arbitrary(&r.to_bytes_be());

        let n_bytes = &e.to_bytes_be().len();
        let mut exp_fee = n_bytes * GEXPONENTBYTE;
        exp_fee += EXP_SUPPLEMENTAL_GAS * n_bytes;
        self.gas -= exp_fee as u64;
        Ok(())
    }

    // boolean
    // crypto

    // contract context
    pub fn calldata_load(&mut self, calldata: &[u8]) {
        self.put_arbitrary(&calldata[self.calldata_i..self.calldata_i + self.calldata_size]);
        self.calldata_i += self.calldata_size;
    }
    pub fn calldata_size(&mut self, calldata: &[u8]) {
        self.calldata_size = calldata.len();
        self.push(u256::usize_to_u256(self.calldata_size));
    }
    fn spend_gas_data_copy(&mut self, length: usize) {
        let length32 = upper_multiple_of_32(length);
        self.gas -= ((GCOPY * length32) / 32) as u64;
    }
    pub fn code_copy(&mut self, code: &[u8]) -> Result<(), String> {
        let dest_offset = u256::u256_to_u64(self.pop()?) as usize;
        let offset = u256::u256_to_u64(self.pop()?) as usize;
        let length = u256::u256_to_u64(self.pop()?) as usize;

        self.extend_mem(dest_offset, length);
        self.spend_gas_data_copy(length);

        for i in 0..length {
            if offset + i < code.len() {
                self.mem[dest_offset + i] = code[offset + i];
            } else {
                self.mem[dest_offset + i] = 0;
            }
        }
        // self.mem[dest_offset..dest_offset+length] =
        Ok(())
    }

    // blockchain context

    // storage and execution
    pub fn extend_mem(&mut self, start: usize, size: usize) {
        if size <= self.mem.len() || start + size <= self.mem.len() {
            return;
        }
        let old_size = self.mem.len() / 32;
        let new_size = upper_multiple_of_32(start + size) / 32;
        let old_total_fee = old_size * GMEMORY + old_size.pow(2) / GQUADRATICMEMDENOM;
        let new_total_fee = new_size * GMEMORY + new_size.pow(2) / GQUADRATICMEMDENOM;
        let mem_fee = new_total_fee - old_total_fee;
        self.gas -= mem_fee as u64;
        let mut new_bytes: Vec<u8> = vec![0; (new_size - old_size) * 32];
        self.mem.append(&mut new_bytes);
    }
    pub fn mload(&mut self) -> Result<(), String> {
        let pos = u256::u256_to_u64(self.pop()?) as usize;
        self.extend_mem(pos as usize, 32);
        let mem32 = self.mem[pos..pos + 32].to_vec();
        self.push_arbitrary(&mem32);
        Ok(())
    }
    pub fn mstore(&mut self) -> Result<(), String> {
        let pos = u256::u256_to_u64(self.pop()?);
        let val = self.pop()?;
        self.extend_mem(pos as usize, 32);

        self.mem[pos as usize..].copy_from_slice(&val);
        Ok(())
    }
    pub fn sstore(&mut self) -> Result<(), String> {
        let key = self.pop()?;
        let value = self.pop()?;
        if self.storage.contains_key(&key) {
            let old_value = self.storage.get(&key).unwrap();
            if &value.to_vec() == old_value {
                // if the new value is the same as the old one, do not set
                return Ok(());
            }
            // if value (from self.pop()) does not exist in the stack, is a STORAGEKILL TODO
            println!("mingas {:?}", GSTORAGEMOD);
            self.gas -= GSTORAGEMOD as u64;
        } else {
            // if value does not exist, substract gas for the addition
            println!("mingas {:?}", GSTORAGEADD);
            self.gas -= GSTORAGEADD as u64;
        }
        println!(
            "insert {:?} - {:?}",
            vec_u8_to_hex(key.to_vec()),
            vec_u8_to_hex(value.to_vec())
        );
        self.storage.insert(key, value.to_vec());
        Ok(())
    }
    pub fn jump(&mut self) -> Result<(), String> {
        // TODO that jump destination is valid
        self.pc = u256::u256_to_u64(self.pop()?) as usize;
        Ok(())
    }
    pub fn jump_i(&mut self) -> Result<(), String> {
        let new_pc = u256::u256_to_u64(self.pop()?) as usize;
        if !self.stack.is_empty() {
            let cond = u256::u256_to_u64(self.pop()?) as usize;
            if cond != 0 {
                self.pc = new_pc;
            }
        }
        // let cont = self.pop();
        // if cont {} // TODO depends on having impl Err in pop()
        Ok(())
    }
    pub fn jump_dest(&mut self) -> Result<(), String> {
        // TODO
        Ok(())
    }
}

fn upper_multiple_of_32(n: usize) -> usize {
    ((n - 1) | 31) + 1
}
