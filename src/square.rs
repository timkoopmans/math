use crate::decimal::FixedPoint;
use checked_decimal_macro::BigOps;

impl FixedPoint {
    pub fn square(self) -> Self {
        self.big_mul(self)
    }
}
