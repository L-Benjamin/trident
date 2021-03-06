use crate::genericity::Id;

use super::symbol::{FormalParameter, Symbol};
use super::symbol::private::SymbolPrivate;

#[repr(transparent)]
#[derive(Clone, Copy, Eq, Debug)]
pub struct Parameter<'id> {
    _id: Id<'id>,
    bits: u32,
}

impl<'id> Parameter<'id> {
    /// The precision with which to compare two parameters. Quantum computers
    /// can't reach this level of precision nowadays.
    pub const PRECISION: f32 = 1e-4;

    /// Returns a new `Parameter` from it's bits.
    #[inline]
    fn new(bits: u32) -> Self {
        Self { bits, _id: Id::default() }
    }

    #[inline]
    pub fn is_value(self) -> bool {
        f32::from_bits(self.bits).is_finite()
    }

    #[inline]
    pub fn is_formal(self) -> bool {
        !self.is_value()
    }

    #[inline]
    pub fn as_value(self) -> Option<f32> {
        self.is_value().then(|| f32::from_bits(self.bits))
    }

    #[inline]
    pub fn as_formal(self) -> Option<FormalParameter<'id>> {
        const MANTISSA_MASK: u32 = (1 << f32::MANTISSA_DIGITS) - 1;
        self.is_formal().then(|| FormalParameter::new(self.bits & MANTISSA_MASK))
    }
}

impl<'id> From<f32> for Parameter<'id> {
    #[inline]
    fn from(mut value: f32) -> Self {
        if !value.is_finite() {
            value = 0.0;
        }

        Self::new(value.to_bits())
    }
}

impl<'id> From<FormalParameter<'id>> for Parameter<'id> {
    #[inline]
    fn from(formal: FormalParameter<'id>) -> Self {
        Self::new(u32::from(formal.id()) | f32::INFINITY.to_bits())
    }
}

impl PartialEq for Parameter<'_> {
    #[inline]
    fn eq(&self, rhs: &Self) -> bool {
        match (self.as_value(), rhs.as_value()) {
            (Some(x), Some(y)) => (x - y).abs() < Self::PRECISION,
            (None, None) => self.bits == rhs.bits,
            _ => false,
        }
    }
}