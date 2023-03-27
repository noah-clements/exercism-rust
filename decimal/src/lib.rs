use num_bigint::{BigInt, Sign};
use lazy_static::lazy_static;
use std::{ops::{Add, Sub, Mul}, cmp::{self, Ordering, min}};
use regex::Regex;

/// Type implementing arbitrary-precision decimal arithmetic
#[derive(Debug, PartialEq, Eq, Clone, PartialOrd)]
pub struct Decimal {
    sign: Sign,
    whole: BigInt,
    fraction: String,
}


impl Decimal {
    pub fn try_from(input: &str) -> Option<Decimal> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(-?)(\d+)(\.(\d+))?$").unwrap();
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
            let fraction =String::from(caps.get(4).map_or("0", |m| m.as_str()));
            println!("fraction string: {:?}", fraction);
            Some(Decimal{sign, whole, fraction})
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
    (carry, frac1)
}
fn sub_fractions(mut frac1: String, frac2: String, num_digits: usize) -> (i32, String) {
    let mut borrow = 0;
    for i in (0..num_digits).rev() {
        let digit1 = frac1.get(i..i+1).unwrap().parse::<i32>().unwrap();
        let digit2 = frac2.get(i..i+1).unwrap().parse::<i32>().unwrap();
        let sum: i32 = digit1 - digit2 - borrow;
        if sum < 0 {
            borrow = 1;
            let digit = sum + 10;
            frac1.replace_range(i..i+1, &digit.to_string());
        } else {
            borrow = 0;
            frac1.replace_range(i..i+1, &sum.to_string());
        }
    }
    (borrow, frac1)
}

fn mult_fractions(mut frac1: String, frac2: String, num_digits: usize) -> (u32, String) {
    let mut carry = 0;
    // let mut product
    for i in (0..num_digits).rev() {
        let digit1 = frac1.get(i..i+1).unwrap().parse::<u32>().unwrap();
        for j in (0..num_digits).rev() {
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
        if self.sign == other.sign {
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
        fraction = fraction.trim_end_matches('0').to_string();
        if fraction.len() == 0 {
            fraction = "0".to_string();
        }
    
        Decimal {sign, whole, fraction}
    }
}

impl Sub<Decimal> for Decimal {
    type Output = Decimal;

    fn sub(self, other: Decimal) -> Decimal {
        let mut whole = self.whole.clone() - other.whole.clone();

        let (my_frac, other_frac, num_digits) = equalize_fraction_magnitudes(self.fraction, other.fraction);
        let mut fraction = String::new();
        let mut carry = 0;
        let mut borrow = 0;
        if self.sign == other.sign {
            if self.sign == Sign::Minus && whole.sign() == Sign::Plus {
                (borrow, fraction) = sub_fractions(other_frac, my_frac, num_digits);
                if borrow > 0 {
                    whole -= borrow;
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
        } else {
            (carry, fraction) = add_fractions(my_frac, other_frac, num_digits);
            if carry > 0 {
                if whole.sign() == Sign::Minus {
                    whole -= carry;
                } else {
                    whole += carry;
                }
            }
        }
        let mut sign = whole.sign();
        if let Sign::NoSign = sign {
            sign = Sign::Plus;
        }
        fraction = fraction.trim_end_matches('0').to_string();
        if fraction.len() == 0 {
            fraction = "0".to_string();
        }
        Decimal {sign, whole, fraction}
    }
}

impl Mul<Decimal> for Decimal {
    type Output = Decimal;

    fn mul(self, other: Decimal) -> Decimal {
        // multiply lhs whole by rhs whole, then multiply lhs fraction by rhs fraction, then lhs whole by rhs fraction, then lhs fraction by rhs whole

        let mut lhs = self.whole;
        let mut rhs = other.whole;
        let mut whole = lhs.clone() * rhs.clone();

        // Cross multiply fractions to whole numbers and determine carry to add to whole
        let mut lhs_frac_len = self.fraction.len();
        let mut rhs_frac_len = other.fraction.len();
        let mut lh_frac = self.fraction.parse::<u64>().unwrap();
        let mut rh_frac = other.fraction.parse::<u64>().unwrap();
        lhs *= rh_frac / 10u64.pow(rhs_frac_len as u32);
        rhs *= lh_frac / 10u64.pow(lhs_frac_len as u32);
        let mut lhs_str = lhs.magnitude().to_string();
        let mut lh_frac_carry: u32 = 0;
        if lhs_str.len() > rhs_frac_len {
            lh_frac_carry = lhs_str.drain(..(lhs_str.len() - rhs_frac_len))
            .collect::<String>().parse().unwrap();
        } 
        let mut rhs_str = rhs.magnitude().to_string();
        let mut rh_frac_carry: u32 = 0;
        if rhs_str.len() > lhs_frac_len {
            rh_frac_carry = rhs_str.drain(..(rhs_str.len() - lhs_frac_len))
            .collect::<String>().parse().unwrap();
        } 

        // let (mut my_frac, mut other_frac, num_digits) = equalize_fraction_magnitudes(self.fraction, other.fraction);
        let mut frac_places: u32 = 0;
        let mut num_digits = lhs_frac_len;
        if lhs_frac_len > rhs_frac_len {
            frac_places = (lhs_frac_len - rhs_frac_len).try_into().unwrap();
            rh_frac *= 10u64.pow(frac_places);
        } else if rhs_frac_len > lhs_frac_len {
            num_digits = rhs_frac_len;
            frac_places = (rhs_frac_len - lhs_frac_len).try_into().unwrap();
            lh_frac *= 10u64.pow(frac_places);
        }

        let frac_prod: u64 = lh_frac * rh_frac / 10u64.pow(frac_places.try_into().unwrap());
        let mut frac_prod_str = frac_prod.to_string();
        let mut frac_prod_carry: u32 = 0;
        if frac_prod_str.len() > num_digits {
            frac_prod_carry = frac_prod_str.drain(..(frac_prod_str.len() - num_digits))
            .collect::<String>().parse().unwrap();
        } else {
            frac_prod_str = format!("{:0>width$}", frac_prod_str, width=num_digits);
        }
        frac_prod_carry += lh_frac_carry + rh_frac_carry;
        let mut temp_carry = 0;
        (temp_carry, frac_prod_str) = add_fractions(frac_prod_str, lhs_str, num_digits);
        frac_prod_carry += temp_carry;
        (temp_carry, frac_prod_str) = add_fractions(frac_prod_str, rhs_str, num_digits);
        frac_prod_carry += temp_carry;

        whole += lhs + rhs;
        if whole.sign() == Sign::Minus {
            whole -= frac_prod_carry;
        } else {
            whole += frac_prod_carry;
        }
        
        let mut sign = whole.sign();
        if let Sign::NoSign = sign {
            sign = Sign::Plus;
        }
        frac_prod_str = frac_prod_str.trim_end_matches('0').to_string();
        if frac_prod_str.len() == 0 {
            frac_prod_str = "0".to_string();
        }
    
        Decimal {sign, whole, fraction: frac_prod_str}
    }
}
