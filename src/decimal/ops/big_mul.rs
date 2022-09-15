use crate::decimal::core::uint::U192;
use crate::decimal::Decimal;

pub trait BigMul<T>: Sized {
    fn big_mul(self, rhs: T) -> Self;
}

/// Multiply another [Decimal] value against itself, including signed multiplication after
/// converting its value to and from a u256 in order to support ranges > u128.
impl BigMul<Decimal> for Decimal {
    fn big_mul(self, rhs: Decimal) -> Self {
        let lhs = U192::try_from(self.value)
            .unwrap_or_else(|_| panic!("decimal: lhs value does not fit in Decimal::big_mul()"));
        let denominator = U192::try_from(rhs.denominator()).unwrap_or_else(|_| {
            panic!("decimal: denominator value does not fit in Decimal::big_mul()")
        });
        let negative = self.negative != rhs.negative;
        let rhs = U192::try_from(rhs.value)
            .unwrap_or_else(|_| panic!("decimal: rhs value does not fit in Decimal::big_mul()"));

        let result = lhs
            .checked_mul(rhs)
            .unwrap_or_else(|| panic!("decimal: overflow in method Decimal::big_mul().checked_mul"))
            .checked_div(denominator)
            .unwrap_or_else(|| {
                panic!("decimal: overflow in method Decimal::big_mul().checked_div")
            });

        let value: u128 = result.try_into().unwrap_or_else(|_| {
            panic!("decimal: overflow in method Decimal::big_mul() casting to u128")
        });

        Self {
            value,
            scale: self.scale,
            negative,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::decimal::ops::BigMul;
    use crate::decimal::Decimal;

    #[test]
    fn test_big_mul_decimal() {
        {
            // test: 340282366920938463463374607.431768211454 * 0.000000000002 = 680564733841876.926926749214
            let a = Decimal::new(u128::MAX - 1, 12, false);
            let b = Decimal::new(2, 12, false);
            let actual = a.big_mul(b);
            let expected = Decimal::new(680_564_733_841_876_926_926_749_214, 12, false);

            assert_eq!(actual, expected);
        }
    }
}
