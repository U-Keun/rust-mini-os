#![allow(dead_code)]

use core::sync::atomic::{ AtomicUsize, Ordering };
use crate::mem::PAGE_SIZE;
use crate::mem::addr::PAddr;

unsafe extern "C" {
    static __free_ram: u8;
    static __free_ram_end: u8;
}

static NEXT: AtomicUsize = AtomicUsize::new(0);
static LIMIT: AtomicUsize = AtomicUsize::new(0);

#[inline]
const fn align_up(v: usize, a: usize) -> usize {
    (v + (a - 1)) & !(a - 1)
}

#[derive(Debug, Clone, Copy)]
pub struct Oom;

#[derive(Debug, Clone, Copy)]
pub struct PageFrame {
    paddr: PAddr,
}

impl PageFrame {
    #[inline] pub fn paddr(self) -> PAddr { self.paddr }

    pub unsafe fn as_bytes_mut_static(self, n_pages: usize) -> &'static mut [u8] {
        let len = n_pages * PAGE_SIZE;
        unsafe { core::slice::from_raw_parts_mut(self.paddr as *mut u8, len) }
    }

    pub unsafe fn fill(self, n_pages: usize, byte: u8) {
        let len = n_pages * PAGE_SIZE;
        unsafe { core::ptr::write_bytes(self.paddr as *mut u8, byte, len); }
    }
}

pub fn init() {
    let start = unsafe { &__free_ram as *const u8 as usize };
    let end = unsafe { &__free_ram_end as *const u8 as usize };
    let start = align_up(start, PAGE_SIZE);
    NEXT.store(start, Ordering::Relaxed);
    LIMIT.store(end, Ordering::Relaxed);
}

pub fn alloc_pages(n: usize) -> Result<PageFrame, Oom> {
    let bytes = n.checked_mul(PAGE_SIZE).ok_or(Oom)?;

    loop {
        let cur = NEXT.load(Ordering::Relaxed);
        let end = LIMIT.load(Ordering::Relaxed);
        let new_ = cur.checked_add(bytes).ok_or(Oom)?;
        if new_ > end { return Err(Oom); }

        if NEXT.compare_exchange_weak(cur, new_, Ordering::AcqRel, Ordering::Relaxed).is_ok() {
            unsafe { core::ptr::write_bytes(cur as *mut u8, 0, bytes); }
            return Ok(PageFrame { paddr: PAddr(cur) });
        }
    }
}
