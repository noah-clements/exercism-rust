use std::cmp::max;

pub struct Item {
    pub weight: usize,
    pub value: usize,
}

pub fn maximum_value(_max_weight: u32, mut _items: Vec<Item>) -> u32 {
    let n = _items.len();
    let mut table = vec![vec![0; _max_weight as usize + 1]; n+1];
    for i in 1..=n {
        for w in 1..=_max_weight as usize {
            if _items[i-1].weight > w {
                table[i][w] = table[i-1][w];
            } else {
                table[i][w] = max(table[i-1][w], _items[i-1].value + table[i-1][w-_items[i-1].weight]);
            }
        }
    }
    table[n][_max_weight as usize] as u32
}
