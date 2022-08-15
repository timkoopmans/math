use crate::decimal::FixedPoint;
use checked_decimal_macro::*;
use std::ops::Div;

impl FixedPoint {
    pub fn log10(self) -> Option<(FixedPoint, bool)> {
        let x: u128 = self.get().into();
        let scale: u128 = 10u128.checked_pow(FixedPoint::scale() as u32)?;

        assert!(x > 0, "must be greater than zero");

        if x == 1000000000000 {
            return Some((FixedPoint::new(0), false));
        }

        let negative = x < scale;

        let power_of_ten: u128 = match x {
            1 => 12,
            10 => 11,
            100 => 10,
            1000 => 9,
            10000 => 8,
            100000 => 7,
            1000000 => 6,
            10000000 => 5,
            100000000 => 4,
            1000000000 => 3,
            10000000000 => 2,
            100000000000 => 1,
            1000000000000 => 0,
            10000000000000 => 1,
            100000000000000 => 2,
            1000000000000000 => 3,
            10000000000000000 => 4,
            100000000000000000 => 5,
            1000000000000000000 => 6,
            10000000000000000000 => 7,
            100000000000000000000 => 8,
            1000000000000000000000 => 9,
            10000000000000000000000 => 10,
            100000000000000000000000 => 11,
            1000000000000000000000000 => 12,
            _ => 0,
        };

        if power_of_ten > 0 {
            Some((FixedPoint::new(power_of_ten.checked_mul(scale)?), negative))
        } else {
            // log2(10) = 3.3219280948873623478703194294893901758648313930245806120547563958...
            let log2_10 = FixedPoint::new(3_321928094887);
            let (log2_x, log2_negative) = self.log2()?;
            // log2(x) / log2(10)
            Some((log2_x.div(log2_10), negative && log2_negative))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::decimal::FixedPoint;
    use checked_decimal_macro::*;

    #[test]
    fn test_log10() {
        // log10(1.1) = 0.0413926851582250407501999712430242417067021904664530945965390186...
        {
            let decimal = FixedPoint::new(1_100000000000); // 1.1
            let actual = decimal.log10();
            let expected = Some((FixedPoint::new(41392685156u128), false));
            assert_eq!(actual, expected);
        }

        // log10(18446744.073709551615) = 7.26591972249479649366...
        {
            let decimal = FixedPoint::new(u64::MAX as u128); // 18446744073709551615
            let actual = decimal.log10();
            let expected = Some((FixedPoint::new(7_265919722493u128), false));
            assert_eq!(actual, expected);
        }
    }
}