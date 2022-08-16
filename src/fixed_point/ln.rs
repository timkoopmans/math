use crate::fixed_point::FixedPoint;
use checked_decimal_macro::*;
use std::ops::Div;

impl FixedPoint {
    pub fn ln(self) -> Option<(FixedPoint, bool)> {
        let (log2_x, negative) = self.log2()?;

        // 1.4426950408889634073599246810018921374266459541529859341354494069
        let log2_e = FixedPoint::new(1_442695040888u128);

        // ln(x) = log2(x) / log2(e)
        Some((log2_x.div(log2_e), negative))
    }
}

#[cfg(test)]
mod tests {
    use crate::fixed_point::FixedPoint;
    use checked_decimal_macro::*;
    use proptest::prelude::*;

    #[test]
    fn test_ln() {
        //  with integer and fractional digits
        // ln(2.25) = 0.8109302162163287639560262309286982731439808469249883952280
        {
            let decimal = FixedPoint::new(2250000000000u128);
            let actual = decimal.ln();
            let expected = Some((FixedPoint::new(810930216211u128), false));
            assert_eq!(actual, expected);
        }

        //  with fractional digits only
        // ln(0.810930216211) = -0.209573275164505847614143429005277100396934915004957131195
        {
            let decimal = FixedPoint::new(810930216211u128);
            let actual = decimal.ln();
            let expected = Some((FixedPoint::new(209573275158u128), true));
            assert_eq!(actual, expected);
        }

        // with very small fractional digits only
        // ln(0.000000000001) = -27.63102111592854820821589745621237049121321786354527571239
        {
            let decimal = FixedPoint::new(1u128);
            let actual = decimal.ln();
            let expected = Some((FixedPoint::new(27_631021115941u128), true));
            assert_eq!(actual, expected);
        }
    }

    proptest! {
        #[test]
        fn test_full_u64_range_ln(
            x in 1..u64::MAX, // 0.000000001 .. 18,446,744,073.709551615
        ) {
            let scale: f64 = 9.0; // decimal places
            let precision = 2; // accuracy +/- 0.000001
            let x_decimal = FixedPoint::from_integer(x as u128);
            let x_f64: f64 = x_decimal.get() as f64;
            let den_f64: f64 = 10f64.powf(scale);

            {
                let ln_f64 = x_f64.ln();
                let ln_f64_negative = ln_f64.is_sign_negative();
                let ln_f64_u128 = (((ln_f64 * den_f64).round() / den_f64) * den_f64) as u128;
                let (ln_decimal, ln_decimal_negative) = x_decimal.ln().unwrap();
                let ln_decimal_u128 = ln_decimal.get();
                let difference = ln_f64_u128.saturating_sub(ln_decimal_u128).lt(&precision);

                assert_eq!(ln_decimal_negative, ln_f64_negative);
                assert!(difference, "ln compare\n{}\n{}\n{}", ln_f64_u128, ln_decimal_u128, x_decimal);
            }
        }
    }
}
