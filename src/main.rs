#![no_std]
#![no_main]

mod sbi;
mod console;

use core::arch::global_asm;
use console::{ puts, kprintf, Arg };

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

    puts("\n\n Hello World!\n");

    kprintf("1 + 2 = %d, hex=%x, str=%s\n",
        &[Arg::D(1 + 2), Arg::X(0x1a2b3c4d), Arg::S("ok")]);

    loop { unsafe { core::arch::asm!("wfi", options(nomem, nostack)) } }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let _ = info;
    loop {}
}
