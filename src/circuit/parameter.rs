use std::mem;

use crate::genericity::Id;

use super::storage;
use super::symbol::{FormalParameter, Symbol};
use super::symbol::private::SymbolPrivate;

#[repr(transparent)]
#[derive(Clone, Copy, Eq, Debug)]
pub struct Parameter<'id> {
    _id: Id<'id>,
    bits: u64,
}

impl<'id> Parameter<'id> {
    #[inline]
    fn new(bits: u64) -> Self {
        Self { bits, _id: Id::default() }
    }

    #[inline]
    pub fn is_value(self) -> bool {
        f64::from_bits(self.bits).is_finite()
    }

    #[inline]
    pub fn is_formal(self) -> bool {
        !self.is_value()
    }

    #[inline]
    pub fn as_value(self) -> Option<f64> {
        self.is_value().then(|| f64::from_bits(self.bits))
    }

    #[inline]
    pub fn as_formal(self) -> Option<FormalParameter<'id>> {
        self.is_formal().then(|| FormalParameter::new((self.bits & 0xFFFFFFFF) as u32))
    }
}

impl<'id> From<f64> for Parameter<'id> {
    #[inline]
    fn from(mut value: f64) -> Self {
        if !value.is_normal() {
            value = 0.0;
        }

        Self::new(value.to_bits())
    }
}

impl<'id> From<FormalParameter<'id>> for Parameter<'id> {
    #[inline]
    fn from(formal: FormalParameter<'id>) -> Self {
        Self::new(u64::from(formal.id()) | f64::INFINITY.to_bits())
    }
}

impl PartialEq for Parameter<'_> {
    #[inline]
    fn eq(&self, rhs: &Self) -> bool {
        match (self.as_value(), rhs.as_value()) {
            (Some(x), Some(y)) => (x - y).abs() < 1e-10,
            (None, None) => self.bits == rhs.bits,
            _ => false,
        }
    }
}