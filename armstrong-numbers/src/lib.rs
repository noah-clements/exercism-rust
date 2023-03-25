pub fn is_armstrong_number(num: u32) -> bool {
    let len = num.to_string().len() as u32;
    let sum = num.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .map(|d| d.pow(len))
        .map(|e| e as u64)
        .sum::<u64>();
    sum == num as u64
}
