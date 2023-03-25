use num_bigint::{BigInt, Sign};
use lazy_static::lazy_static;
use std::{ops::{Add, Sub, Mul}, cmp::{self, Ordering}};
use regex::Regex;

/// Type implementing arbitrary-precision decimal arithmetic
#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub struct Decimal {
    whole: BigInt,
    fraction: String,
    sign: Sign,
    // implement your type here
}


impl Decimal {
    pub fn try_from(input: &str) -> Option<Decimal> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(-?)(\d+)\.(\d+)$").unwrap();
        }
        // let re = Regex::new(r"^(-?)(\d+)\.(\d+)$").unwrap();
        if let Some(caps) = RE.captures(input) {
            let sign = if caps.get(1).unwrap().as_str() == "-" {
                Sign::Minus
            } else {
                Sign::Plus
            };
            let whole = BigInt::from_biguint(sign, caps.get(2).unwrap().as_str().parse().unwrap());
            println!("whole BigInt: {:?}", whole);
            // let whole = BigInt::from
            let fraction =String::from(caps.get(3).unwrap().as_str());
            println!("fraction BigInt: {:?}", fraction);
            Some(Decimal{whole, fraction, sign})
        } else {
            None
        }
    }
}

        // count the digits, and pad with 0s if necessary
fn equalize_fraction_magnitudes(mut frac1: String, mut frac2: String) -> (String, String, usize) {
    // let mut frac1_str = frac1.magnitude().to_string();
    // let mut frac2_str = frac2.magnitude().to_string();
    let mut num_digits = frac1.len();

    if frac2.len() > num_digits {
        num_digits = frac2.len();
        frac1 = format!("{:0<width$}", frac1, width=num_digits);
    } else if frac2.len() < num_digits {
        frac2 = format!("{:0<width$}", frac2, width=num_digits);
    }
    (frac1, frac2, num_digits)
}

fn add_fractions(mut frac1: String, frac2: String, num_digits: usize) -> (u32, String) {
    let mut carry = 0;
    for i in (0..num_digits).rev() {
        let digit1 = frac1.get(i..i+1).unwrap().parse::<u32>().unwrap();
        let digit2 = frac2.get(i..i+1).unwrap().parse::<u32>().unwrap();
        let sum = digit1 + digit2 + carry;
        if sum > 9 {
            carry = sum / 10;
            let digit = sum % 10;
            frac1.replace_range(i..i+1, &digit.to_string());
        } else {
            carry = 0;
            frac1.replace_range(i..i+1, &sum.to_string());
        }
    }
    if num_digits > 1 {
        frac1 = frac1.trim_end_matches('0').to_string();
    }
    (carry, frac1)
}
fn sub_fractions(mut frac1: String, frac2: String, num_digits: usize) -> (i32, String) {
    let mut borrow = 0;
    for i in (0..num_digits).rev() {
        let digit1 = frac1.get(i..i+1).unwrap().parse::<i32>().unwrap();
        let digit2 = frac2.get(i..i+1).unwrap().parse::<i32>().unwrap();
        let sum: i32 = (digit1 - digit2 - borrow);
        if sum < 0 {
            borrow = 1;
            let digit = sum + 10;
            frac1.replace_range(i..i+1, &digit.to_string());
        } else {
            borrow = 0;
            frac1.replace_range(i..i+1, &sum.to_string());
        }
    }
    if num_digits > 1 {
        frac1 = frac1.trim_end_matches('0').to_string();
    }
    (borrow, frac1)
}

fn mult_fractions(mut frac1: String, frac2: String, num_digits: usize) -> (u32, String) {
    let mut carry = 0;
    for i in (0..num_digits).rev() {
        let digit1 = frac1.get(i..i+1).unwrap().parse::<u32>().unwrap();
        let digit2 = frac2.get(i..i+1).unwrap().parse::<u32>().unwrap();
        let prod = digit1 * digit2 + carry;
        if prod > 9 {
            carry = prod / 10;
            let digit = prod % 10;
            frac1.replace_range(i..i+1, &digit.to_string());
        } else {
            carry = 0;
            frac1.replace_range(i..i+1, &prod.to_string());
        }
    }
    if num_digits > 1 {
        frac1 = frac1.trim_end_matches('0').to_string();
    }
    (carry, frac1)
}

impl Add<Decimal> for Decimal {
    type Output = Decimal;

    fn add(self, other: Decimal) -> Decimal {
        let mut whole = self.whole.clone() + other.whole.clone();

        let (mut my_frac, other_frac, num_digits) = equalize_fraction_magnitudes(self.fraction, other.fraction);
        let mut fraction = String::new();
        let mut carry = 0;
        let mut borrow = 0;
        if self.sign == other.sign && self.sign == whole.sign() {
            (carry, fraction) = add_fractions(my_frac, other_frac, num_digits);
            if carry > 0 {
                if whole.sign() == Sign::Minus {
                    whole -= carry;
                } else {
                    whole += carry;
                }
            }
        } else {
            (borrow, fraction) = sub_fractions(my_frac, other_frac, num_digits);
            if borrow > 0 {
                if whole.sign() == Sign::Minus {
                    whole += borrow;
                } else {
                    whole -= borrow;
                }
            }
        }
        let mut sign = whole.sign();
        if let Sign::NoSign = sign {
            sign = Sign::Plus;
        }
        Decimal {whole, fraction, sign}
    }
}

impl Sub<Decimal> for Decimal {
    type Output = Decimal;

    fn sub(self, other: Decimal) -> Decimal {
        let mut whole = self.whole.clone() - other.whole.clone();

        let (my_frac, other_frac, num_digits) = equalize_fraction_magnitudes(self.fraction, other.fraction);
        let mut fraction = String::new();
        let mut borrow: i32 = 0;
        if self.sign == other.sign {
            (borrow, fraction) = sub_fractions(my_frac, other_frac, num_digits);
            if borrow > 0 {
                if whole.sign() == Sign::Minus {
                    whole += borrow;
                } else {
                    whole -= borrow;
                }
            }
        } else {
            // converse of add, ignore the carry
            let (_, fraction) = add_fractions(my_frac, other_frac, num_digits);
        }
        let mut sign = whole.sign();
        if let Sign::NoSign = sign {
            sign = Sign::Plus;
        }
        Decimal {whole, fraction, sign}
    }
}

impl Mul<Decimal> for Decimal {
    type Output = Decimal;

    fn mul(self, other: Decimal) -> Decimal {
        let mut whole = self.whole.clone() * other.whole.clone();

        let (my_frac, other_frac, num_digits) = equalize_fraction_magnitudes(self.fraction, other.fraction);
        let (carry, fraction) = mult_fractions(my_frac, other_frac, num_digits);
        if carry > 0 {
            if whole.sign() == Sign::Minus {
                whole -= carry;
            } else {
                whole += carry;
            }
        }
        let mut sign = whole.sign();
        if let Sign::NoSign = sign {
            sign = Sign::Plus;
        }
        Decimal {whole, fraction, sign}
    }
}
