use crate::decimal::FixedPoint;
use checked_decimal_macro::*;

impl FixedPoint {
    pub fn log2(self) -> Option<(u128, bool)> {
        let mut x: u128 = self.get();

        assert!(x > 0, "must be greater than zero");

        let scale: u128 = 10u128.checked_pow(FixedPoint::scale() as u32)?;

        let negative = x < scale;

        // log2(x) = -log2(1/x)
        if negative {
            x = scale.checked_mul(scale)?.checked_div(x)?;
        }

        // integer part of the logarithm is simply n
        let n = (128u32 - x.checked_div(scale)?.leading_zeros()) as u128 - 1u128;

        let mut result = n.checked_mul(scale)?;

        let mut y = x >> n;

        // if y = 1, then the algorithm is done, and the fractional part is zero
        if y == scale {
            return Some((result, negative))
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

        Some((result, negative))
    }
}

#[cfg(test)]
mod tests {
    use crate::decimal::FixedPoint;
    use checked_decimal_macro::*;

    #[test]
    fn test_log2() {
        let decimal = FixedPoint::new(2250000000000);

        let actual = decimal.log2();

        // 1.1699250014423123629074778878956330175196288153849621209115
        let expected = Some((1_169925001434, false));
        assert_eq!(actual, expected);
    }

}