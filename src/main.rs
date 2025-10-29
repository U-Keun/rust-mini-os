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


    let entry_addr = (kernel_entry as usize) & !0b11;
    csr::write_stvec_direct(entry_addr);

    kprintln!("[trap] stvec set to {:#x}, triggering illegal instruction..", entry_addr);

    unsafe {
        core::arch::asm!("csrrw x0, cycle, x0", options(nostack));
    }

    loop {
        unsafe { core::arch::asm!("wfi", options(nomem, nostack)) }
    }
}
