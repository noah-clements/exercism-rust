pub fn production_rate_per_hour(speed: u8) -> f64 {
    assert!(speed <= 10);
    let base_rate = 221.0;
    match speed {
        0 => 0.0,
        1..= 4 => base_rate * speed as f64,
        5..= 8 => base_rate * speed as f64 * 0.9,
        9|10 => base_rate * speed as f64 * 0.77,
        _ => unreachable!(),
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    assert!(speed <= 10);
    (production_rate_per_hour(speed) / 60.0) as u32
}
