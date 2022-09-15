use crate::decimal::{Decimal, COMPUTE_SCALE};
use crate::decimal::errors::ErrorCode;

pub trait Log2<T>: Sized {
    fn log2(self) -> Result<Self, ErrorCode>;
}

impl Log2<Decimal> for Decimal {
    fn log2(self) -> Result<Self, ErrorCode> {
        let mut x: u128 = self.value;

        assert!(x > 0, "must be greater than zero");

        let scale: u128 = 10u128.checked_pow(self.scale as u32).expect("scale");

        let negative = x < scale;

        // log2(x) = -log2(1/x)
        if negative {
            x = scale
                .checked_mul(scale)
                .expect("mul")
                .checked_div(x)
                .expect("div");
        }

        // integer part of the logarithm is most significant bit n
        let integer_part = x.checked_div(scale).expect("div");
        let leading_zeros = integer_part.leading_zeros() as u128;
        let n = 128u128 - leading_zeros - 1u128;

        let mut result = n.checked_mul(scale).expect("mul");

        let mut y = x >> n;

        // if y = 1, then the algorithm is done, and the fractional part is zero
        if y == scale {
            return Ok(Decimal::new(result, COMPUTE_SCALE, negative).to_scale(self.scale));
        }

        // calculate fractional part via iterative approximation.
        // https://en.wikipedia.org/wiki/Binary_logarithm#Iterative_approximation
        let mut z = scale >> 1;

        while z.gt(&0u128) {
            // y = y^2 / scale;
            y = (y.checked_mul(y).expect("checked_mul"))
                .checked_div(scale)
                .expect("checked_div");

            // if y^2 >= 2
            if y >= 2u128.checked_mul(scale).expect("checked_mul") {
                // result += 2^(-z)
                result = result.checked_add(z).expect("checked_add");
                y >>= 1;
            }

            // z /= 2
            z >>= 1;
        }

        Ok(Decimal::new(result, COMPUTE_SCALE, negative).to_scale(self.scale))
    }
}

#[cfg(test)]
mod tests {
    use crate::decimal::ops::Log2;
    use crate::decimal::Decimal;

    #[test]
    fn test_log2() {
        // log2(2.25) = 1.1699250014423123629074778878956330175196288153849621209115
        {
            let decimal = Decimal::new(2250000000000, 12, false); // 2.25
            let actual = decimal.log2().unwrap();
            let expected = Decimal::new(1_169925001434, 12, false);
            assert_eq!(actual, expected);
        }

        // log2(18446744.073709551615) = 24.1368628613516518255
        {
            let decimal = Decimal::new(18446744073709551615, 12, false); // 2.25
            let actual = decimal.log2().unwrap();
            let expected = Decimal::new(24_136862861344, 12, false);
            assert_eq!(actual, expected);
        }
    }
}
