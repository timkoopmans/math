use crate::decimal::ops::{Div, Log2};
use crate::decimal::{Decimal, COMPUTE_SCALE};
use crate::decimal::errors::ErrorCode;

pub trait Log10<T>: Sized {
    fn log10(self) -> Result<Self, ErrorCode>;
}

impl Log10<Decimal> for Decimal {
    fn log10(self) -> Result<Self, ErrorCode> {
        let scale = self.scale;
        let x = self.to_compute_scale();
        let x_scale = 10u128.checked_pow(COMPUTE_SCALE as u32).expect("scale");
        let negative = x.value < x_scale;

        if x.eq(&Decimal::one()) {
            return Ok(Decimal::zero().to_scale(scale));
        }

        let power_of_ten: u128 = match x.value {
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
            let value = power_of_ten.checked_mul(x_scale).expect("value");
            Ok(Decimal::new(value, x.scale, negative).to_scale(scale))
        } else {
            // log2(10) = 3.3219280948873623478703194294893901758648313930245806120547563958...
            let log2_10 = Decimal::new(3_321928094887u128, COMPUTE_SCALE, false);

            let log2_x = self.log2().expect("log2_x");
            // log2(x) / log2(10)
            Ok(log2_x.div(log2_10).to_scale(self.scale))
        }
    }
}

#[cfg(test)]
mod test {
    use crate::decimal::ops::Log10;
    use crate::decimal::Decimal;

    #[test]
    fn test_log10() {
        // log10(1) = 0
        {
            let actual = Decimal::new(1_00000000, 8, false).log10().unwrap();
            let expected = Decimal::new(0, 8, false);
            assert_eq!(actual, expected);
        }

        // log10(10) = 1
        {
            let actual = Decimal::new(10_00000000, 8, false).log10().unwrap();
            let expected = Decimal::new(1_00000000, 8, false);
            assert_eq!(actual, expected);
        }

        // log10(1000) = 3
        {
            let actual = Decimal::new(1000_00000000, 8, false).log10().unwrap();
            let expected = Decimal::new(3_00000000, 8, false);
            assert_eq!(actual, expected);
        }

        // log10(1.1) = 0.0413926851582250407501999712430242417067021904664530945965390186...
        {
            let actual = Decimal::new(1_100000000000, 12, false).log10().unwrap();
            let expected = Decimal::new(41392685156, 12, false);
            assert_eq!(actual, expected);
        }

        // log10(18446744.073709551615) = 7.26591972249479649366...
        {
            let actual = Decimal::new(u64::MAX as u128, 12, false).log10().unwrap();
            let expected = Decimal::new(7_265919722493, 12, false);
            assert_eq!(actual, expected);
        }
    }
}
