#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base < 2 {
        return Err(Error::InvalidInputBase);
    }
    if to_base < 2 {
        return Err(Error::InvalidOutputBase);
    }
    let mut sum = number.iter().try_fold(0, |sum, digit|{
        if *digit >= from_base {
            Err(Error::InvalidDigit(*digit))
        } else {
            Ok(sum * from_base + digit)
        }
    })?;
// The above try_fold is equivalent to the following loop code:
// It is "more Rusty" (and more efficient) to use try_fold, 
// but it is also more difficult to understand.
    // for digit in number {
    //     if *digit >= from_base {
    //         return Err(Error::InvalidDigit(*digit));
    //     }
    //     sum = sum * from_base + digit;
    // }

    let mut result: Vec<u32> = Vec::new();
    if sum == 0 {
        result.push(0);
    } else {
        while sum > 0 {
            result.push(sum % to_base);
            sum /= to_base;
        }
    }
    result.reverse();
    Ok(result)
}
