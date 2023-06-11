use std::cmp::max;

pub struct Item {
    pub weight: usize,
    pub value: usize,
}

pub fn maximum_value(max_weight: u32, items: &[Item]) -> u32 {
    let mut table = vec![0; max_weight as usize + 1];
    for item in items {
        for w in (item.weight..=max_weight as usize).rev() {
            table[w] = max(table[w], item.value + table[w - item.weight]);
        }
    }
    table[max_weight as usize] as u32
}