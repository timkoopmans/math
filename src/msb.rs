use crate::decimal::FixedPoint;
use checked_decimal_macro::*;

impl FixedPoint {
    pub fn msb(self) -> u32 {
        self.get().leading_zeros() - 1
    }
}
#[cfg(test)]
mod tests {
    use crate::decimal::FixedPoint;
    use checked_decimal_macro::*;

    #[test]
    fn test_msb() {
        let decimal = FixedPoint::new(u64::MAX.into());

        let actual = decimal.msb();
        let expected = 128u32 - 64u32 - 1u32;

        assert_eq!(actual, expected)
    }
}