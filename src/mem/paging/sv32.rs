#![allow(dead_code)]

use crate::mem::PAGE_SIZE;
use crate::mem::frame_alloc::{ self, Oom };
use crate::mem::addr::{ PAddr, VAddr };

impl PAddr {
    #[inline] pub const fn ppn(self) -> u32 { (self.0 >> 12) as u32 }
}

impl VAddr {
    #[inline] pub const fn vpn1(self) -> usize { (self.0 >> 22) & 0x3ff }
    #[inline] pub const fn vpn0(self) -> usize { (self.0 >> 12) & 0x3ff }
    #[inline] pub const fn page_offset(self) -> usize { self.0 & (PAGE_SIZE - 1) }
}

pub const SATP_SV32: usize = 1 << 31;

#[inline]
pub const fn satp_from_root(root_paddr: PAddr) -> usize {
    SATP_SV32 | (root_paddr.raw() / PAGE_SIZE)
}

pub fn alloc_root_table() -> Result<PAddr, Oom> {
    let frame = frame_alloc::alloc_pages(1)?;
    unsafe { frame.fill(1, 0); }
    Ok(frame.paddr())
}

pub const PTE_V: u32 = 1 << 0; // valid
pub const PTE_R: u32 = 1 << 1; // readable
pub const PTE_W: u32 = 1 << 2; // writable
pub const PTE_X: u32 = 1 << 3; // executable
pub const PTE_U: u32 = 1 << 4; // user

#[derive(Debug, Clone, Copy)]
pub enum MapError {
    UnalignedVaddr(usize),
    UnalignedPaddr(usize),
    Oom,
}

impl From<Oom> for MapError {
    fn from(_: Oom) -> Self { MapError::Oom }
}

pub unsafe fn map_page(l1_phys: PAddr, vaddr: VAddr, paddr: PAddr, flags: u32) -> Result<(), MapError> {
    if !vaddr.is_aligned(PAGE_SIZE) { return Err(MapError::UnalignedVaddr(vaddr.raw())); }
    if !paddr.is_aligned(PAGE_SIZE) { return Err(MapError::UnalignedPaddr(paddr.raw())); }

    let l1 = l1_phys.raw() as *mut u32;
    let i1 = vaddr.vpn1();

    let mut l1e = core::ptr::read_volatile(l1.add(i1));
    if (l1e & PTE_V) == 0 {
        let l0 = frame_alloc::alloc_pages(1)?;
        let l0_phys = l0.paddr();
        unsafe { l0.fill(1, 0); }

        l1e = (l0_phys.ppn() << 10) | PTE_V;
        core::ptr::write_volatile(l1.add(i1), l1e);
    }

    let l0_phys = ((l1e >> 10) as usize) * PAGE_SIZE;
    let l0 = l0_phys as *mut u32;
    let i0 = vaddr.vpn0();

    let pte = (paddr.ppn() << 10) | flags | PTE_V;
    core::ptr::write_volatile(l0.add(i0), pte);
    Ok(())
}

extern "C" {
    static __kernel_base: u8;
    static __free_ram_end: u8;
}

#[inline]
const fn align_up(x: usize, a: usize) -> usize { (x + (a - 1)) & !(a - 1) }

pub unsafe fn map_kernel_identity(l1_phys: PAddr) -> Result<(), MapError> {
    let start = (&__kernel_base as *const u8 as usize) & !(PAGE_SIZE - 1);
    let end_raw = (&__free_ram_end as *const u8 as usize);
    let end = align_up(end_raw, PAGE_SIZE);

    let mut p = start;
    while p < end {
        map_page(l1_phys, VAddr(p), PAddr(p), PTE_R | PTE_W | PTE_X)?;
        p += PAGE_SIZE;
    }
    Ok(())
}

use core::arch::asm;

pub unsafe fn install_address_space(root_paddr: PAddr) {
    let satp = satp_from_root(root_paddr);
    asm!(
        "sfence.vma",
        "csrw satp, {satp}",
        "sfence.vma",
        satp = in(reg) satp,
        options(nostack, preserves_flags)
    );
}

pub unsafe fn init_kernel_paging() -> Result<PAddr, MapError> {
    let root = alloc_root_table()?;
    map_kernel_identity(root)?;
    install_address_space(root);
    Ok(root)
}
