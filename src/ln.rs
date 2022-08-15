use crate::decimal::FixedPoint;
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
    use crate::decimal::FixedPoint;
    use checked_decimal_macro::*;

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
}
