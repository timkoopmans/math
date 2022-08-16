use crate::decimal::core::uint::U256;
use crate::decimal::core::Compare;
use crate::decimal::Decimal;
use crate::decimal::errors::DecimalError;

pub trait Sqrt<T>: Sized {
    fn sqrt(self) -> Result<Self, DecimalError>;
}

/// Calculate the square root of a [Decimal] value. For full algorithm please refer to:
/// https://docs.google.com/spreadsheets/d/1dw7HaR_YsgvT7iA_4kv2rgWb-EvSyQGM/edit#gid=432909162
impl Sqrt<Decimal> for Decimal {
    fn sqrt(self) -> Result<Self, DecimalError> {
        let zero = Decimal::new(0, self.scale, false);
        let one = Decimal::from_u128(1).to_scale(self.scale);

        if self.eq(zero).unwrap() || self.eq(one).unwrap() {
            return Ok(self);
        }

        let value = U256::try_from(self.value)
            .unwrap_or_else(|_| panic!("decimal: rhs value does not fit in Decimal::sqrt()"));
        let denominator = U256::try_from(self.denominator()).unwrap_or_else(|_| {
            panic!("decimal: denominator value does not fit in Decimal::sqrt()")
        });
        let value_scaled = match value.checked_mul(denominator) {
            Some(x) => x,
            None => return Err(DecimalError::ExceedsPrecisionRange),
        };
        let value_scaled_bit_length = 256u32
            .checked_sub(value_scaled.leading_zeros())
            .expect("bit_length");
        let value_scaled_mid_length = U256::try_from(
            value_scaled_bit_length
                .checked_div(2)
                .expect("value_scaled_mid_length"),
        )
        .unwrap();
        let value_approx = U256::try_from(2u128)
            .unwrap()
            .checked_pow(value_scaled_mid_length)
            .expect("value_approx");
        let mut y = value_scaled.checked_div(value_approx).expect("y");
        let mut y_0 = U256::try_from(0u128).unwrap();
        let y_1 = U256::try_from(1u128).unwrap();
        let threshold = U256::try_from(1u128).unwrap();

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

        Ok(Self {
            value: y.try_into().unwrap_or_else(|_| {
                panic!("decimal: overflow in method Decimal::sqrt() casting to u128")
            }),
            scale: self.scale,
            negative: self.negative,
        })
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
            lhs in 1_000_000..u64::MAX, // 1.000000 .. 18,446,744,073,709.551615
        ) {
            let scale = 6; // decimal places
            let precision = 2; // accuracy +/- 0.000001
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
