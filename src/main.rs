#![no_std]
#![no_main]

mod sbi;
mod console;
mod runtime;
mod panic;
mod csr;
mod trap;
mod mem;
mod process;

use crate::process::{ yield_now, init_and_boot };

use core::arch::global_asm;

unsafe extern "C" {
    static __bss: u8;
    static __bss_end: u8;
    static __stack_top: u8;

    static __kernel_base: u8;
    static __free_ram_end: u8;

    fn trap_entry();
}

global_asm!(r#"
.section .text.boot
.global boot
.align 2
boot:
    la  sp, __stack_top
    j   kernel_main
"#);

use core::sync::atomic::{ AtomicUsize, Ordering };
static PRINT_CNT: AtomicUsize = AtomicUsize::new(0);
const MAX_PRINTS: usize = 200;

extern "C" fn proc_a_entry() -> ! {
    kprintln!("starting process A");
    loop {
        crate::console::putchar(b'A');
        if PRINT_CNT.fetch_add(1, Ordering::Relaxed) + 1 >= MAX_PRINTS {
            kprintln!("\n[done] reached {} prints -> shutdown", MAX_PRINTS);
            crate::sbi::shutdown();
        }
        unsafe { yield_now(); }
        busy_delay();
    }
}

extern "C" fn proc_b_entry() -> ! {
    kprintln!("starting process B");
    loop {
        crate::console::putchar(b'B');
        if PRINT_CNT.fetch_add(1, Ordering::Relaxed) + 1 >= MAX_PRINTS {
            kprintln!("\n[done] reached {} prints -> shutdown", MAX_PRINTS);
            crate::sbi::shutdown();
        }
        unsafe { yield_now(); }
        busy_delay();
    }
}

#[inline(always)]
fn busy_delay() {
    for _ in 0..30_000 { unsafe { core::arch::asm!("nop"); } }
}

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    unsafe {
        let bss_start = &__bss as *const u8 as usize;
        let bss_end = &__bss_end as *const u8 as usize;
        let len = bss_end - bss_start;
        core::ptr::write_bytes(bss_start as *mut u8, 0, len);
    }

    let entry_addr = (trap_entry as usize) & !0b11;
    csr::write_stvec_direct(entry_addr);

    crate::mem::frame_alloc::init();

    unsafe {
        init_and_boot(proc_a_entry, proc_b_entry);
    }

    loop {
        unsafe { core::arch::asm!("wfi", options(nomem, nostack)) }
    }
}
