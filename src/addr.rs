#![allow(dead_code)]

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct PAddr(pub usize);

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct VAddr(pub usize);

impl PAddr {
    #[inline] pub const fn align_up(self, align: usize) -> Self {
        debug_assert!(align.is_power_of_two());
        PAddr((self.0 + align - 1) & !(align - 1))
    }
    #[inline] pub const fn is_aligned(self, align: usize) -> bool {
        debug_assert!(align.is_power_of_two());
        (self.0 & (align - 1)) == 0
    }
}

impl VAddr {
    #[inline] pub const fn align_up(self, align: usize) -> Self {
        debug_assert!(align.is_power_of_two());
        VAddr((self.0 + align - 1) & !(align - 1))
    }
    #[inline] pub const fn is_aligned(self, align: usize) -> bool {
        debug_assert!(align.is_power_of_two());
        (self.0 & (align - 1)) == 0
    }
}
