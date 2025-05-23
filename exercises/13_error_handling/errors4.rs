use std::cmp::Ordering;

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<Self, CreationError> {
        // TODO: 此函数不应该总是返回 `Ok`.
        // if (..0).contains(&value) {
        //     Err(CreationError::Negative)
        // } else if value == 0 {
        //     return Err(CreationError::Zero);
        // } else {
        //     Ok(Self(value as u64))
        // }
        match value.cmp(&0) {
            Ordering::Less => Err(CreationError::Negative),
            Ordering::Equal => Err(CreationError::Zero),
            Ordering::Greater => Ok(Self(value as u64)),
        }
    }
}

fn main() {
    // (可选)你可以选择性地在此处进行试验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        assert_eq!(
            PositiveNonzeroInteger::new(10),
            Ok(PositiveNonzeroInteger(10)),
        );
        assert_eq!(
            PositiveNonzeroInteger::new(-10),
            Err(CreationError::Negative),
        );
        assert_eq!(PositiveNonzeroInteger::new(0), Err(CreationError::Zero));
    }
}
