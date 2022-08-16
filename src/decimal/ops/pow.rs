use crate::decimal::core::Compare;
use crate::decimal::ops::sqrt::Sqrt;
use crate::decimal::ops::{BigMul, Div, Mul};
use crate::decimal::Decimal;

pub trait Pow<T>: Sized {
    fn pow(self, rhs: T) -> Self;
}

/// Calculate the power of a [Decimal] with another [Decimal] as the exponent.
impl Pow<Decimal> for Decimal {
    fn pow(self, exp: Decimal) -> Self {
        let one = Decimal::from_u64(1).to_scale(self.scale);
        let zero_point_two_five = Decimal::from_u64(1)
            .to_scale(self.scale)
            .div(Decimal::from_u64(4).to_scale(self.scale));
        let zero_point_five = Decimal::from_u64(1)
            .to_scale(self.scale)
            .div(Decimal::from_u64(2).to_scale(self.scale));
        let one_point_two_five = Decimal::from_u64(5)
            .to_scale(self.scale)
            .div(Decimal::from_u64(4).to_scale(self.scale));
        let one_point_five = Decimal::from_u64(3)
            .to_scale(self.scale)
            .div(Decimal::from_u64(2).to_scale(self.scale));

        let exp = Some(exp);
        match exp {
            // e.g. x^0 = 1
            Some(x) if x.is_zero() => one,
            // e.g. x^0.25 = ⁴√x = √(√x) = sqrt(sqrt(x))
            Some(x) if x.eq(zero_point_two_five).unwrap() => self.sqrt().unwrap().sqrt().unwrap(),
            // e.g. x^0.5 = √x = sqrt(x)
            Some(x) if x.eq(zero_point_five).unwrap() => self.sqrt().unwrap(),
            // e.g. x^1 = x
            Some(x) if x.eq(one).unwrap() => self,
            // e.g. x^1.25 = x(√(√x)) = x(sqrt(sqrt(x)))
            Some(x) if x.eq(one_point_two_five).unwrap() => {
                self.mul(self.sqrt().unwrap().sqrt().unwrap())
            }
            // e.g. x^1.50 = x(√x) = x(sqrt(x))
            Some(x) if x.eq(one_point_five).unwrap() => self.mul(self.sqrt().unwrap()),
            // e.g. x^2
            Some(x) if x.is_integer() && x.is_positive() => self.pow(x.abs() as u128),
            // e.g. x^-2 == 1/x^2
            Some(x) if x.is_integer() && x.is_negative() => one.div(self.pow(x.abs() as u128)),
            // e.g. x^-0.5 = 1/x^0.5
            Some(x) if x.is_negative() => one.div(self.pow(Decimal::new(x.value, x.scale, false))),
            _ => panic!("pow not implemented for exponent: {}", exp.unwrap()),
        }
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

#[cfg(test)]
mod test {
    use crate::decimal::ops::{Div, Pow, Sub};
    use crate::decimal::Decimal;
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
        let base = Decimal::new(42_000000, 6, false);
        let exp = Decimal::new(250000, 6, true);
        let result = base.pow(exp);
        let expected = Decimal::new(392814, 6, false);
        assert_eq!(result, expected);

        // 42^-1 = 0.02380952381
        let base = Decimal::new(42_000000, 6, false);
        let exp = Decimal::new(1_000000, 6, true);
        let result = base.pow(exp);
        let expected = Decimal::new(23809, 6, false);
        assert_eq!(result, expected);

        // 42^0 = 1
        let base = Decimal::new(42_000000, 6, false);
        let exp = Decimal::new(0, 6, false);
        let result = base.pow(exp);
        let expected = Decimal::new(1_000000, 6, false);
        assert_eq!(result, expected);

        // 42^0.25 = 2.545729895021831
        let base = Decimal::new(42_000000000000, 12, false);
        let exp = Decimal::new(250000000000, 12, false);
        let result = base.pow(exp);
        let expected = Decimal::new(2_545_729_895_021u128, 12, false);
        assert_eq!(result, expected);

        // 42^0.5 = 6.48074069840786
        let base = Decimal::new(42_000000000000, 12, false);
        let exp = Decimal::new(500000000000, 12, false);
        let result = base.pow(exp);
        let expected = Decimal::new(6_480_740_698_407u128, 12, false);
        assert_eq!(result, expected);

        // 42^1 = 42
        let base = Decimal::new(42_000000000000, 12, false);
        let exp = Decimal::new(1000000000000, 12, false);
        let result = base.pow(exp);
        let expected = Decimal::new(42_000000000000u128, 12, false);
        assert_eq!(result, expected);

        // 42^1.25 = 106.920655590916882
        let base = Decimal::new(42_000000000000, 12, false);
        let exp = Decimal::new(1250000000000, 12, false);
        let result = base.pow(exp);
        let expected = Decimal::new(106_920_655_590_882u128, 12, false);
        assert_eq!(result, expected);

        // 42^1.5 = 272.19110933313013
        let base = Decimal::new(42_000000000000, 12, false);
        let exp = Decimal::new(1500000000000, 12, false);
        let result = base.pow(exp);
        let expected = Decimal::new(272_191_109_333_094, 12, false);
        assert_eq!(result, expected);

        // 42^2 = 1764
        let base = Decimal::new(42_000000000000, 12, false);
        let exp = Decimal::new(2000000000000, 12, false);
        let result = base.pow(exp);
        let expected = Decimal::new(1764_000000000000u128, 12, false);
        assert_eq!(result, expected);
    }
}
