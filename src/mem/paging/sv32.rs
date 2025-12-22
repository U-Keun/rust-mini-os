#![allow(dead_code)]

use crate::mem::PAGE_SIZE;

pub const SATP_SV32: usize = 1 << 31;

pub const PTE_V: u32 = 1 << 0; // valid
pub const PTE_R: u32 = 1 << 1; // readable
pub const PTE_W: u32 = 1 << 2; // writable
pub const PTE_X: u32 = 1 << 3; // executable
pub const PTE_U: u32 = 1 << 4; // user

#[inline]
pub const fn is_aligned(x: usize, align: usize) -> bool { (x & (align - 1)) == 0 }

#[inline]
pub const fn vpn1_of(va: usize) -> usize { (va >> 22) & 0x3ff }
#[inline]
pub const fn vpn0_of(va: usize) -> usize { (va >> 12) & 0x3ff }
#[inline]
pub const fn ppn_of(pa: usize) -> u32 { (pa >> 12) as u32 }

pub unsafe fn map_page(l1_phys: usize, vaddr: usize, paddr: usize, flags: u32) {
    assert!(is_aligned(vaddr, PAGE_SIZE), "unaligned vaddr {:x}", vaddr);
    assert!(is_aligned(paddr, PAGE_SIZE), "unaligned paddr {:x}", paddr);

    let l1 = l1_phys as *mut u32;
    let i1 = vpn1_of(vaddr);

    let mut l1e = core::ptr::read_volatile(l1.add(i1));
    if (l1e & PTE_V) == 0 {
        let l0_phys = crate::mem::frame_alloc::alloc_pages(1).unwrap().paddr();
        unsafe {
            core::ptr::write_bytes(l0_phys as *mut u8, 0, PAGE_SIZE);
        }
        l1e = ((ppn_of(l0_phys) as u32) << 10) | PTE_V;
        core::ptr::write_volatile(l1.add(i1), l1e);
    }

    let l0_phys = ((l1e >> 10) as usize) * PAGE_SIZE;
    let l0 = l0_phys as *mut u32;
    let i0 = vpn0_of(vaddr);

    let pte = ((ppn_of(paddr) as u32) << 10) | flags | PTE_V;
    core::ptr::write_volatile(l0.add(i0), pte);
}
