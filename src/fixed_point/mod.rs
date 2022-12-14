use checked_decimal_macro::*;
use checked_decimal_macro::U256;

pub mod ln;
pub mod square;
pub mod msb;
pub mod log2;
pub mod log10;
pub mod ln_tables;

#[decimal(12)]
#[derive(Default, PartialEq, Debug, Clone, Copy)]
pub struct FixedPoint(u128, U256);

#[decimal(0)]
#[derive(Default, PartialEq, Debug, Clone, Copy)]
pub struct Integer(u128, U256);
