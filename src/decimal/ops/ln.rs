use crate::decimal::ops::{Div, Log2};
use crate::decimal::{Decimal, COMPUTE_SCALE};
use crate::decimal::errors::ErrorCode;

pub trait Ln<T>: Sized {
    fn ln(self) -> Result<Self, ErrorCode>;
}

impl Ln<Decimal> for Decimal {
    fn ln(self) -> Result<Self, ErrorCode> {
        let log2_x = self.to_compute_scale().log2().expect("log_2");

        // 1.4426950408889634073599246810018921374266459541529859341354494069
        let log2_e = Decimal::new(1_442695040888u128, COMPUTE_SCALE, false);

        // ln(x) = log2(x) / log2(e)
        Ok(log2_x.div(log2_e).to_scale(self.scale))
    }
}

#[cfg(test)]
mod tests {
    use crate::decimal::ops::Ln;
    use crate::decimal::Decimal;
    use proptest::prelude::*;

    #[test]
    fn test_log_e() {
        //  with integer and fractional digits
        // ln(2.25) = 0.8109302162163287639560262309286982731439808469249883952280
        {
            let decimal = Decimal::new(2250000000000, 12, false);
            let actual = decimal.ln().unwrap();
            let expected = Decimal::new(810930216211, 12, false);
            assert_eq!(actual, expected);
        }

        //  with fractional digits only
        // ln(0.810930216211) = -0.209573275164505847614143429005277100396934915004957131195
        {
            let decimal = Decimal::new(810930216211u128, 12, false);
            let actual = decimal.ln().unwrap();
            let expected = Decimal::new(209573275158u128, 12, true);
            assert_eq!(actual, expected);
        }

        // with very small fractional digits only
        // ln(0.000000000001) = -27.63102111592854820821589745621237049121321786354527571239
        {
            let decimal = Decimal::new(1u128, 12, false);
            let actual = decimal.ln().unwrap();
            let expected = Decimal::new(27_631021115941u128, 12, true);
            assert_eq!(actual, expected);
        }

        // ln(.93859063) = -0.06337585862
        {
            let decimal = Decimal::new(93859063, 8, false);
            let actual = decimal.ln().unwrap();
            let expected = Decimal::new(6337585, 8, true);
            assert_eq!(actual, expected);
        }

        // ln(0.9) = -0.105360
        {
            let decimal = Decimal::new(900000u128, 6, false);
            let actual = decimal.ln().unwrap();
            let expected = Decimal::new(105360u128, 6, true);
            assert_eq!(actual, expected);
        }

        // ln(0.9) = -0.105360515652
        {
            let decimal = Decimal::new(900_000_000_000u128, 12, false);
            let actual = decimal.ln().unwrap();
            let expected = Decimal::new(105360515652u128, 12, true);
            assert_eq!(actual, expected);
        }

        // ln(0.1) = -2.302585092990
        {
            let decimal = Decimal::new(100000000000u128, 12, false);
            let actual = decimal.ln().unwrap();
            let expected = Decimal::new(2302585092990u128, 12, true);
            assert_eq!(actual, expected);
        }

        // ln(10) = 2.302585092990
        {
            let decimal = Decimal::new(10_000000000000, 12, false);
            let actual = decimal.ln().unwrap();
            let expected = Decimal::new(2302585092990u128, 12, false);
            assert_eq!(actual, expected);
        }
    }

    proptest! {
        #[test]
        fn test_full_u64_range_ln(
            lhs in 1..u64::MAX, // 1.000000 .. 18,446,744,073,709.551615
        ) {
            let scale = 9; // decimal places
            let precision = 2; // accuracy +/- 0.000001
            let lhs_decimal = Decimal::new(lhs as u128, scale, false);
            let lhs_f64: f64 = lhs_decimal.into();
            let den_f64: f64 = lhs_decimal.denominator() as f64;

            // f64 ln == Decimal ln
            {
                let ln_f64_u128 = (((lhs_f64.ln() * den_f64).round() / den_f64) * den_f64) as u128;
                let ln_decimal_u128 = lhs_decimal.to_scale(scale).ln().unwrap().value;
                let difference = ln_f64_u128.saturating_sub(ln_decimal_u128).lt(&precision);

                assert!(difference, "ln compare\n{}\n{}\n{}", ln_f64_u128, ln_decimal_u128, lhs_decimal);
            }
        }
    }
}
