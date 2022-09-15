use crate::decimal::Decimal;
use crate::decimal::errors::ErrorCode;

pub trait Compare<T>: Sized {
    fn eq(self, rhs: T) -> Result<bool, ErrorCode>;
    fn almost_eq(self, rhs: T, precision: u128) -> Result<bool, ErrorCode>;
    fn lt(self, rhs: T) -> Result<bool, ErrorCode>;
    fn gt(self, rhs: T) -> Result<bool, ErrorCode>;
    fn gte(self, rhs: T) -> Result<bool, ErrorCode>;
    fn lte(self, rhs: T) -> Result<bool, ErrorCode>;
    fn min(self, rhs: T) -> Self;
    fn max(self, rhs: T) -> Self;
}

/// Compare two [Decimal] values/scale with comparison query operators.
impl Compare<Decimal> for Decimal {
    /// Show if two [Decimal] values equal each other
    fn eq(self, other: Decimal) -> Result<bool, ErrorCode> {
        if self.scale != other.scale {
            Err(ErrorCode::DifferentScale)
        } else {
            Ok(self.value == other.value && self.negative == other.negative)
        }
    }

    /// Show if two [Decimal] values are almost equal to each other, given a precision
    fn almost_eq(self, other: Decimal, precision: u128) -> Result<bool, ErrorCode> {
        let difference = self.value.saturating_sub(other.value);
        Ok(difference.lt(&precision))
    }

    /// Show if one [Decimal] value is less than another.
    fn lt(self, other: Decimal) -> Result<bool, ErrorCode> {
        if self.scale != other.scale {
            Err(ErrorCode::DifferentScale)
        } else if self.negative && other.negative {
            Ok(self.value > other.value)
        } else if self.negative && !other.negative {
            Ok(true)
        } else if !self.negative && other.negative {
            Ok(false)
        } else {
            Ok(self.value < other.value)
        }
    }

    /// Show if one [Decimal] value is greater than another.
    fn gt(self, other: Decimal) -> Result<bool, ErrorCode> {
        if self.scale != other.scale {
            Err(ErrorCode::DifferentScale)
        } else if self.negative && other.negative {
            Ok(self.value < other.value)
        } else if self.negative && !other.negative {
            Ok(false)
        } else if !self.negative && other.negative {
            Ok(true)
        } else {
            Ok(self.value > other.value)
        }
    }

    /// Show if one [Decimal] value is greater than or equal to another.
    fn gte(self, other: Decimal) -> Result<bool, ErrorCode> {
        if self.scale != other.scale {
            Err(ErrorCode::DifferentScale)
        } else if self.negative && other.negative {
            Ok(self.value <= other.value)
        } else if self.negative && !other.negative {
            Ok(false)
        } else if !self.negative && other.negative {
            Ok(true)
        } else {
            Ok(self.value >= other.value)
        }
    }

    /// Show if one [Decimal] value is less than or equal to another.
    fn lte(self, other: Decimal) -> Result<bool, ErrorCode> {
        if self.scale != other.scale {
            Err(ErrorCode::DifferentScale)
        } else if self.negative && other.negative {
            Ok(self.value >= other.value)
        } else if self.negative && !other.negative {
            Ok(true)
        } else if !self.negative && other.negative {
            Ok(false)
        } else {
            Ok(self.value <= other.value)
        }
    }

    /// Show the minimum of two [Decimal] values.
    fn min(self, other: Decimal) -> Decimal {
        if self.lte(other).unwrap() {
            self
        } else {
            other
        }
    }

    /// Show the maximum of two [Decimal] values.
    fn max(self, other: Decimal) -> Decimal {
        if self.gte(other).unwrap() {
            self
        } else {
            other
        }
    }
}

#[cfg(test)]
mod test {
    use crate::decimal::core::Compare;
    use crate::decimal::Decimal;

    #[test]
    fn test_lte() {
        {
            let decimal = Decimal::new(1001, 4, false);
            let other = Decimal::new(33, 2, false);
            let result = decimal.lte(other);

            assert!(result.is_err());
        }

        {
            let decimal = Decimal::new(1001, 4, false);
            let other = Decimal::new(33, 4, false);
            let actual = decimal.lte(other).unwrap();
            let expected = false;

            assert_eq!(actual, expected);
        }

        {
            let decimal = Decimal::new(33, 4, false);
            let other = Decimal::new(33, 4, false);
            let actual = decimal.lte(other).unwrap();
            let expected = true;

            assert_eq!(actual, expected);
        }

        {
            let decimal = Decimal::new(10, 4, false);
            let other = Decimal::new(33, 4, false);
            let actual = decimal.lte(other).unwrap();
            let expected = true;

            assert_eq!(actual, expected);
        }

        {
            let decimal = Decimal::new(42, 0, true);
            let other = Decimal::new(42, 0, true);
            let actual = decimal.lte(other).unwrap();
            let expected = true;

            assert_eq!(actual, expected);
        }

        {
            let decimal = Decimal::new(42, 0, false);
            let other = Decimal::new(42, 0, true);
            let actual = decimal.lte(other).unwrap();
            let expected = false;

            assert_eq!(actual, expected);
        }

        {
            let decimal = Decimal::new(42, 0, true);
            let other = Decimal::new(42, 0, false);
            let actual = decimal.lte(other).unwrap();
            let expected = true;

            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_lt() {
        {
            let decimal = Decimal::new(1001, 4, false);
            let other = Decimal::new(33, 2, false);
            let result = decimal.lt(other);

            assert!(result.is_err());
        }

        {
            let decimal = Decimal::new(1001, 4, false);
            let other = Decimal::new(33, 4, false);
            let actual = decimal.lt(other).unwrap();
            let expected = false;

            assert_eq!(actual, expected);
        }

        {
            let decimal = Decimal::new(33, 4, false);
            let other = Decimal::new(33, 4, false);
            let actual = decimal.lt(other).unwrap();
            let expected = false;

            assert_eq!(actual, expected);
        }

        {
            let decimal = Decimal::new(10, 4, false);
            let other = Decimal::new(33, 4, false);
            let actual = decimal.lt(other).unwrap();
            let expected = true;

            assert_eq!(actual, expected);
        }

        {
            let decimal = Decimal::new(43, 0, true);
            let other = Decimal::new(42, 0, true);
            let actual = decimal.lt(other).unwrap();
            let expected = true;

            assert_eq!(actual, expected);
        }

        {
            let decimal = Decimal::new(43, 0, false);
            let other = Decimal::new(42, 0, true);
            let actual = decimal.lt(other).unwrap();
            let expected = false;

            assert_eq!(actual, expected);
        }

        {
            let decimal = Decimal::new(43, 0, true);
            let other = Decimal::new(42, 0, false);
            let actual = decimal.lt(other).unwrap();
            let expected = true;

            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_gt() {
        {
            let decimal = Decimal::new(1001, 4, false);
            let other = Decimal::new(33, 2, false);
            let result = decimal.gt(other);

            assert!(result.is_err());
        }

        {
            let decimal = Decimal::new(1001, 4, false);
            let other = Decimal::new(33, 4, false);
            let actual = decimal.gt(other).unwrap();
            let expected = true;

            assert_eq!(actual, expected);
        }

        {
            let decimal = Decimal::new(33, 4, false);
            let other = Decimal::new(33, 4, false);
            let actual = decimal.gt(other).unwrap();
            let expected = false;

            assert_eq!(actual, expected);
        }

        {
            let decimal = Decimal::new(10, 4, false);
            let other = Decimal::new(33, 4, false);
            let actual = decimal.gt(other).unwrap();
            let expected = false;

            assert_eq!(actual, expected);
        }

        {
            let decimal = Decimal::new(43, 0, true);
            let other = Decimal::new(42, 0, true);
            let actual = decimal.gt(other).unwrap();
            let expected = false;

            assert_eq!(actual, expected);
        }

        {
            let decimal = Decimal::new(43, 0, false);
            let other = Decimal::new(42, 0, true);
            let actual = decimal.gt(other).unwrap();
            let expected = true;

            assert_eq!(actual, expected);
        }

        {
            let decimal = Decimal::new(43, 0, true);
            let other = Decimal::new(42, 0, false);
            let actual = decimal.gt(other).unwrap();
            let expected = false;

            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_gte() {
        {
            let decimal = Decimal::new(1001, 4, false);
            let other = Decimal::new(33, 2, false);
            let result = decimal.gte(other);

            assert!(result.is_err());
        }

        {
            let decimal = Decimal::new(1001, 4, false);
            let other = Decimal::new(33, 4, false);
            let actual = decimal.gte(other).unwrap();
            let expected = true;

            assert_eq!(actual, expected);
        }

        {
            let decimal = Decimal::new(33, 4, false);
            let other = Decimal::new(33, 4, false);
            let actual = decimal.gte(other).unwrap();
            let expected = true;

            assert_eq!(actual, expected);
        }

        {
            let decimal = Decimal::new(10, 4, false);
            let other = Decimal::new(33, 4, false);
            let actual = decimal.gte(other).unwrap();
            let expected = false;

            assert_eq!(actual, expected);
        }

        {
            let decimal = Decimal::new(42, 0, true);
            let other = Decimal::new(42, 0, true);
            let actual = decimal.gte(other).unwrap();
            let expected = true;

            assert_eq!(actual, expected);
        }

        {
            let decimal = Decimal::new(42, 0, false);
            let other = Decimal::new(42, 0, true);
            let actual = decimal.gte(other).unwrap();
            let expected = true;

            assert_eq!(actual, expected);
        }

        {
            let decimal = Decimal::new(42, 0, true);
            let other = Decimal::new(42, 0, false);
            let actual = decimal.gte(other).unwrap();
            let expected = false;

            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_eq() {
        {
            let decimal = Decimal::new(1001, 4, false);
            let other = Decimal::new(33, 2, false);
            let result = decimal.eq(other);

            assert!(result.is_err());
        }

        {
            let decimal = Decimal::new(1001, 4, false);
            let other = Decimal::new(33, 4, false);
            let actual = decimal.eq(other).unwrap();
            let expected = false;

            assert_eq!(actual, expected);
        }

        {
            let decimal = Decimal::new(33, 4, false);
            let other = Decimal::new(33, 4, false);
            let actual = decimal.eq(other).unwrap();
            let expected = true;

            assert_eq!(actual, expected);
        }

        {
            let decimal = Decimal::new(10, 4, false);
            let other = Decimal::new(33, 4, false);
            let actual = decimal.eq(other).unwrap();
            let expected = false;

            assert_eq!(actual, expected);
        }

        {
            let decimal = Decimal::new(33, 4, false);
            let other = Decimal::new(33, 4, true);
            let actual = decimal.eq(other).unwrap();
            let expected = false;

            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_min_max() {
        {
            let decimal = Decimal::new(10, 2, false);
            let other = Decimal::new(11, 2, false);
            let result = decimal.min(other);

            assert_eq!(decimal, result);
        }

        {
            let decimal = Decimal::new(10, 2, false);
            let other = Decimal::new(11, 2, false);
            let result = decimal.max(other);

            assert_eq!(other, result);
        }

        {
            let decimal = Decimal::new(10, 2, false);
            let other = Decimal::new(11, 2, true);
            let result = decimal.min(other);

            assert_eq!(other, result);
        }

        {
            let decimal = Decimal::new(10, 2, false);
            let other = Decimal::new(11, 2, true);
            let result = decimal.max(other);

            assert_eq!(decimal, result);
        }
    }
}
