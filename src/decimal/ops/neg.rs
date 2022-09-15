use crate::decimal::{BigDecimal, Decimal};

pub trait Neg<T>: Sized {
    fn neg(self) -> Self;
}

/// An implementation of Neg for [Decimal], which allows the use of - to negate its value.
impl Neg<Decimal> for Decimal {
    fn neg(self) -> Self {
        if self.is_negative() {
            Self {
                value: self.value,
                scale: self.scale,
                negative: false,
            }
        } else if self.is_positive() {
            Self {
                value: self.value,
                scale: self.scale,
                negative: true,
            }
        } else {
            self
        }
    }
}

/// An implementation of Neg for [BigDecimal], which allows the use of - to negate its value.
impl Neg<BigDecimal> for BigDecimal {
    fn neg(self) -> Self {
        if self.is_negative() {
            Self {
                value: self.value,
                scale: self.scale,
                negative: false,
            }
        } else if self.is_positive() {
            Self {
                value: self.value,
                scale: self.scale,
                negative: true,
            }
        } else {
            self
        }
    }
}

#[cfg(test)]
mod test {
    use crate::decimal::ops::Neg;
    use crate::decimal::Decimal;

    #[test]
    fn test_neg() {
        // when zero, is zero
        {
            let decimal = Decimal::new(0, 0, false);
            assert!(decimal.neg().is_zero());
        }

        // when positive, is negative
        {
            let decimal = Decimal::new(42, 4, false);
            assert!(decimal.neg().is_negative());
        }

        // when negative, is positive
        {
            let decimal = Decimal::new(42, 4, true);
            assert!(decimal.neg().is_positive());
        }
    }
}
