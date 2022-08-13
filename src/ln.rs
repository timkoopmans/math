use crate::decimal::FixedPoint;
use checked_decimal_macro::BigOps;

impl FixedPoint {
    pub fn ln(self) -> Self {
        self.big_mul(self)
    }
}
