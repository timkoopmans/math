pub mod decimal;
pub mod ln;
pub mod square;

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
}
