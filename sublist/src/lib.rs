#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    let first_len = _first_list.len();
    let second_len = _second_list.len();
    match (first_len, second_len) {
        (0, 0) => Comparison::Equal,
        (0, _) => Comparison::Sublist,
        (_, 0) => Comparison::Superlist,
        (first_len, second_len) if first_len < second_len => {
            if _second_list.windows(first_len).any(|w| w == _first_list) {
                Comparison::Sublist
            } else {
                Comparison::Unequal
            }
        },
        (first_len, second_len) if first_len > second_len => {
            if _first_list.windows(second_len).any(|w| w == _second_list) {
                Comparison::Superlist
            } else {
                Comparison::Unequal
            }
        },
        (_,_) => {            
            if _first_list == _second_list {
                Comparison::Equal
            } else {
                Comparison::Unequal
            }
        },
    }
}
