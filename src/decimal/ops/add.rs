use crate::decimal::Decimal;
use crate::decimal::errors::DecimalError;
use std::cmp::Ordering;

pub trait Add<T>: Sized {
    fn add(self, rhs: T) -> Result<Self, DecimalError>;
}

/// Add another [Decimal] value to itself, including signed addition.
impl Add<Decimal> for Decimal {
    fn add(self, rhs: Decimal) -> Result<Self, DecimalError> {
        if self.scale != rhs.scale {
            Err(DecimalError::DifferentScale)
        } else if self.negative == rhs.negative {
            // covers when both positive, and both negative.
            // just add the add absolute values and use common sign
            // e.g: (-4) + (-3) = -7 ; 4 + 3 = 7;
            Ok(Self {
                value: self
                    .value
                    .checked_add(rhs.value)
                    .unwrap_or_else(|| panic!("decimal: overflow in method Decimal::add()")),
                scale: self.scale,
                negative: self.negative,
            })
        } else {
            // if different signs value is the difference of absolute values.
            // (so need to know which has bigger absolute value)
            // sign is the sign of the one with bigger absolute value
            match self.value.cmp(&rhs.value) {
                Ordering::Greater => {
                    // e.g: 4 + (-3) = 1 ; -4 + 3 = -1;
                    Ok(Self {
                        value: self.value.checked_sub(rhs.value).expect("checked_sub"),
                        scale: self.scale,
                        negative: self.negative,
                    })
                }
                Ordering::Less => {
                    // e.g: 2 + (-5) = -3 ; -2 + 5 = 3;
                    Ok(Self {
                        value: rhs.value.checked_sub(self.value).expect("checked_sub"),
                        scale: self.scale,
                        negative: rhs.negative,
                    })
                }
                Ordering::Equal => {
                    // if equal abs value and opposite sign then result is zero
                    Ok(Self {
                        value: 0,
                        scale: self.scale,
                        negative: false,
                    })
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::decimal::ops::Add;
    use crate::decimal::Decimal;

    #[test]
    fn test_add() {
        {
            // error: 0.000012 + 1.3 = DecimalError::DifferentScale
            let a = Decimal::new(12, 6, false);
            let b = Decimal::new(13, 2, false);
            let actual = a.add(b);

            assert!(actual.is_err());
            assert!(matches!(
                actual,
                Err(crate::decimal::errors::DecimalError::DifferentScale)
            ));
        }

        {
            // test: 1.2 + 1.3 = 2.5
            let a = Decimal::new(12, 1, false);
            let b = Decimal::new(13, 1, false);
            let actual = a.add(b).unwrap();
            let expected = Decimal::new(25, 1, false);

            assert_eq!(actual, expected);
        }

        {
            // test: 2 + 2 = 4
            let a = Decimal::new(2, 0, false);
            let b = Decimal::new(2, 0, false);

            let expected = Decimal::new(4, 0, false);

            assert_eq!(a.add(b).unwrap(), expected);
        }

        {
            // test: -2 + (-2) = -4
            let a = Decimal::new(2, 0, true);
            let b = Decimal::new(2, 0, true);

            let expected = Decimal::new(4, 0, true);

            assert_eq!(a.add(b).unwrap(), expected);
        }

        {
            // test: 4 + (-3) = +1
            let a = Decimal::new(4, 0, false);
            let b = Decimal::new(3, 0, true);

            let expected = Decimal::new(1, 0, false);

            assert_eq!(a.add(b).unwrap(), expected);
        }

        {
            // test: 2 + (-5) = -3;
            let a = Decimal::new(2, 0, false);
            let b = Decimal::new(5, 0, true);

            let expected = Decimal::new(3, 0, true);

            assert_eq!(a.add(b).unwrap(), expected);
        }

        {
            // test -4 + 3 = -1
            let a = Decimal::new(4, 0, true);
            let b = Decimal::new(3, 0, false);

            let expected = Decimal::new(1, 0, true);

            assert_eq!(a.add(b).unwrap(), expected);
        }

        {
            // test: -2 + 5 = 3
            let a = Decimal::new(2, 0, true);
            let b = Decimal::new(5, 0, false);

            let expected = Decimal::new(3, 0, false);

            assert_eq!(a.add(b).unwrap(), expected);
        }
    }

    #[test]
    #[should_panic(expected = "decimal: overflow in method Decimal::add()")]
    fn test_add_panic() {
        let a = Decimal::new(u128::MAX - 1, 2, false);
        let b = Decimal::new(2, 2, false);

        assert!(a.add(b).is_err());
    }
}
