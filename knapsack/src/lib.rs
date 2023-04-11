use std::cmp::{max, Reverse};

pub struct Item {
    pub weight: u32,
    pub value: u32,
}

pub fn maximum_value(_max_weight: u32, mut _items: Vec<Item>) -> u32 {
    let mut max_value = 0;
    // let mut max_val_weight = 0;
    _items.sort_unstable_by_key(|item| Reverse(item.value));
    for (i, item) in _items.iter().enumerate() {
        let mut weight = 0;
        let mut value = 0;
        let mut tmp_value = 0;
        if item.weight <= _max_weight {
            value = item.value;
            weight = item.weight;
        }
        for j in i+1.._items.len() {
            if weight + _items[j].weight <= _max_weight {
                weight += _items[j].weight;
                value += _items[j].value;
            } else if weight - _items[j-1].weight + _items[j].weight <= _max_weight {
                tmp_value = max(value, tmp_value);
                weight += _items[j].weight;
                weight -= _items[j-1].weight;
                value += _items[j].value;
                value -= _items[j-1].value;
            }
        }
        max_value = max(max_value, value);
        max_value = max(max_value, tmp_value);
    }
    max_value
}
