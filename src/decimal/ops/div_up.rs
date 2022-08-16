use crate::decimal::Decimal;

pub trait DivUp<T>: Sized {
    fn div_up(self, rhs: T) -> Self;
}

/// Divide a [Decimal] over another [Decimal], including signed division.
/// and round up (ceiling) the value.
impl DivUp<Decimal> for Decimal {
    fn div_up(self, rhs: Decimal) -> Self {
        Self {
            value: self
                .value
                .checked_mul(rhs.denominator())
                .unwrap_or_else(|| {
                    panic!("decimal: overflow in method Decimal::div_up().checked_mul")
                })
                .checked_add(rhs.value.checked_sub(1).unwrap_or_else(|| {
                    panic!("decimal: overflow in method Decimal::div_up().checked_sub")
                }))
                .unwrap_or_else(|| {
                    panic!("decimal: overflow in method Decimal::div_up().checked_div")
                })
                .checked_div(rhs.value)
                .unwrap_or_else(|| {
                    panic!("decimal: overflow in method Decimal::div_up().checked_div")
                }),
            scale: self.scale,
            negative: self.negative != rhs.negative,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::decimal::core::Compare;
    use crate::decimal::ops::DivUp;
    use crate::decimal::Decimal;

    #[test]
    fn test_div_up() {
        // 0/n = 0
        {
            let a = Decimal::new(0, 0, false);
            let b = Decimal::new(1, 0, false);
            assert_eq!(a.div_up(b), Decimal::new(0, 0, false));
        }

        // 1/2 = 1 rounded up
        {
            let a = Decimal::new(1, 0, false);
            let b = Decimal::new(2, 0, false);
            assert_eq!(a.div_up(b), Decimal::new(1, 0, false));
        }

        // 200,000.000001/2 = 100000.000001 rounded up
        {
            let a = Decimal::new(200_000_000_001, 6, false);
            let b = Decimal::new(2_000, 3, false);
            assert!(!a
                .div_up(b)
                .lt(Decimal::new(100_000_000_001, 6, false))
                .unwrap());
        }

        // 42.00/10 = 4.20 = 5.00 rounded up
        {
            let a = Decimal::new(42, 2, false);
            let b = Decimal::new(10, 0, false);
            assert_eq!(a.div_up(b), Decimal::new(5, 2, false));
        }
    }
}
