use evm::*;

#[test]
fn stack_simple_push_pop() {
    let mut s = Stack::new();
    s.push(str_to_u256("1"));
    s.push(str_to_u256("2"));
    s.push(str_to_u256("3"));
    assert_eq!(s.pop(), str_to_u256("3"));
    assert_eq!(s.pop(), str_to_u256("2"));
    assert_eq!(s.pop(), str_to_u256("1"));
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

#[test]
fn execute_opcodes_3() {
    // contains calldata
    let code = hex::decode("60003560203501").unwrap();
    let calldata = hex::decode("00000000000000000000000000000000000000000000000000000000000000050000000000000000000000000000000000000000000000000000000000000004").unwrap();

    let mut s = Stack::new();
    s.execute(&code, &calldata, false);

    assert_eq!(s.gas, 9999999985);
    assert_eq!(s.pc, 7);
    assert_eq!(s.pop(), str_to_u256("9"));
}

// storage and execution
#[test]
fn execute_opcodes_4() {
    // contains loops
    let code = hex::decode("6000356000525b600160005103600052600051600657").unwrap();
    let calldata =
        hex::decode("0000000000000000000000000000000000000000000000000000000000000005").unwrap();

    let mut s = Stack::new();
    s.execute(&code, &calldata, false);

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
    s.execute(&code, &calldata, false);

    assert_eq!(s.gas, 9999999968);
    assert_eq!(s.pc, 12);

    let code = hex::decode("6000355b6001900380600357").unwrap();
    let calldata =
        hex::decode("0000000000000000000000000000000000000000000000000000000000000002").unwrap();

    let mut s = Stack::new();
    s.execute(&code, &calldata, false);

    assert_eq!(s.gas, 9999999942);
    assert_eq!(s.pc, 12);

    let code = hex::decode("6000355b6001900380600357").unwrap();
    let calldata =
        hex::decode("0000000000000000000000000000000000000000000000000000000000000005").unwrap();

    let mut s = Stack::new();
    s.execute(&code, &calldata, false);

    assert_eq!(s.gas, 9999999864);
    assert_eq!(s.pc, 12);
}
// #[test]
// fn execute_opcodes_6() {
//     // 0x36: calldata_size
//     let code = hex::decode("366020036101000a600035045b6001900380600c57").unwrap();
//     let calldata = hex::decode("05").unwrap();
//
//     let mut s = Stack::new();
//     s.execute(&code, &calldata, false);
//
//     assert_eq!(s.gas, 9999999788);
//     assert_eq!(s.pc, 21);
//     assert_eq!(s.stack.len(), 0);
// }
