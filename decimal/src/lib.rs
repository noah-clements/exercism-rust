use num_bigint::{BigInt, Sign};
use lazy_static::lazy_static;
use std::{ops::{Add, Sub, Mul}, cmp::{self, Ordering}};
use regex::Regex;

/// Type implementing arbitrary-precision decimal arithmetic
#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub struct Decimal {
    whole: BigInt,
    fraction: BigInt,
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
            let fraction = BigInt::from_biguint(sign, caps.get(3).unwrap().as_str().parse().unwrap());
            println!("fraction BigInt: {:?}", fraction);
            Some(Decimal{whole, fraction})
        } else {
            None
        }
    }
}

        // convert fractions to string, count the digits, and pad with 0s
        // then convert back to BigInt so we can do math
fn equalize_fraction_magnitudes(mut frac1: BigInt, mut frac2: BigInt) -> (BigInt, BigInt, usize) {
    let mut frac1_str = frac1.magnitude().to_string();
    let mut frac2_str = frac2.magnitude().to_string();
    let mut num_digits = frac1_str.len();

    if frac2_str.len() > num_digits {
        num_digits = frac2_str.len();
        frac1_str = format!("{:0<width$}", frac1_str, width=num_digits);
        frac1 = BigInt::from_biguint(frac1.sign(), frac1_str.parse().unwrap());
    } else if frac2_str.len() < num_digits {
        frac2_str = format!("{:0<width$}", frac2_str, width=num_digits);
        frac2 = BigInt::from_biguint(frac2.sign(), frac2_str.parse().unwrap());    
    }
    (frac1, frac2, num_digits)
}

fn compose_decimal(mut whole: BigInt, mut fraction: BigInt, num_digits: usize) -> Decimal {
    let mut frac_str = fraction.magnitude().to_string();
    if fraction.sign() != whole.sign() {
        match fraction.sign() {
            Sign::Minus => {
                whole -= 1;
                fraction += 10u32.pow(num_digits as u32);
            }
            _ => {
                whole += 1;
                fraction -= 10u32.pow(num_digits as u32);
            },
        }
    } else if frac_str.len() > num_digits {
        let mut carry = frac_str.drain(..(frac_str.len() - num_digits)).collect::<String>();
        if fraction.sign() == Sign::Minus {
            carry = format!("-{}", carry);
        }
        let carry_int: u32 = carry.parse().unwrap();
        whole += carry_int;
        fraction = frac_str.parse().unwrap();
    }
    Decimal{whole, fraction}
}

impl Add<Decimal> for Decimal {
    type Output = Decimal;

    fn add(self, other: Decimal) -> Decimal {
        let whole = self.whole + other.whole;

        let (my_frac, other_frac, num_digits) = equalize_fraction_magnitudes(self.fraction, other.fraction);
        let fraction = my_frac + other_frac;
        compose_decimal(whole, fraction, num_digits)
    }
}

impl Sub<Decimal> for Decimal {
    type Output = Decimal;

    fn sub(self, other: Decimal) -> Decimal {
        let whole = self.whole - other.whole;

        let (my_frac, other_frac, num_digits) = equalize_fraction_magnitudes(self.fraction, other.fraction);

        let fraction = my_frac - other_frac;
        compose_decimal(whole, fraction, num_digits)
    }
}

impl Mul<Decimal> for Decimal {
    type Output = Decimal;

    fn mul(self, other: Decimal) -> Decimal {
        let whole = self.whole * other.whole;

        let (my_frac, other_frac, num_digits) = equalize_fraction_magnitudes(self.fraction, other.fraction);

        let fraction = my_frac * other_frac;
        compose_decimal(whole, fraction, num_digits)
    }
}
