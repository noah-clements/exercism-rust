/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut digits: Vec<u32> = Vec::new();
    for (i, c) in code.chars().rev()
    .filter(|c| !c.is_whitespace())
    .enumerate() {
        if c.is_digit(10) {
            if i % 2 == 1 {
                let d = c.to_digit(10).unwrap() * 2;
                if d > 9 {
                    digits.push(d - 9);
                } else {
                    digits.push(d);
                }
            } else {
                digits.push(c.to_digit(10).unwrap());
            }
        } else if c != ' ' {
            return false;
        }
    }
    let sum: u32 = digits.iter().sum();
    if digits.len() > 1 && sum % 10 == 0 {
        true
    } else {
        false
    }
}
