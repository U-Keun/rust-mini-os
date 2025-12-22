#![allow(dead_code)]

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct PAddr(pub(crate) usize);

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct VAddr(pub(crate) usize);

pub trait Addr: Copy + Eq + Ord {
    fn raw(self) -> usize;
    fn from_raw(x: usize) -> Self;

    #[inline]
    fn align_up(self, align: usize) -> Self {
        debug_assert!(align.is_power_of_two());
        let x = self.raw();
        let y = (x + align - 1) & !(align - 1);
        Self::from_raw(y)
    }

    #[inline]
    fn is_aligned(self, align: usize) -> bool {
        debug_assert!(align.is_power_of_two());
        (self.raw() & (align - 1)) == 0
    }
}

impl Addr for PAddr {
    #[inline] fn raw(self) -> usize { self.0 }
    #[inline] fn from_raw(x: usize) -> Self { PAddr(x) }
}

impl Addr for VAddr {
    #[inline] fn raw(self) -> usize { self.0 }
    #[inline] fn from_raw(x: usize) -> Self { VAddr(x) }
}
