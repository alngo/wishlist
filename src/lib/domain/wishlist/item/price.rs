use core::f32;

use rust_decimal::{prelude::FromPrimitive, Decimal};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemPrice(rust_decimal::Decimal);

impl From<f32> for ItemPrice {
    fn from(value: f32) -> Self {
        Self(Decimal::from_f32(value).unwrap())
    }
}
