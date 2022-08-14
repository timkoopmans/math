use checked_decimal_macro::*;
use checked_decimal_macro::U256;

#[decimal(12)]
#[derive(Default, PartialEq, Debug, Clone, Copy)]
pub struct FixedPoint(u128, U256);
