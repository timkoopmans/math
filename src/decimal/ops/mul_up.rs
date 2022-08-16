use crate::decimal::Decimal;

pub trait MulUp<T>: Sized {
    fn mul_up(self, rhs: T) -> Self;
}

/// Multiply another [Decimal] value against itself, including signed multiplication
/// and round up (ceiling) the value.
impl MulUp<Decimal> for Decimal {
    fn mul_up(self, rhs: Decimal) -> Self {
        let denominator = rhs.denominator();

        Self {
            value: self
                .value
                .checked_mul(rhs.value)
                .unwrap_or_else(|| {
                    panic!("decimal: overflow in method Decimal::mul_up().checked_mul")
                })
                .checked_add(denominator.checked_sub(1).unwrap())
                .unwrap_or_else(|| {
                    panic!("decimal: overflow in method Decimal::mul_up().checked_add")
                })
                .checked_div(denominator)
                .unwrap_or_else(|| {
                    panic!("decimal: overflow in method Decimal::mul_up().checked_div")
                }),
            scale: self.scale,
            negative: self.negative != rhs.negative,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::decimal::ops::MulUp;
    use crate::decimal::Decimal;

    #[test]
    fn test_mul_up() {
        // mul of small number
        {
            let a = Decimal::new(1, 12, false);
            let b = Decimal::new(1, 12, false);
            assert_eq!(a.mul_up(b), Decimal::new(1, 12, false));
        }

        // mul same precision
        // 1.000000 * 0.300000 = 0.300000
        {
            let a = Decimal::new(1000000, 6, false);
            let b = Decimal::new(300000, 6, false);
            assert_eq!(a.mul_up(b), Decimal::new(300000, 6, false));
        }

        // mul by zero
        // 1.00 * 0 = 0.00
        {
            let a = Decimal::new(100, 2, false);
            let b = Decimal::new(0, 0, false);
            assert_eq!(a.mul_up(b), Decimal::new(0, 2, false));
        }

        // mul different decimals increases precision
        {
            let a = Decimal::new(1_000_000_000, 9, false);
            let b = Decimal::new(3, 6, false);
            assert_eq!(a.mul_up(b), Decimal::new(3000, 9, false));
        }
    }
}
