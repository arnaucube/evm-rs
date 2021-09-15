use num_bigint::BigUint;

pub fn u256_to_u64(a: [u8; 32]) -> u64 {
    let mut b8: [u8; 8] = [0; 8];
    b8.copy_from_slice(&a[32 - 8..32]);
    u64::from_be_bytes(b8)
}
pub fn usize_to_u256(i: usize) -> [u8; 32] {
    let i_bytes = i.to_be_bytes();
    let mut r: [u8; 32] = [0; 32];
    r[32 - i_bytes.len()..].copy_from_slice(&i_bytes);
    r
}
pub fn str_to_u256(s: &str) -> [u8; 32] {
    let bi = s.parse::<BigUint>().unwrap().to_bytes_be();
    let mut r: [u8; 32] = [0; 32];
    r[32 - bi.len()..].copy_from_slice(&bi[..]);
    r
}
