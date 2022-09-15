use crate::decimal::{BigDecimal, Decimal};

pub trait Mul<T>: Sized {
    fn mul(self, rhs: T) -> Self;
}

/// Multiply another [Decimal] value against itself, including signed multiplication.
impl Mul<Decimal> for Decimal {
    fn mul(self, rhs: Decimal) -> Self {
        Self {
            value: self
                .value
                .checked_mul(rhs.value)
                .unwrap_or_else(|| panic!("decimal: overflow in method Decimal::mul().checked_mul"))
                .checked_div(rhs.denominator())
                .unwrap_or_else(|| {
                    panic!("decimal: overflow in method Decimal::mul().checked_div")
                }),
            scale: self.scale,
            negative: self.negative != rhs.negative,
        }
    }
}

/// Multiply an unsigned integer value against a [Decimal].
impl Mul<u128> for Decimal {
    fn mul(self, rhs: u128) -> Self {
        Self {
            value: self.value.checked_mul(rhs).unwrap_or_else(|| {
                panic!("decimal: overflow in method Decimal::mul().checked_mul")
            }),
            scale: self.scale,
            negative: self.negative,
        }
    }
}

/// Multiply another [BigDecimal] value against itself, including signed multiplication.
impl Mul<BigDecimal> for BigDecimal {
    fn mul(self, rhs: BigDecimal) -> Self {
        Self {
            value: self
                .value
                .checked_mul(rhs.value)
                .unwrap_or_else(|| {
                    panic!("decimal: overflow in method BigDecimal::mul().checked_mul")
                })
                .checked_div(self.denominator())
                .unwrap_or_else(|| {
                    panic!("decimal: overflow in method BigDecimal::mul().checked_div")
                }),
            scale: self.scale,
            negative: self.negative != rhs.negative,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::decimal::core::uint::U192;
    use crate::decimal::ops::Mul;
    use crate::decimal::{BigDecimal, Decimal};

    #[test]
    fn test_mul_decimal() {
        {
            // test: 1.234 * 0.04321 = 0.053
            let a = Decimal::new(1234, 3, false);
            let b = Decimal::new(4321, 5, false);
            let actual = a.mul(b);
            let expected = Decimal::new(53, 3, false);

            assert_eq!(actual, expected);
        }

        {
            // test: -4 * -3 = 12
            let a = Decimal::new(4, 0, true);
            let b = Decimal::new(3, 0, true);
            let expected = Decimal::new(12, 0, false);

            assert_eq!(a.mul(b), expected);
        }

        {
            // test: -4 * 3 = -12
            let a = Decimal::new(4, 0, true);
            let b = Decimal::new(3, 0, false);
            let expected = Decimal::new(12, 0, true);

            assert_eq!(a.mul(b), expected);
        }

        {
            // test: 4 * -3 = -12
            let a = Decimal::new(4, 0, false);
            let b = Decimal::new(3, 0, true);
            let expected = Decimal::new(12, 0, true);

            assert_eq!(a.mul(b), expected);
        }

        {
            // test: 4 * 3 = 12
            let a = Decimal::new(4, 0, false);
            let b = Decimal::new(3, 0, false);
            let expected = Decimal::new(12, 0, false);

            assert_eq!(a.mul(b), expected);
        }
    }

    #[test]
    #[should_panic(expected = "decimal: overflow in method Decimal::mul().checked_mul")]
    fn test_mul_decimal_panic() {
        let a = Decimal::new(u128::MAX - 1, 3, false);
        let b = Decimal::new(2, 3, false);
        a.mul(b);
    }

    #[test]
    fn test_mul_u128() {
        {
            // test: 98.76 * 555 = 54811.80
            let a = Decimal::new(9876, 2, false);
            let b: u128 = 555;
            let actual = a.mul(b);
            let expected = Decimal::new(5481180, 2, false);

            assert_eq!(actual, expected);
        }
    }

    #[test]
    #[should_panic(expected = "decimal: overflow in method Decimal::mul().checked_mul")]
    fn test_mul_u128_panic() {
        let a = Decimal::new(u128::MAX, 2, false);
        let b = 2;
        a.mul(b);
    }

    #[test]
    fn test_mul_u192() {
        {
            // test: 18446744073709551615 * 18446744073709551615
            // = 340282366920938463426481119284349108225
            let a = BigDecimal::new(U192::from(u64::MAX), 0, false);
            let b = BigDecimal::new(U192::from(u64::MAX), 0, false);

            let actual = a.mul(b);
            let expected = BigDecimal::new(
                U192([1, u64::MAX.checked_sub(1).expect("u64::MAX"), 0]),
                0,
                false,
            );

            assert_eq!(actual, expected);
        }
    }
}
