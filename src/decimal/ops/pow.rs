use crate::decimal::core::Compare;
use crate::decimal::ops::sqrt::Sqrt;
use crate::decimal::ops::{BigMul, Div, Mul, Neg};
use crate::decimal::{BigDecimal, Decimal};

pub trait Pow<T>: Sized {
    fn pow(self, rhs: T) -> Self;
}

/// Calculate the power of a [Decimal] with another [Decimal] as the exponent.
impl Pow<Decimal> for Decimal {
    fn pow(self, exp: Decimal) -> Self {
        let positive = exp.is_positive();

        let base = self.to_compute_scale();
        let exp = exp.to_compute_scale();
        let exp = Some(exp);

        let result = match exp {
            // e.g. x^0 = 1
            Some(x) if x.is_zero() => Decimal::one(),

            // e.g. x^0.25 = ⁴√x = √(√x) = sqrt(sqrt(x))
            Some(x) if positive && x.eq(Decimal::zero_point_two_five()).unwrap() => base
                .sqrt()
                .expect("sqrt")
                .to_compute_scale()
                .sqrt()
                .expect("sqrt")
                .to_compute_scale(),

            // e.g. x^0.5 = √x = sqrt(x)
            Some(x) if positive && x.eq(Decimal::zero_point_five()).unwrap() => {
                base.sqrt().expect("sqrt").to_compute_scale()
            }

            // e.g. x^1 = x
            Some(x) if positive && x.eq(Decimal::one()).unwrap() => base,

            // e.g. x^1.25 = x(√(√x)) = x(sqrt(sqrt(x)))
            Some(x) if positive && x.eq(Decimal::one_point_two_five()).unwrap() => base.mul(
                base.sqrt()
                    .expect("sqrt")
                    .to_compute_scale()
                    .sqrt()
                    .expect("sqrt")
                    .to_compute_scale(),
            ),

            // e.g. x^1.50 = x(√x) = x(sqrt(x))
            Some(x) if positive && x.eq(Decimal::one_point_five()).unwrap() => {
                base.mul(base.sqrt().expect("sqrt").to_compute_scale())
            }

            // e.g. x^2
            Some(x) if positive && x.eq(Decimal::two()).unwrap() => base.mul(self),

            // e.g. x^N
            Some(x) if positive && x.is_integer() => base.pow(x.abs() as u128),

            // e.g. x^-0.25 = 1/x^0.25
            Some(x) if !positive && x.eq(Decimal::zero_point_two_five().neg()).unwrap() => {
                Decimal::one().div(base.pow(Decimal::zero_point_two_five()).to_compute_scale())
            }

            // e.g. x^-0.5 = 1/x^0.5
            Some(x) if !positive && x.eq(Decimal::zero_point_five().neg()).unwrap() => {
                Decimal::one().div(base.sqrt().expect("sqrt").to_compute_scale())
            }

            // e.g. x^-1 == 1/x
            Some(x) if !positive && x.eq(Decimal::one().neg()).unwrap() => Decimal::one().div(base),

            // e.g. x^-1.25 == 1/x^1.25
            Some(x) if !positive && x.eq(Decimal::one_point_two_five().neg()).unwrap() => {
                Decimal::one().div(base.pow(Decimal::one_point_two_five()).to_compute_scale())
            }

            // e.g. x^-1.5 == 1/x^1.5
            Some(x) if !positive && x.eq(Decimal::one_point_five().neg()).unwrap() => {
                Decimal::one().div(base.pow(Decimal::one_point_five()).to_compute_scale())
            }

            // e.g. x^-2 == 1/x^2
            Some(x) if !positive && x.eq(Decimal::two()).unwrap() => {
                Decimal::one().div(base.pow(Decimal::two()).to_compute_scale())
            }

            // e.g. x^-N == 1/x^N
            Some(x) if !positive && x.is_integer() => {
                Decimal::one().div(base.pow(x.abs() as u128).to_compute_scale())
            }

            _ => panic!("pow not implemented for exponent: {}", exp.unwrap()),
        };

        result.to_scale(self.scale)
    }
}

/// Calculate the power of a [Decimal] with an unsigned integer as the exponent.
impl Pow<u128> for Decimal {
    fn pow(self, exp: u128) -> Self {
        let one = Decimal::one().to_scale(self.scale);

        if exp == 0 {
            return one;
        }

        let mut current_exp = exp;
        let mut base = self;
        let mut result = one;

        while current_exp > 0 {
            if current_exp % 2 != 0 {
                result = result.big_mul(base);
            }
            current_exp /= 2;
            base = base.big_mul(base);
        }
        result
    }
}

/// Calculate the power of a [Decimal] with another [Decimal] as the exponent.
impl Pow<BigDecimal> for BigDecimal {
    fn pow(self, exp: BigDecimal) -> Self {
        let positive = !exp.negative;

        let base = self.to_compute_scale();
        let exp = exp.to_compute_scale();

        let exp = Some(exp);

        match exp {
            // e.g. x^0 = 1
            Some(x) if x.value.is_zero() => BigDecimal::one(),

            // e.g. x^0.25 = ⁴√x = √(√x) = sqrt(sqrt(x))
            Some(x) if positive && x.eq(&BigDecimal::zero_point_two_five()) => {
                base.sqrt().expect("sqrt").sqrt().expect("sqrt")
            }

            // e.g. x^0.5 = √x = sqrt(x)
            Some(x) if positive && x.eq(&BigDecimal::zero_point_five()) => {
                base.sqrt().expect("sqrt")
            }

            // e.g. x^1 = x
            Some(x) if positive && x.eq(&BigDecimal::one()) => base,

            // e.g. x^1.25 = x(√(√x)) = x(sqrt(sqrt(x)))
            Some(x) if positive && x.eq(&BigDecimal::one_point_two_five()) => {
                base.mul(base.sqrt().expect("sqrt").sqrt().expect("sqrt"))
            }

            // e.g. x^1.50 = x(√x) = x(sqrt(x))
            Some(x) if positive && x.eq(&BigDecimal::one_point_five()) => {
                base.mul(base.sqrt().expect("sqrt"))
            }

            // e.g. x^2
            Some(x) if positive && x.eq(&BigDecimal::two()) => base.mul(self),

            // e.g. x^-0.25 = 1/x^0.25
            Some(x) if !positive && x.eq(&BigDecimal::zero_point_two_five().neg()) => {
                BigDecimal::one().div(base.pow(BigDecimal::zero_point_two_five()))
            }

            // e.g. x^-0.5 = 1/x^0.5
            Some(x) if !positive && x.eq(&BigDecimal::zero_point_five().neg()) => {
                BigDecimal::one().div(base.sqrt().expect("sqrt"))
            }

            // e.g. x^-1 == 1/x
            Some(x) if !positive && x.eq(&BigDecimal::one().neg()) => BigDecimal::one().div(base),

            // e.g. x^-1.25 == 1/x^1.25
            Some(x) if !positive && x.eq(&BigDecimal::one_point_two_five().neg()) => {
                BigDecimal::one().div(base.pow(BigDecimal::one_point_two_five()))
            }

            // e.g. x^-1.5 == 1/x^1.5
            Some(x) if !positive && x.eq(&BigDecimal::one_point_five().neg()) => {
                BigDecimal::one().div(base.pow(BigDecimal::one_point_five()))
            }

            // e.g. x^-2 == 1/x^2
            Some(x) if !positive && x.eq(&BigDecimal::two()) => {
                BigDecimal::one().div(base.pow(BigDecimal::two()))
            }

            _ => panic!("pow not implemented for exponent: {:?}", exp.unwrap()),
        }
    }
}

#[cfg(test)]
#[allow(clippy::inconsistent_digit_grouping)]
mod test {
    use crate::decimal::core::uint::U192;
    use crate::decimal::ops::{Div, Pow, Sub};
    use crate::decimal::{BigDecimal, Decimal, BIG_COMPUTE_SCALE, COMPUTE_SCALE};
    use std::str::FromStr;

    #[test]
    fn test_pow_with_integer_exp() {
        // 0**n = 0
        {
            let scale: u8 = 6;
            let base = Decimal::new(0, scale, false);
            let exp: u128 = 100;
            let result = base.pow(exp);
            let expected = Decimal::new(0, scale, false);
            assert_eq!(result, expected);
        }

        // n**0 = 1
        let scale: u8 = 6;
        let base = Decimal::from_u64(10).to_scale(scale);
        let exp: u128 = 0;
        let result = base.pow(exp);
        let expected = Decimal::from_u64(1).to_scale(scale);
        assert_eq!(result, expected);

        // 2**18 = 262,144
        {
            let scale: u8 = 6;
            let base = Decimal::from_u64(2).to_scale(scale);
            let exp: u128 = 18;
            let result = base.pow(exp);
            let expected = Decimal::from_u64(262_144).to_scale(scale);
            assert_eq!(result, expected);
        }

        // (-0.001459854015)**2 = 0.000002131174
        {
            let base = Decimal::from_str("-0.001459854015").unwrap();
            let exp: u128 = 2;
            let result = base.pow(exp);
            let expected = Decimal::from_str("0.000002131173").unwrap();
            assert_eq!(result, expected);
        }

        // (3420/3425-1)**2 = 0.000002131174
        {
            let mut base = Decimal::from_u64(3420)
                .to_compute_scale()
                .div(Decimal::from_u64(3425).to_compute_scale());
            base = base.sub(Decimal::one()).unwrap();
            let exp: u128 = 2;
            let result = base.pow(exp);
            let expected = Decimal::from_str("0.000002131173").unwrap();
            assert_eq!(result, expected);
        }

        // 3.41200000**8 = 18368.43602322
        {
            let base = Decimal::new(3_41200000, 8, false);
            let exp: u128 = 8;
            let result = base.pow(exp);
            let expected = Decimal::new(18368_43602280, 8, false);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_pow_with_decimal_exp() {
        // 42^-0.25 = 0.3928146509
        {
            let base = Decimal::new(42_000000, 6, false);
            let exp = Decimal::new(250000, 6, true);
            let result = base.pow(exp);
            let expected = Decimal::new(392814, 6, false);
            assert_eq!(result, expected);
        }

        // 42^-1 = 0.02380952381
        {
            let base = Decimal::new(42_000000, 6, false);
            let exp = Decimal::new(1_000000, 6, true);
            let result = base.pow(exp);
            let expected = Decimal::new(23809, 6, false);
            assert_eq!(result, expected);
        }

        // 42^0 = 1
        {
            let base = Decimal::new(42_000000, 6, false);
            let exp = Decimal::new(0, 6, false);
            let result = base.pow(exp);
            let expected = Decimal::new(1_000000, 6, false);
            assert_eq!(result, expected);
        }

        // 42^0.25 = 2.545729895021831
        {
            let base = Decimal::new(42_000000000000, 12, false);
            let exp = Decimal::new(250000000000, 12, false);
            let result = base.pow(exp);
            let expected = Decimal::new(2_545_729_895_021u128, 12, false);
            assert_eq!(result, expected);
        }

        // 42^0.5 = 6.48074069840786
        {
            let base = Decimal::new(42_000000000000, 12, false);
            let exp = Decimal::new(500000000000, 12, false);
            let result = base.pow(exp);
            let expected = Decimal::new(6_480_740_698_407u128, 12, false);
            assert_eq!(result, expected);
        }

        // 42^1 = 42
        {
            let base = Decimal::new(42_000000000000, 12, false);
            let exp = Decimal::new(1000000000000, 12, false);
            let result = base.pow(exp);
            let expected = Decimal::new(42_000000000000u128, 12, false);
            assert_eq!(result, expected);
        }

        // 42^1.25 = 106.920655590916882
        {
            let base = Decimal::new(42_000000000000, 12, false);
            let exp = Decimal::new(1250000000000, 12, false);
            let result = base.pow(exp);
            let expected = Decimal::new(106_920_655_590_882u128, 12, false);
            assert_eq!(result, expected);
        }

        // 42^1.5 = 272.19110933313013
        {
            let base = Decimal::new(42_000000000000, 12, false);
            let exp = Decimal::new(1500000000000, 12, false);
            let result = base.pow(exp);
            let expected = Decimal::new(272_191_109_333_094, 12, false);
            assert_eq!(result, expected);
        }

        // 42^2 = 1764
        {
            let base = Decimal::new(42_000000000000, 12, false);
            let exp = Decimal::new(2000000000000, 12, false);
            let result = base.pow(exp);
            let expected = Decimal::new(1764_000000000000u128, 12, false);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_pow_with_big_decimal_exp() {
        // 249383740734.349125162518^-1 = 4.009884513943... × 10^-18
        {
            let base = BigDecimal::new(
                U192::from(249383740734_349125162518u128),
                COMPUTE_SCALE,
                false,
            );
            let exp = BigDecimal::new(U192::from(1_000000000000u128), COMPUTE_SCALE, true);
            let result = base.pow(exp);
            let expected = BigDecimal::new(U192::from(4009884u128), BIG_COMPUTE_SCALE, false);
            assert_eq!(result, expected);
        }
    }
}
