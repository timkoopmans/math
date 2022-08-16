use crate::decimal::ops::Add;
use crate::decimal::Decimal;
use crate::decimal::errors::DecimalError;

pub trait Sub<T>: Sized {
    fn sub(self, rhs: T) -> Result<Self, DecimalError>;
}

/// Subtract another [Decimal] value from itself, including signed subtraction.
impl Sub<Decimal> for Decimal {
    fn sub(self, rhs: Decimal) -> Result<Self, DecimalError> {
        // as a - b is always a + (-b) so we let add handle it
        let new_rhs = Decimal {
            negative: !rhs.negative,
            ..rhs
        };
        self.add(new_rhs)
    }
}

#[cfg(test)]
mod test {
    use crate::decimal::ops::Sub;
    use crate::decimal::Decimal;

    #[test]
    fn test_sub() {
        {
            // error: 0.000012 - 1.3 = DecimalError::DifferentScale
            let a = Decimal::new(12, 6, false);
            let b = Decimal::new(13, 2, false);
            let actual = a.sub(b);

            assert!(actual.is_err());
            assert!(matches!(
                actual,
                Err(crate::decimal::errors::DecimalError::DifferentScale)
            ));
        }

        {
            // test 1.3 - 1.2 = 0.1
            let a = Decimal::new(13, 1, false);
            let b = Decimal::new(12, 1, false);
            let actual = a.sub(b).unwrap();
            let expected = Decimal::new(1, 1, false);

            assert_eq!(actual, expected);
        }

        {
            // test: 10 - 15 = -5
            let a = Decimal::new(10, 6, false);
            let b = Decimal::new(15, 6, false);

            let expected = Decimal::new(5, 6, true);

            assert_eq!(a.sub(b).unwrap(), expected);
        }

        {
            // test: 15 - 10 = 5
            let a = Decimal::new(15, 6, false);
            let b = Decimal::new(10, 6, false);

            let expected = Decimal::new(5, 6, false);

            assert_eq!(a.sub(b).unwrap(), expected);
        }

        {
            // test: -10 - (-15) = 5
            let a = Decimal::new(10, 6, true);
            let b = Decimal::new(15, 6, true);

            let expected = Decimal::new(5, 6, false);

            assert_eq!(a.sub(b).unwrap(), expected);
        }

        {
            // test: -10 - (-5) = -5
            let a = Decimal::new(10, 6, true);
            let b = Decimal::new(5, 6, true);

            let expected = Decimal::new(5, 6, true);

            assert_eq!(a.sub(b).unwrap(), expected);
        }

        {
            // test: -10 - 15 = -25
            let a = Decimal::new(10, 6, true);
            let b = Decimal::new(15, 6, false);

            let expected = Decimal::new(25, 6, true);

            assert_eq!(a.sub(b).unwrap(), expected);
        }

        {
            // test: 10 - (-15) = 25
            let a = Decimal::new(10, 6, false);
            let b = Decimal::new(15, 6, true);

            let expected = Decimal::new(25, 6, false);

            assert_eq!(a.sub(b).unwrap(), expected);
        }

        {
            // test: 0 - 15 = -15
            let a = Decimal::new(0, 6, false);
            let b = Decimal::new(15, 6, false);

            let expected = Decimal::new(15, 6, true);

            assert_eq!(a.sub(b).unwrap(), expected);
        }

        {
            // test: 0 - (-15) = 15
            let a = Decimal::new(0, 6, false);
            let b = Decimal::new(15, 6, true);

            let expected = Decimal::new(15, 6, false);

            assert_eq!(a.sub(b).unwrap(), expected);
        }
    }

    #[test]
    #[should_panic(expected = "decimal: overflow in method Decimal::add()")]
    fn test_sub_panic() {
        let a = Decimal::new(u128::MAX, 2, true);
        let b = Decimal::new(1, 2, false);
        assert!(a.sub(b).is_err());
    }
}
