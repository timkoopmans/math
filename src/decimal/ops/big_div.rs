use crate::decimal::core::uint::U192;
use crate::decimal::Decimal;

pub trait BigDiv<T>: Sized {
    fn big_div(self, rhs: T) -> Self;
}

/// Divide a [Decimal] over another [Decimal], including signed division after
/// converting its value to and from a u256 in order to support ranges > u128.
impl BigDiv<Decimal> for Decimal {
    fn big_div(self, rhs: Decimal) -> Self {
        let lhs = U192::try_from(self.value)
            .unwrap_or_else(|_| panic!("decimal: lhs value does not fit in Decimal::big_div()"));
        let denominator = U192::try_from(rhs.denominator()).unwrap_or_else(|_| {
            panic!("decimal: denominator value does not fit in Decimal::big_mul()")
        });
        let negative = self.negative != rhs.negative;
        let rhs = U192::try_from(rhs.value)
            .unwrap_or_else(|_| panic!("decimal: rhs value does not fit in Decimal::big_div()"));

        let result = lhs
            .checked_mul(denominator)
            .unwrap_or_else(|| panic!("decimal: overflow in method Decimal::big_div().checked_mul"))
            .checked_div(rhs)
            .unwrap_or_else(|| {
                panic!("decimal: overflow in method Decimal::big_div().checked_div")
            });

        let value: u128 = result.try_into().unwrap_or_else(|_| {
            panic!("decimal: overflow in method Decimal::big_div() casting to u128")
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
    use crate::decimal::ops::BigDiv;
    use crate::decimal::Decimal;

    #[test]
    fn test_big_div() {
        {
            // test: 18446744073709551615.000000000 / 0.000000001 = 18446744073709551615000000000.000000000
            let a = Decimal::new(u128::MAX, 0, false);
            let b = Decimal::new(1, 0, false);
            let actual = a.big_div(b);
            let expected = Decimal::new(u128::MAX, 0, false);

            assert_eq!(actual, expected);
        }
    }
}
