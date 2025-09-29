static A: f64 = 0.6180339887;

pub fn hash_by_division(key: u32, table_size: u32) -> u32 {
    (key % table_size) as u32
}

pub fn hash_by_multiplication(key: u32, table_size: u32) -> u32 {
    let fract = (key as f64 * A) % 1.0;
    (table_size as f64 * fract).floor() as u32
}

pub fn hash_by_string(key: &str, table_size: u32) -> u32 {
    let mut hash: u32 = 0;
    for b in key.bytes() {
        hash = hash.wrapping_mul(31).wrapping_add(b as u32);
    }
    (hash as u32) % table_size
}
