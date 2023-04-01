use num_bigint::BigInt;
use std::{ops::{Add, Sub, Mul}, cmp::Ordering};

// Totally cribbed from slyngbaek's answer here: https://exercism.org/tracks/rust/exercises/decimal/solutions/slyngbaek

#[derive(Debug, Eq, Clone)]
pub struct Decimal {
    whole: BigInt,
    decimal_place: usize,
}

impl Decimal {
    pub fn try_from(input: &str) -> Option<Decimal> {
        let mut num_string = input.to_string();
        let mut decimal_place = 0;
        if let Some(point) = input.find('.') {
            decimal_place = input.len() - point - 1;
            num_string.remove(point);
        }
        let whole = BigInt::parse_bytes(num_string.as_bytes(), 10).unwrap();
        Some(Decimal{whole, decimal_place})
    }

    fn discard_unnecessary_decimal(mut self) -> Self {
        let mut decimal_place = self.decimal_place;
        while decimal_place > 0 && self.whole.clone() % 10 == BigInt::from(0) {
            self.whole /= 10;
            decimal_place -= 1;
        }
        self.decimal_place = decimal_place;
        self
    }

    fn upshift(mut self, decimal_place:usize) -> Self {
        if self.decimal_place >= decimal_place {
            panic!("invalid upshift");
        }
        let shift_delta = decimal_place - self.decimal_place;
        self.whole *= BigInt::from(10).pow(shift_delta as u32);
        self.decimal_place = decimal_place;
        self
    }

    fn equalize_decimal_places(lhs: Self, rhs: Self) -> (Self, Self) {
        let left_decimal_place = lhs.decimal_place;
        let right_decimal_place = rhs.decimal_place;
        match lhs.decimal_place.cmp(&rhs.decimal_place) {
            Ordering::Equal => (lhs, rhs),
            Ordering::Less => (lhs.upshift(right_decimal_place), rhs),
            Ordering::Greater => (lhs, rhs.upshift(left_decimal_place)),
        }
    }
}

impl Add<Decimal> for Decimal {
    type Output = Decimal;

    fn add(self, rhs: Self) -> Decimal {
        let (mut lhs, rhs) = Decimal::equalize_decimal_places(self, rhs);
        lhs.whole += rhs.whole;
        lhs.discard_unnecessary_decimal()
    }
}

impl Sub<Decimal> for Decimal {
    type Output = Decimal;

    fn sub(self, rhs: Self) -> Decimal {
        let (mut lhs, rhs) = Decimal::equalize_decimal_places(self, rhs);
        lhs.whole -= rhs.whole;
        lhs.discard_unnecessary_decimal()
    }
}

impl Mul<Decimal> for Decimal {
    type Output = Decimal;

    fn mul(self, rhs: Self) -> Decimal {
        let mut lhs = self;
        lhs.whole *= rhs.whole;
        lhs.decimal_place += rhs.decimal_place;
        lhs.discard_unnecessary_decimal()
    }
}

impl PartialEq<Decimal> for Decimal {
    fn eq(&self, other: &Self) -> bool {
        let (lhs, rhs) = Decimal::equalize_decimal_places(self.clone(), other.clone());
        lhs.decimal_place == rhs.decimal_place && lhs.whole == rhs.whole
    }
}

impl PartialOrd<Decimal> for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let (lhs, rhs) = Decimal::equalize_decimal_places(self.clone(), other.clone());
        lhs.whole.partial_cmp(&rhs.whole)
    }
}

