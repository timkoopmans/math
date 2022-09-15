use crate::decimal::core::uint::U192;
use crate::decimal::{BigDecimal, Decimal};
use crate::decimal::errors::ErrorCode;

pub trait Sqrt<T>: Sized {
    fn sqrt(self) -> Result<Self, ErrorCode>;
}

/// Calculate the square root of a [Decimal] value.
impl Sqrt<Decimal> for Decimal {
    fn sqrt(self) -> Result<Self, ErrorCode> {
        let big_decimal: BigDecimal = self.into();

        // Note: we always use BigDecimal.sqrt() method for Decimal to
        // avoid arithmetic overflow of u128 internal value
        let big_sqrt = big_decimal.sqrt().expect("big_sqrt");

        Ok(big_sqrt.into())
    }
}

/// Calculate the square root of a [BigDecimal] value.
impl Sqrt<BigDecimal> for BigDecimal {
    fn sqrt(self) -> Result<BigDecimal, ErrorCode> {
        let zero = BigDecimal::new(U192::from(0u64), self.scale, false);
        let one = BigDecimal::new(
            U192::from(1u64)
                .checked_mul(self.denominator())
                .expect("one scaled"),
            self.scale,
            false,
        );

        if self.eq(&zero) || self.eq(&one) {
            return Ok(self);
        }

        let value = self.value;
        let denominator = self.denominator();

        // we double the precision by scaling out on itself
        let value_scaled = match value.checked_mul(denominator) {
            Some(x) => x,
            None => return Err(ErrorCode::ExceedsPrecisionRange),
        };

        let bit_length = 192u32
            .checked_sub(value_scaled.leading_zeros())
            .expect("bit_length");

        let mid_length = U192::from(bit_length.checked_div(2).expect("mid_length"));

        let approx = U192::from(2u128).checked_pow(mid_length).expect("approx");

        let mut y = value_scaled.checked_div(approx).expect("y");
        let mut y_0 = U192::from(0u128);
        let y_1 = U192::from(1u128);
        let threshold = U192::from(1u128);

        loop {
            if y.gt(&y_0) && (y.checked_sub(y_0).unwrap()).gt(&threshold)
                || y.lt(&y_0) && (y_0.checked_sub(y).unwrap()).gt(&threshold)
            {
                let tmp_y = value_scaled.checked_div(y).unwrap();
                y_0 = y;
                y = y.checked_add(tmp_y).unwrap();
                y >>= y_1;
            } else {
                break;
            }
        }

        Ok(BigDecimal::new(y, self.scale, self.negative))
    }
}

#[cfg(test)]
mod test {
    use crate::decimal::ops::Sqrt;
    use crate::decimal::Decimal;
    use proptest::prelude::*;

    #[test]
    fn test_sqrt() {
        // The square roots of the perfect squares (e.g., 0, 1, 4, 9, 16) are integers.
        // In all other cases, the square roots of positive integers are irrational numbers,
        // and hence have non-repeating decimals in their decimal representations.
        // Decimal approximations of the square roots of the first few natural numbers
        // are given in the following specs.

        // 0**0.5 = 0
        let n = Decimal::from_u64(0).to_compute_scale();
        let result = n.sqrt().unwrap();
        let expected = Decimal::from_u64(0).to_compute_scale();
        assert_eq!(result, expected);

        // 1**0.5 = 1
        let n = Decimal::from_u64(1).to_compute_scale();
        let result = n.sqrt().unwrap();
        let expected = Decimal::from_u64(1).to_compute_scale();
        assert_eq!(result, expected);

        // 2**0.5 = 1.414213562373
        let n = Decimal::from_u64(2).to_compute_scale();
        let result = n.sqrt().unwrap();
        let expected = Decimal::new(1_414_213_562_373u128, 12, false);
        assert_eq!(result, expected);

        // 3**0.5 = 1.7320508076
        let n = Decimal::from_u64(3).to_compute_scale();
        let result = n.sqrt().unwrap();
        let expected = Decimal::new(1_732_050_807_568u128, 12, false);
        assert_eq!(result, expected);

        // 4**0.5 = 2
        let n = Decimal::from_u64(4).to_compute_scale();
        let result = n.sqrt().unwrap();
        let expected = Decimal::from_u64(2).to_compute_scale();
        assert_eq!(result, expected);

        // MAX**0.5 = 4294967296
        let n = Decimal::from_u64(u64::MAX).to_scale(6);
        let result = n.sqrt().unwrap().to_scale_up(0).value;
        let expected = 4294967296u128;
        assert_eq!(result, expected);

        // 3.141592653589**0.5 = 1.7724538509
        let n = Decimal::new(3_141_592_653_589u128, 12, false);
        let result = n.sqrt().unwrap();
        let expected = Decimal::new(1_772_453_850_905u128, 12, false);
        assert_eq!(result, expected);

        // 3.141592**0.5 = 1.772453
        let n = Decimal::new(3_141_592u128, 6, false);
        let result = n.sqrt().unwrap();
        let expected = Decimal::new(1_772_453u128, 6, false);
        assert_eq!(result, expected);
    }

    proptest! {
        #[test]
        fn test_full_u64_range_sqrt(
            lhs in 1_000_000..u64::MAX, // 1.000000000 .. 18,446,744,073.709551615
        ) {
            let scale = 9; // decimal places
            let precision = 2; // accuracy +/- 0.000000001
            let lhs_decimal = Decimal::from_scaled_amount(lhs, scale);
            let lhs_f64: f64 = lhs_decimal.into();
            let den_f64: f64 = lhs_decimal.denominator() as f64;

            // f64 sqrt == Decimal sqrt
            {
                let sqrt_f64_u128 = (((lhs_f64.sqrt() * den_f64).round() / den_f64) * den_f64) as u128;
                let sqrt_decimal_u128 = lhs_decimal.to_scale(scale).sqrt().unwrap().value;
                let difference = sqrt_f64_u128.saturating_sub(sqrt_decimal_u128).lt(&precision);

                assert!(difference, "sqrt compare\n{}\n{}\n{}", sqrt_f64_u128, sqrt_decimal_u128, lhs_decimal);
            }
        }
    }
}
