use crate::decimal::FixedPoint;
use checked_decimal_macro::*;

impl FixedPoint {
    pub fn ln(self) -> Option<u128> {
        let (log2_x, _negative) = self.log2()?;
        let scale: u128 = 10u128.checked_pow(FixedPoint::scale() as u32)?;

        // 1.4426950408889634073599246810018921374266459541529859341354494069
        let log2_e = 1_442695040888u128;

        // ln(x) = log2(x) / log2(e)
        log2_x.checked_mul(scale)?.checked_div(log2_e)
    }
}

#[cfg(test)]
mod tests {
    use crate::decimal::FixedPoint;
    use checked_decimal_macro::*;

    #[test]
    fn test_ln() {
        let decimal = FixedPoint::new(2250000000000);

        let actual = decimal.ln();

        // 0.8109302162163287639560262309286982731439808469249883952280
        let expected = Some(810930216211);
        assert_eq!(actual, expected);
    }
}
