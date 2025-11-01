#![no_std]
#![no_main]

mod sbi;
mod console;
mod mem;
mod addr;
mod runtime;
mod panic;
mod csr;
mod trap;
mod alloc;

use core::arch::global_asm;

unsafe extern "C" {
    static __bss: u8;
    static __bss_end: u8;
    static __stack_top: u8;

    fn kernel_entry();
}

global_asm!(r#"
.section .text.boot
.global boot
.align 2
boot:
    la  sp, __stack_top
    j   kernel_main
"#);

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    unsafe {
        let bss_start = &__bss as *const u8 as usize;
        let bss_end = &__bss_end as *const u8 as usize;
        let len = bss_end - bss_start;
        core::ptr::write_bytes(bss_start as *mut u8, 0, len);
    }

    alloc::init();

    let p0 = alloc::alloc_pages(2).expect("oom").paddr();
    let p1 = alloc::alloc_pages(1).expect("oom").paddr();
    kprintln!("[alloc] p0={:#010x}", p0 as u32);
    kprintln!("[alloc] p1={:#010x}", p1 as u32);
    kprintln!("[alloc] delta={:#x} (expect 0x2000)", p1 - p0);

    loop {
        unsafe { core::arch::asm!("wfi", options(nomem, nostack)) }
    }
}
