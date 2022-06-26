use std::marker::PhantomData;
use std::ops::Range;

use crate::genericity::Id;

use super::CircuitBuilder;

#[doc(hidden)]
pub trait CircuitSymbolPrivate<'id>: Sized {
    fn new(n: u32) -> Self;

    #[inline]
    fn list(range: Range<u32>) -> List<Self> {
        List { range, _phantom: PhantomData }
    }

    fn count<'a>(circ: &'a mut CircuitBuilder) -> &'a mut u32;
}

pub trait CircuitSymbol<'id>: CircuitSymbolPrivate<'id> {
    fn id(self) -> u32;
}

macro_rules! circuit_symbol_impl {
    { $(#[doc$($args :tt)*])* $name: ident $count: ident} => {
        $(#[doc$($args)*])*
        #[repr(transparent)]
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
        pub struct $name<'id> {
            n: u32,
            id: Id<'id>,
        }

        impl<'id> CircuitSymbolPrivate<'id> for $name<'id> {
            #[inline]
            fn new(n: u32) -> Self {
                Self { n, id: Id::default() }
            }

            #[inline]
            fn count<'b>(circ: &'b mut CircuitBuilder) -> &'b mut u32 {
                &mut circ.$count
            }
        }

        impl<'id> CircuitSymbol<'id> for $name<'id> {
            #[inline]
            fn id(self) -> u32 {
                self.n
            }
        }
    }
}

circuit_symbol_impl! {
    /// TODO: Doc
    Qubit 
    qubit_count
}

circuit_symbol_impl! {
    /// TODO: Doc
    FormalParameter 
    parameter_count
}

circuit_symbol_impl! {
    /// TODO: Doc
    Bit 
    bit_count
}

pub struct List<T> {
    range: Range<u32>,
    _phantom: PhantomData<T>,
}

impl<'id, T: CircuitSymbol<'id> + 'id> List<T> {
    #[inline]
    pub fn range(&self) -> Range<u32> {
        self.range.clone()
    }

    #[inline]
    pub fn len(&self) -> usize {
        (self.range.end - self.range.start) as usize
    }

    #[inline]
    pub fn get(&self, id: usize) -> Option<T> {
        (id < self.len()).then(|| T::new(id as u32 + self.range.start))
    }

    #[inline]
    pub fn contains(&self, parameter: T) -> bool {
        self.range.contains(&parameter.id())
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = T> + 'id {
        self.range.clone().map(T::new)
    }
}