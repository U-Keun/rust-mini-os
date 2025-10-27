#![no_std]
#![no_main]

use core::arch::global_asm;

unsafe extern "C" {
    static __bss: u8;
    static __bss_end: u8;
    static __stack_top: u8;
}

global_asm!(
    r#"
    .section .text.boot
    .global boot
    .align 2
boot:
    la  sp, __stack_top
    j   kernel_main
"#
);

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    unsafe {
        let bss_start = &__bss as *const u8 as usize;
        let bss_end = &__bss_end as *const u8 as usize;
        let len = bss_end - bss_start;
        core::ptr::write_bytes(bss_start as *mut u8, 0, len);
    }
    loop {
        core::hint::spin_loop();
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
