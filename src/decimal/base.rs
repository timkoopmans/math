use crate::decimal::ops::{DivUp, MulUp, Sub};
use crate::decimal::errors::DecimalError;
use num_traits::FromPrimitive;
use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;
use thiserror::Error;

/// Internal scale used for high precision compute operations
pub const COMPUTE_SCALE: u8 = 12;

/// [Decimal] representation of a number with a value, scale (precision in terms of number of decimal places
/// and a negative boolean to handle signed arithmetic.
#[derive(Default, Clone, Copy, PartialEq, PartialOrd, Debug, Error)]
pub struct Decimal {
    pub value: u128,
    pub scale: u8,
    pub negative: bool,
}

impl Decimal {
    /// Create a new [Decimal] from its value, scale and negative parts.
    pub fn new(value: u128, scale: u8, negative: bool) -> Self {
        Self {
            value,
            scale,
            negative,
        }
    }

    pub fn zero() -> Decimal {
        Decimal::from_u64(0).to_compute_scale()
    }

    pub fn one() -> Decimal {
        Decimal::from_u64(1).to_compute_scale()
    }

    pub fn two() -> Decimal {
        Decimal::from_u64(2).to_compute_scale()
    }

    /// Create a [Decimal] from an unsigned integer, assumed positive by default.
    pub fn from_u64(integer: u64) -> Self {
        Decimal {
            value: integer.into(),
            ..Decimal::default()
        }
    }

    /// Create a [Decimal] from an unsigned integer, assumed positive by default.
    pub fn from_u128(integer: u128) -> Self {
        Decimal {
            value: integer,
            scale: 0,
            ..Decimal::default()
        }
    }

    /// Computes the absolute value of a [Decimal] and round down (floor) the value.
    pub fn abs(self) -> u64 {
        self.to_scale(0).into()
    }

    /// Computes the absolute value of a [Decimal] and round up (ceiling) the value.
    pub fn abs_up(self) -> u64 {
        self.to_scale_up(0).into()
    }

    /// Create a [Decimal] from an unsigned amount with scale, assumed positive by default.
    pub fn from_scaled_amount(amount: u64, scale: u8) -> Self {
        Decimal {
            value: amount.into(),
            scale,
            ..Decimal::default()
        }
    }

    /// Convert a [Decimal] back to a scaled u64 amount.
    pub fn to_scaled_amount(self, scale: u8) -> u64 {
        self.to_scale(scale).into()
    }

    /// Convert a [Decimal] back to a scaled u64 amount and round up (ceiling) the value.
    pub fn to_scaled_amount_up(self, scale: u8) -> u64 {
        self.to_scale_up(scale).into()
    }

    /// Modify the scale (precision) of a [Decimal] to a different scale.
    pub fn to_scale(self, scale: u8) -> Self {
        Self {
            value: match self.scale.cmp(&scale) {
                Ordering::Equal => self.value,
                Ordering::Greater => self
                    .value
                    .checked_div(10u128.pow((self.scale.checked_sub(scale).unwrap()).into()))
                    .expect("scaled_down"),
                _ => self
                    .value
                    .checked_mul(10u128.pow((scale.checked_sub(self.scale).unwrap()).into()))
                    .expect("scaled_up"),
            },
            scale,
            negative: self.negative,
        }
    }

    /// Modify the scale (precision) of a [Decimal] to a different scale and round up (ceiling) the value.
    pub fn to_scale_up(self, scale: u8) -> Self {
        let decimal = Self::new(self.value, scale, self.negative);
        if self.scale >= scale {
            decimal.div_up(Self::new(
                10u128.pow((self.scale.checked_sub(scale).unwrap()).try_into().unwrap()),
                0,
                self.negative,
            ))
        } else {
            decimal.mul_up(Self::new(
                10u128.pow((scale.checked_sub(self.scale).unwrap()).try_into().unwrap()),
                0,
                self.negative,
            ))
        }
    }

    /// Convert to a higher precision compute scale
    pub fn to_compute_scale(self) -> Self {
        self.to_scale(COMPUTE_SCALE)
    }

    /// Show the scale of a [Decimal] expressed as a power of 10.
    pub fn denominator(self) -> u128 {
        10u128.pow(self.scale.into())
    }

    /// Returns true if [Decimal] is positive and false if the number is zero or negative.
    pub fn is_positive(self) -> bool {
        !self.negative && !self.is_zero()
    }

    /// Returns true if [Decimal] is negative and false if the number is zero or positive.
    pub fn is_negative(self) -> bool {
        self.negative && !self.is_zero()
    }

    /// Returns true if [Decimal] value is zero.
    pub fn is_zero(self) -> bool {
        self.value == 0
    }

    /// Returns true if and only if the [Decimal] is an exact integer.
    pub fn is_integer(self) -> bool {
        let integer = self.to_scale(0).to_scale(self.scale);

        self.sub(integer).expect("zero").is_zero()
    }

    /// Converts a string slice in a given base to a [Decimal].
    /// The string is expected to be an optional - sign followed by digits.
    /// Leading and trailing whitespace represent an error.
    /// Digits are a subset of these characters, depending on radix.
    /// This function panics if radix is not in the range base 10.
    fn from_str_radix(s: &str, radix: u32) -> Result<Decimal, DecimalError> {
        if radix != 10 {
            return Err(DecimalError::ParseErrorBase10);
        }

        let exp_separator: &[_] = &['e', 'E'];

        // split slice into base and exponent parts
        let (base, exp) = match s.find(exp_separator) {
            // exponent defaults to 0 if (e|E) not found
            None => (s, 0),

            // split and parse exponent field
            Some(loc) => {
                // slice up to `loc` and 1 after to skip the 'e' char
                let (base, exp) = (&s[..loc], &s[loc + 1..]);
                (base, i64::from_str(exp).unwrap())
            }
        };

        if base.is_empty() {
            return Err(DecimalError::ParseErrorEmpty);
        }

        // look for signed (negative) decimals
        let (base, negative): (String, _) = match base.find('-') {
            // no sign found, pass to Decimal
            None => (base.to_string(), false),
            Some(loc) => {
                if loc == 0 {
                    (String::from(&base[1..]), true)
                } else {
                    // negative sign not in the first position
                    return Err(DecimalError::ParseError);
                }
            }
        };

        // split decimal into a digit string and decimal-point offset
        let (digits, decimal_offset): (String, _) = match base.find('.') {
            // no decimal point found, pass directly to Decimal
            None => (base.to_string(), 0),

            // decimal point found - copy into new string buffer
            Some(loc) => {
                // split into leading and trailing digits
                let (lead, trail) = (&base[..loc], &base[loc + 1..]);

                // copy all leading characters into 'digits' string
                let mut digits = String::from(lead);

                // copy all trailing characters after '.' into the digits string
                digits.push_str(trail);

                (digits, trail.len() as i64)
            }
        };

        let scale = (decimal_offset - exp).abs() as u8;

        if exp.is_positive() {
            Ok(Decimal::new(
                Decimal::from_str(base.as_str())
                    .expect("decimal of base")
                    .to_scale(exp.abs() as u8)
                    .value,
                0,
                negative,
            )
            .to_scale(exp.abs() as u8))
        } else {
            Ok(Decimal::new(
                u128::from_str_radix(&digits, radix).unwrap(),
                scale,
                negative,
            ))
        }
    }
}

impl From<Decimal> for u64 {
    fn from(decimal: Decimal) -> u64 {
        u64::from_u128(decimal.value).unwrap_or_else(|| {
            panic!("decimal: overflow in From<Decimal> for u64, value does not fit")
        })
    }
}

impl From<Decimal> for u128 {
    fn from(decimal: Decimal) -> u128 {
        decimal.value
    }
}

impl From<Decimal> for usize {
    fn from(decimal: Decimal) -> usize {
        usize::from_u128(decimal.value).unwrap_or_else(|| {
            panic!("decimal: overflow in From<Decimal> for usize, value does not fit")
        })
    }
}

impl From<Decimal> for f64 {
    fn from(decimal: Decimal) -> f64 {
        let numerator = f64::from_u128(decimal.value).unwrap_or_else(|| {
            panic!("decimal: overflow in From<Decimal> for f64, numerator does not fit")
        });
        let denominator = f64::from_u128(decimal.denominator()).unwrap_or_else(|| {
            panic!("decimal: overflow in From<Decimal> for f64, denominator does not fit")
        });
        numerator / denominator
    }
}

impl From<Decimal> for i32 {
    fn from(decimal: Decimal) -> i32 {
        let sign = if decimal.negative { -1i32 } else { 1i32 };
        let value = i32::from_u128(decimal.value).unwrap_or_else(|| {
            panic!("decimal: overflow in From<Decimal> for f64, value does not fit")
        });

        value
            .checked_mul(sign)
            .unwrap_or_else(|| panic!("decimal: overflow in From<Decimal> for i32, checked_mul"))
    }
}

impl FromStr for Decimal {
    type Err = DecimalError;

    #[inline]
    fn from_str(s: &str) -> Result<Decimal, DecimalError> {
        Decimal::from_str_radix(s, 10)
    }
}

impl fmt::Display for Decimal {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let scale = self.scale as usize;
        let mut rep = self.value.to_string();
        let len = rep.len();

        // inject decimal point
        if scale > 0 {
            match scale.cmp(&len) {
                Ordering::Greater => {
                    let mut new_rep = String::new();
                    let zeros = "0".repeat(scale as usize - len);
                    new_rep.push_str("0.");
                    new_rep.push_str(&zeros[..]);
                    new_rep.push_str(&rep[..]);
                    rep = new_rep;
                }
                Ordering::Equal => {
                    rep.insert(0, '.');
                    rep.insert(0, '0');
                }
                _ => {
                    rep.insert(len - scale as usize, '.');
                }
            }
        } else if rep.is_empty() {
            // corner case for truncated decimals
            rep.insert(0, '0');
        }

        f.pad_integral(!self.negative, "", &rep)
    }
}

#[cfg(test)]
mod test {
    use crate::decimal::ops::{Add, Div, DivUp, Mul, Pow, Sqrt, Sub};
    use crate::decimal::Decimal;
    use proptest::prelude::*;
    use std::str::FromStr;

    #[test]
    fn test_basic_examples() {
        {
            // 1.000000 * 1.000000 = 1.000000
            let a = Decimal::from_scaled_amount(1_000000, 6);
            let b = Decimal::from_scaled_amount(1_000000, 6);
            let actual = a.mul(b);
            let expected = Decimal {
                value: 1_000000,
                scale: 6,
                negative: false,
            };
            assert_eq!(actual, expected)
        }
        {
            // 3/2 = 1.500000
            let a = Decimal::from_u64(3).to_scale(6);
            let b = Decimal::from_u64(2).to_scale(6);

            let actual = a.div(b);
            let expected = Decimal {
                value: 1_500000,
                scale: 6,
                negative: false,
            };

            assert_eq!({ actual.value }, { expected.value });
            assert_eq!(actual.scale, expected.scale);

            // 2/3 = 0.666667 rounded up
            let a = Decimal::from_u64(2).to_scale(6);
            let b = Decimal::from_u64(3).to_scale(6);

            let actual = a.div_up(b);
            let expected = Decimal {
                value: 666667,
                scale: 6,
                negative: false,
            };

            assert_eq!({ actual.value }, { expected.value });
            assert_eq!(actual.scale, expected.scale);

            // 2/3 = 0.666666 truncated (default)
            let a = Decimal::from_u64(2).to_scale(6);
            let b = Decimal::from_u64(3).to_scale(6);

            let actual = a.div(b);
            let expected = Decimal {
                value: 666666,
                scale: 6,
                negative: false,
            };

            assert_eq!({ actual.value }, { expected.value });
            assert_eq!(actual.scale, expected.scale);
        }
    }

    #[test]
    fn test_advanced_examples() {
        // large number multiplication
        let lhs = Decimal::from_u128(17134659154348278833);
        let rhs = Decimal::from_u128(11676758639919526015);
        let result = lhs.mul(rhs);
        let expected = Decimal::from_u128(200077279322612464128594731044417340495);
        assert_eq!(result, expected);

        let lhs = Decimal::from_u64(17134659154348278833);
        let rhs = Decimal::from_u64(11676758639919526015);
        let result = lhs.mul(rhs);
        let expected = Decimal::from_u128(200077279322612464128594731044417340495);
        assert_eq!(result, expected);

        let lhs = Decimal::from_scaled_amount(17134659154348278833, 6);
        let rhs = Decimal::from_scaled_amount(11676758639919526015, 6);
        let result = lhs.mul(rhs);
        let expected = Decimal::new(200077279322612464128594731044417, 6, false);
        assert_eq!(result, expected);

        // power function with decimal exponent, scaled down (floor) at lower precision
        // 42^1.5 = 272.191109
        let base = Decimal::new(42_000000000000, 12, false);
        let exp = Decimal::new(1500000000000, 12, false);
        let result = base.pow(exp).to_scale(6);
        let expected = Decimal {
            value: 272_191109,
            scale: 6,
            negative: false,
        };
        assert_eq!(result, expected);

        // power function with decimal exponent, scaled up (ceiling) at lower precision
        // 42^1.5 = 272.191110
        let base = Decimal::new(42_000000000000, 12, false);
        let exp = Decimal::new(1500000000000, 12, false);
        let result = base.pow(exp).to_scale_up(6);
        let expected = Decimal {
            value: 272_191110,
            scale: 6,
            negative: false,
        };
        assert_eq!(result, expected);

        // square root of 2 with accuracy scaled to 12 decimal places
        let n = Decimal::from_u64(2).to_compute_scale();
        let result = n.sqrt().unwrap();
        let expected = Decimal::new(1_414_213_562_373u128, 12, false);
        assert_eq!(result, expected);

        // square root of 2 with accuracy scaled to 8 decimal places
        let n = Decimal::from_u64(2).to_scale(8);
        let result = n.sqrt().unwrap();
        let expected = Decimal::new(141_421_356_u128, 8, false);
        assert_eq!(result, expected);

        // square root of 2 with accuracy scaled to 6 decimal places
        // with last digit rounded up
        let n = Decimal::from_u64(2).to_scale(8);
        let result = n.sqrt().unwrap().to_scale_up(6);
        let expected = Decimal::new(1_414_214u128, 6, false);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_new() {
        {
            let value = 42;
            let scale = 3;
            let actual = Decimal::new(value, scale, false);
            let expected = Decimal {
                value,
                scale,
                negative: false,
            };

            assert_eq!({ actual.value }, { expected.value });
            assert_eq!(actual.scale, expected.scale);
        }
    }

    #[test]
    fn test_denominator() {
        {
            let decimal = Decimal::new(42, 2, false);
            let actual = decimal.denominator();
            let expected = 10u128.pow(2);
            assert_eq!(actual, expected);
        }

        {
            let decimal = Decimal::new(42, 0, false);
            let actual = decimal.denominator();
            let expected = 1;
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_from_integer() {
        let integer: u64 = 42;
        let actual = Decimal::from_u64(integer);
        let expected = Decimal {
            value: 42,
            scale: 0,
            negative: false,
        };

        assert_eq!({ actual.value }, { expected.value });
        assert_eq!(actual.scale, expected.scale);
    }

    #[test]
    fn test_from_scaled_integer() {
        let integer: u64 = 42_000000;
        let scale = 6;
        let actual = Decimal::from_scaled_amount(integer, scale);
        let expected = Decimal {
            value: 42_000000,
            scale: 6,
            negative: false,
        };

        assert_eq!({ actual.value }, { expected.value });
        assert_eq!(actual.scale, expected.scale);
    }

    #[test]
    fn test_to_u64() {
        let decimal = Decimal::new(69420, 6, false);
        let actual: u64 = decimal.into();
        let expected: u64 = 69420;

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_abs() {
        let decimal = Decimal::new(0, 0, false);
        assert_eq!(decimal.abs(), 0);

        let decimal = Decimal::new(42, 0, false);
        assert_eq!(decimal.abs(), 42);

        let decimal = Decimal::new(4269420, 5, false);
        assert_eq!(decimal.abs(), 42);

        let decimal = Decimal::new(4269420, 5, true);
        assert_eq!(decimal.abs(), 42);

        let decimal = Decimal::new(4269420, 5, false);
        assert_eq!(decimal.abs_up(), 43);

        let decimal = Decimal::new(4269420, 5, true);
        assert_eq!(decimal.abs_up(), 43);
    }

    #[test]
    fn test_to_scale() {
        // increase precision
        {
            let decimal = Decimal::new(42, 2, false);
            let result = decimal.to_scale(3);

            assert_eq!(result.scale, 3);
            assert_eq!({ result.value }, 420);
        }
        // decrease precision
        {
            let decimal = Decimal::new(42, 2, false);
            let result = decimal.to_scale(1);

            assert_eq!(result.scale, 1);
            assert_eq!({ result.value }, 4);
        }
        // decrease precision past value
        {
            let decimal = Decimal::new(123, 4, false);
            let result = decimal.to_scale(0);

            assert_eq!(result.scale, 0);
            assert_eq!({ result.value }, 0);
        }
    }

    #[test]
    fn test_to_scale_up() {
        // increase precision
        {
            let decimal = Decimal::new(42, 2, false);
            let result = decimal.to_scale_up(3);

            assert_eq!(result.scale, 3);
            assert_eq!({ result.value }, 420);
        }
        // decrease precision
        {
            let decimal = Decimal::new(42, 2, false);
            let result = decimal.to_scale_up(1);

            assert_eq!(result.scale, 1);
            assert_eq!({ result.value }, 5);
        }
        // decrease precision past value
        {
            let decimal = Decimal::new(123, 4, false);
            let result = decimal.to_scale_up(0);

            assert_eq!(result.scale, 0);
            assert_eq!({ result.value }, 1);
        }
    }

    #[test]
    fn test_into_u64() {
        {
            let decimal = Decimal::new(333333333333333, 15, false);
            let actual: u64 = decimal.into();
            let expected: u64 = 333333333333333;

            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_into_string() {
        {
            let decimal = Decimal::from_u128(42);
            assert_eq!(decimal.to_string(), "42");
        }

        {
            let decimal = Decimal::new(42, 0, true);
            assert_eq!(decimal.to_string(), "-42");
        }

        {
            let decimal = Decimal::from_scaled_amount(0, 6);
            assert_eq!(decimal.to_string(), "0.000000");
        }

        {
            let decimal = Decimal::from_scaled_amount(1_500_000, 6);
            assert_eq!(decimal.to_string(), "1.500000");
        }

        {
            let decimal = Decimal::from_scaled_amount(500_000, 6);
            assert_eq!(decimal.to_string(), "0.500000");
        }

        {
            let decimal = Decimal::new(500_000, 6, true);
            assert_eq!(decimal.to_string(), "-0.500000");
        }
    }

    #[test]
    fn test_from_string() {
        {
            let actual = Decimal::from_str("1e6").unwrap();
            let expected = Decimal::new(1_000_000_000_000, 6, false);
            assert_eq!(actual, expected);
        }

        {
            let actual = Decimal::from_str("1.5e6").unwrap();
            let expected = Decimal::new(1_500_000_000_000, 6, false);
            assert_eq!(actual, expected);
        }

        {
            let actual = Decimal::from_str("-1.5e6").unwrap();
            let expected = Decimal::new(1_500_000_000_000, 6, true);
            assert_eq!(actual, expected);
        }

        {
            let actual = Decimal::from_str("1.5e9").unwrap();
            let expected = Decimal::new(1_500_000_000_000_000_000, 9, false);
            assert_eq!(actual, expected);
        }

        {
            let actual = Decimal::from_str("42").unwrap();
            let expected = Decimal::new(42, 0, false);
            assert_eq!(actual, expected);
        }

        {
            let actual = Decimal::from_str("-42").unwrap();
            let expected = Decimal::new(42, 0, true);
            assert_eq!(actual, expected);
        }

        {
            let actual = Decimal::from_str("1.5").unwrap();
            let expected = Decimal::new(15, 1, false);
            assert_eq!(actual, expected);
        }

        {
            let actual = Decimal::from_str("-1.5").unwrap();
            let expected = Decimal::new(15, 1, true);
            assert_eq!(actual, expected);
        }

        {
            let actual = Decimal::from_str("42.500000").unwrap();
            let expected = Decimal::new(42_500_000, 6, false);
            assert_eq!(actual, expected);
        }

        {
            let actual = Decimal::from_str("42.500420").unwrap();
            let expected = Decimal::new(42_500_420, 6, false);
            assert_eq!(actual, expected);
        }
    }

    #[test]
    #[should_panic]
    #[allow(unused_variables)]
    fn test_into_u64_panic() {
        let decimal = Decimal::new(u128::MAX - 1, 15, false);
        let result: u64 = decimal.into();
    }

    #[test]
    fn test_sign() {
        // is zero
        {
            let decimal = Decimal::new(0, 0, false);
            assert!(decimal.is_zero());
        }

        // is neither positive or negative i.e. zero
        {
            let decimal = Decimal::new(0, 0, false);
            assert!(!decimal.is_positive());

            let decimal = Decimal::new(0, 0, false);
            assert!(!decimal.is_negative());
        }

        // is positive
        {
            let decimal = Decimal::new(42, 4, false);
            assert!(decimal.is_positive());

            let decimal = Decimal::new(24, 4, false);
            assert!(decimal.is_positive());
        }

        // is negative
        {
            let decimal = Decimal::new(42, 4, true);
            assert!(decimal.is_negative());

            let decimal = Decimal::new(24, 4, true);
            assert!(decimal.is_negative());
        }
    }

    #[test]
    fn test_is_integer() {
        // when scale is zero
        {
            let decimal = Decimal::new(0, 0, false);
            assert!(decimal.is_integer());

            let decimal = Decimal::new(42, 0, false);
            assert!(decimal.is_integer());

            let decimal = Decimal::new(42, 0, true);
            assert!(decimal.is_integer());
        }

        // when scale is not zero
        {
            let decimal = Decimal::new(42420, 3, false);
            assert!(!decimal.is_integer());

            let decimal = Decimal::new(42420, 3, true);
            assert!(!decimal.is_integer());
        }
    }

    proptest! {
        #[test]
        fn test_full_u64_range(
            lhs in 1_000_000..u64::MAX, // 1.000000 .. 18,446,744,073,709.551615
            rhs in 1_000_000..u64::MAX,
        ) {
            let scale = 6; // decimal places
            let lhs_decimal = Decimal::from_scaled_amount(lhs, scale);
            let rhs_decimal = Decimal::from_scaled_amount(rhs, scale);

            // basic math both sides
            {
                lhs_decimal.mul(rhs_decimal);
                lhs_decimal.div(rhs_decimal);
                lhs_decimal.add(rhs_decimal).unwrap();
                lhs_decimal.sub(rhs_decimal).unwrap();
            }

            // basic math one side
            {
                lhs_decimal.mul(lhs_decimal);
                lhs_decimal.div(lhs_decimal);
                lhs_decimal.add(lhs_decimal).unwrap();
                lhs_decimal.sub(lhs_decimal).unwrap();
            }
        }
    }
}
