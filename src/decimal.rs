use checked_decimal_macro::*;

#[decimal(12)]
#[derive(Default, PartialEq, Debug, Clone, Copy)]
pub(crate) struct FixedPoint(u128);
