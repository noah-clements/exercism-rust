pub fn square(s: u32) -> u64 {
    if !(1..=64).contains(&s) {
        panic!("Square must be between 1 and 64");
    }
    1 << (s - 1)
}

pub fn total() -> u64 {
    // The equation is 2^(n+1) - 1 counting from zero
    // 2^(n+1) is the same as 1 << (n+1)
    // 1 << 64 would overflow by one, one less is max value of u64
    // (((1u128) << 64) - 1) as u64
    u64::MAX
}
