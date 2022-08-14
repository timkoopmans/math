use crate::decimal::FixedPoint;
use checked_decimal_macro::BigOps;

impl FixedPoint {
    pub fn square(self) -> Self {
        self.big_mul(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::decimal::FixedPoint;
    use checked_decimal_macro::*;

    #[test]
    fn test_square() {
        let decimal = FixedPoint::from_scale(15, 1); // 1.5

        let actual = decimal.square();
        let expected = FixedPoint::new(2250000000000); // 2.25
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_big_square() {
        let decimal = FixedPoint::new(u64::MAX.into());
        let actual = decimal.square();
        let expected = FixedPoint::new(340282366920938463426481119);
        assert_eq!(actual, expected);
    }
}