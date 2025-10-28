#![allow(dead_code)]

#[inline(always)]
pub fn halt() -> ! {
    loop {
        unsafe { core::arch::asm!("wfi", options(nomem, nostack)) }
    }
}
