use crate::fixed_point::FixedPoint;
use checked_decimal_macro::*;

impl FixedPoint {
    pub fn log2(self) -> Option<(Self, bool)> {
        let mut x: u128 = self.get();

        assert!(x > 0, "must be greater than zero");

        let scale: u128 = 10u128.checked_pow(FixedPoint::scale() as u32)?;

        let negative = x < scale;

        // log2(x) = -log2(1/x)
        if negative {
            x = scale.checked_mul(scale)?.checked_div(x)?;
        }

        // integer part of the logarithm is most significant bit n
        let integer_part = x.checked_div(scale)?;
        let leading_zeros =  integer_part.leading_zeros() as u128;
        let n = 128u128 - leading_zeros - 1u128;

        let mut result = n.checked_mul(scale)?;

        let mut y = x >> n;

        // if y = 1, then the algorithm is done, and the fractional part is zero
        if y == scale {
            return Some((FixedPoint::new(result), negative))
        }

        // calculate fractional part via iterative approximation.
        // https://en.wikipedia.org/wiki/Binary_logarithm#Iterative_approximation
        let mut z = scale >> 1;

        while z.gt(&0u128) {
            // y = y^2 / scale;
            y = (y.checked_mul(y)?).checked_div(scale)?;

            // if y^2 >= 2
            if y >= 2u128.checked_mul(scale)? {
                // result += 2^(-z)
                result = result.checked_add(z)?;
                y >>= 1;
            }

            // z /= 2
            z >>= 1;
        }

        Some((FixedPoint::new(result), negative))
    }
}

#[cfg(test)]
mod tests {
    use crate::fixed_point::FixedPoint;
    use checked_decimal_macro::*;

    #[test]
    fn test_log2() {
        // log2(2.25) = 1.1699250014423123629074778878956330175196288153849621209115
        // {
        //     let decimal = FixedPoint::new(2250000000000); // 2.25
        //     let actual = decimal.log2();
        //     let expected = Some((FixedPoint::new(1_169925001434), false));
        //     assert_eq!(actual, expected);
        // }

        // log2(18446744.073709551615) = 24.1368628613516518255
        {
            let decimal = FixedPoint::new(18446744073709551615u128); // u64::MAX
            let actual = decimal.log2();
            let expected = Some((FixedPoint::new(24_136862861344u128), false));
            assert_eq!(actual, expected);
        }
    }
}