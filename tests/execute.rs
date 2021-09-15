use evm::*;

#[test]
fn stack_simple_push_pop() {
    let mut s = Stack::new();
    s.push(u256::str_to_u256("1"));
    s.push(u256::str_to_u256("2"));
    s.push(u256::str_to_u256("3"));
    assert_eq!(s.pop().unwrap(), u256::str_to_u256("3"));
    assert_eq!(s.pop().unwrap(), u256::str_to_u256("2"));
    assert_eq!(s.pop().unwrap(), u256::str_to_u256("1"));
    assert_eq!(s.pop(), Err(format!("pop err"))); // WIP
}

// arithmetic
#[test]
fn execute_opcodes_0() {
    let code = hex::decode("6005600c01").unwrap(); // 5+12
    let calldata = vec![];

    let mut s = Stack::new();
    s.execute(&code, &calldata, false).unwrap();
    assert_eq!(s.pop().unwrap(), u256::str_to_u256("17"));
    assert_eq!(s.gas, 9999999991);
    assert_eq!(s.pc, 5);
}

#[test]
fn execute_opcodes_1() {
    let code = hex::decode("60056004016000526001601ff3").unwrap();
    let calldata = vec![];

    let mut s = Stack::new();
    let out = s.execute(&code, &calldata, false).unwrap();

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
    s.execute(&code, &calldata, false).unwrap();

    // assert_eq!(out[0], 0x09);
    assert_eq!(s.gas, 9999999991);
    assert_eq!(s.pc, 7);
    assert_eq!(s.pop().unwrap(), u256::str_to_u256("515"));
}

#[test]
fn execute_opcodes_3() {
    // contains calldata
    let code = hex::decode("60003560203501").unwrap();
    let calldata = hex::decode("00000000000000000000000000000000000000000000000000000000000000050000000000000000000000000000000000000000000000000000000000000004").unwrap();

    let mut s = Stack::new();
    s.execute(&code, &calldata, false).unwrap();

    assert_eq!(s.gas, 9999999985);
    assert_eq!(s.pc, 7);
    assert_eq!(s.pop().unwrap(), u256::str_to_u256("9"));
}

// storage and execution
#[test]
fn execute_opcodes_4() {
    // contains loops
    let code = hex::decode("6000356000525b600160005103600052600051600657").unwrap();
    let calldata =
        hex::decode("0000000000000000000000000000000000000000000000000000000000000005").unwrap();

    let mut s = Stack::new();
    s.execute(&code, &calldata, false).unwrap();

    assert_eq!(s.gas, 9999999795);
    assert_eq!(s.pc, 22);
    assert_eq!(s.stack.len(), 0);
}
#[test]
fn execute_opcodes_5() {
    // contains loops, without using mem
    let code = hex::decode("6000355b6001900380600357").unwrap();
    let calldata =
        hex::decode("0000000000000000000000000000000000000000000000000000000000000001").unwrap();

    let mut s = Stack::new();
    s.execute(&code, &calldata, false).unwrap();

    assert_eq!(s.gas, 9999999968);
    assert_eq!(s.pc, 12);

    let code = hex::decode("6000355b6001900380600357").unwrap();
    let calldata =
        hex::decode("0000000000000000000000000000000000000000000000000000000000000002").unwrap();

    let mut s = Stack::new();
    s.execute(&code, &calldata, false).unwrap();

    assert_eq!(s.gas, 9999999942);
    assert_eq!(s.pc, 12);

    let code = hex::decode("6000355b6001900380600357").unwrap();
    let calldata =
        hex::decode("0000000000000000000000000000000000000000000000000000000000000005").unwrap();

    let mut s = Stack::new();
    s.execute(&code, &calldata, false).unwrap();

    assert_eq!(s.gas, 9999999864);
    assert_eq!(s.pc, 12);
}
#[test]
fn execute_opcodes_6() {
    // 0x36: calldata_size
    let code = hex::decode("366020036101000a600035045b6001900380600c57").unwrap();
    let calldata = hex::decode("01").unwrap();

    let mut s = Stack::new();
    s.execute(&code, &calldata, false).unwrap();

    assert_eq!(s.gas, 9999999892);
    assert_eq!(s.pc, 21);
    assert_eq!(s.stack.len(), 1);

    let code = hex::decode("366020036101000a600035045b6001900380600c57").unwrap();
    let calldata = hex::decode("05").unwrap();

    let mut s = Stack::new();
    s.execute(&code, &calldata, false).unwrap();

    assert_eq!(s.gas, 9999999788);
    assert_eq!(s.pc, 21);
    assert_eq!(s.stack.len(), 1);

    let code = hex::decode("366020036101000a600035045b6001900380600c57").unwrap();
    let calldata = hex::decode("0101").unwrap();

    let mut s = Stack::new();
    s.execute(&code, &calldata, false).unwrap();

    assert_eq!(s.gas, 9999993236);
    assert_eq!(s.pc, 21);
    assert_eq!(s.stack.len(), 1);
}

#[test]
fn execute_opcodes_7() {
    // contract deployment (code_copy)
    let code = hex::decode("600580600b6000396000f36005600401").unwrap();
    let calldata = hex::decode("").unwrap();

    let mut s = Stack::new();
    let out = s.execute(&code, &calldata, true).unwrap();

    assert_eq!(s.gas, 9999999976);
    assert_eq!(s.pc, 10);
    assert_eq!(s.stack.len(), 0);
    assert_eq!(s.mem.len(), 32);
    assert_eq!(
        s.mem,
        hex::decode("6005600401000000000000000000000000000000000000000000000000000000").unwrap()
    );
    assert_eq!(out, hex::decode("6005600401").unwrap());
}

#[test]
fn execute_opcodes_8() {
    let code = hex::decode("611000805151").unwrap();
    let calldata = hex::decode("").unwrap();

    let mut s = Stack::new();
    s.execute(&code, &calldata, false).unwrap();

    assert_eq!(s.gas, 9999999569);
    assert_eq!(s.pc, 6);
    assert_eq!(s.stack.len(), 2);
    assert_eq!(s.mem.len(), 4128);
}

#[test]
fn execute_opcodes_9() {
    // sstore (0x55)
    let code = hex::decode("60026000556001600055").unwrap();
    let calldata = hex::decode("").unwrap();

    let mut s = Stack::new();
    s.execute(&code, &calldata, true).unwrap();

    assert_eq!(s.gas, 9999974988);
    // assert_eq!(s.gas, 9999977788); // TODO WIP geth reported gas
    assert_eq!(s.pc, 10);
    assert_eq!(s.stack.len(), 0);
    assert_eq!(s.storage.len(), 1);
}
