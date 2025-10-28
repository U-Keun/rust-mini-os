#![no_std]
#![no_main]

mod sbi;
mod console;
mod mem;
mod addr;
mod runtime;
mod panic;

use core::arch::global_asm;

unsafe extern "C" {
    static __bss: u8;
    static __bss_end: u8;
    static __stack_top: u8;
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

    // PANIC!("booted!");

    kprintln!("\n[boot] hello, rusty world!");

    {
        use mem::*;

        let mut buf: [u8; 32] = [0; 32];
        copy_from(&mut buf, b"hi rust\0");
        move_overlap(&mut buf, 0..2, 5);
        fill(&mut buf[10..16], b'X');

        kprintln!("[mem] ops done (buf[10..16] filled with 'X')");
    }

    {
        use addr::VAddr;
        let v = VAddr(0x1234usize);
        kprintln!("[addr] v={:#x}, aligned_up(0x1000)={:#x}",
            v.0, v.align_up(0x1000).0);
    }

    loop {
        unsafe { core::arch::asm!("wfi", options(nomem, nostack)) }
    }
}
