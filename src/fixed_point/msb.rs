use crate::fixed_point::FixedPoint;
use checked_decimal_macro::*;

impl FixedPoint {
    pub fn msb(self) -> u32 {
        self.get().leading_zeros() - 1
    }

    pub fn msb_shift(self) -> u32 {
        let mut x = self.get();
        assert!(x > 0);

        let mut r = 0u32;

        // if x >= 1u128 << 32 {
        //     x >>= 128;
        //     r += 128;
        // }
        if x >= 1u128 << 16 {
            x >>= 64;
            r += 64;
        }
        if x >= 1u128 << 8 {
            x >>= 32;
            r += 32;
        }
        if x >= 1u128 << 4 {
            x >>= 16;
            r += 16;
        }
        if x >= 1u128 << 2 {
            x >>= 8;
            r += 8;
        }
        if x >= 1u128 << 1 {
            x >>= 4;
            r += 4;
        }
        if x >= 4 {
            x >>= 2;
            r += 2;
        }
        if x >= 2{
            r += 1;
        }

        r - 1
    }
}
#[cfg(test)]
mod tests {
    use crate::fixed_point::FixedPoint;
    use checked_decimal_macro::*;

    #[test]
    fn test_msb() {

        let decimal = FixedPoint::new(u64::MAX.into());

        let actual_shift = decimal.msb_shift();

        let actual = decimal.msb();
        let expected = 128u32 - 64u32 - 1u32;

        assert_eq!(actual, expected);
        assert_eq!(actual_shift, expected);
    }
}