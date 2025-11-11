#![allow(dead_code)]

use crate::csr;
use crate::PANIC;

#[repr(C)]
pub struct TrapFrame {
    pub ra:  u32,
    pub gp:  u32,
    pub tp:  u32,
    pub t0:  u32,
    pub t1:  u32,
    pub t2:  u32,
    pub t3:  u32,
    pub t4:  u32,
    pub t5:  u32,
    pub t6:  u32,
    pub a0:  u32,
    pub a1:  u32,
    pub a2:  u32,
    pub a3:  u32,
    pub a4:  u32,
    pub a5:  u32,
    pub a6:  u32,
    pub a7:  u32,
    pub s0:  u32,
    pub s1:  u32,
    pub s2:  u32,
    pub s3:  u32,
    pub s4:  u32,
    pub s5:  u32,
    pub s6:  u32,
    pub s7:  u32,
    pub s8:  u32,
    pub s9:  u32,
    pub s10: u32,
    pub s11: u32,
    pub sp:  u32,
}

use core::arch::global_asm;

global_asm!(r#"
    .section .text.trap
    .globl kernel_entry
    .align 2
kernel_entry:
    csrrw sp, sscratch, sp

    addi sp, sp, -(4 * 31)
    sw ra,  (4 * 0)(sp)
    sw gp,  (4 * 1)(sp)
    sw tp,  (4 * 2)(sp)
    sw t0,  (4 * 3)(sp)
    sw t1,  (4 * 4)(sp)
    sw t2,  (4 * 5)(sp)
    sw t3,  (4 * 6)(sp)
    sw t4,  (4 * 7)(sp)
    sw t5,  (4 * 8)(sp)
    sw t6,  (4 * 9)(sp)
    sw a0,  (4 *10)(sp)
    sw a1,  (4 *11)(sp)
    sw a2,  (4 *12)(sp)
    sw a3,  (4 *13)(sp)
    sw a4,  (4 *14)(sp)
    sw a5,  (4 *15)(sp)
    sw a6,  (4 *16)(sp)
    sw a7,  (4 *17)(sp)
    sw s0,  (4 *18)(sp)
    sw s1,  (4 *19)(sp)
    sw s2,  (4 *20)(sp)
    sw s3,  (4 *21)(sp)
    sw s4,  (4 *22)(sp)
    sw s5,  (4 *23)(sp)
    sw s6,  (4 *24)(sp)
    sw s7,  (4 *25)(sp)
    sw s8,  (4 *26)(sp)
    sw s9,  (4 *27)(sp)
    sw s10, (4 *28)(sp)
    sw s11, (4 *29)(sp)

    csrr a0, sscratch   
    sw   a0, (4 *30)(sp)

    addi a0, sp, 31*4
    csrw sscratch, a0

    mv   a0, sp         
    call handle_trap

"#);

#[unsafe(no_mangle)]
pub extern "C" fn handle_trap(_tf: *mut TrapFrame) {
    let scause  = csr::read_scause();
    let stval   = csr::read_stval();
    let sepc    = csr::read_sepc();

    PANIC!("unexpected trap scause={:#010x}, stval={:#010x}, sepc={:#010x}",
        scause as u32, stval as u32, sepc as u32);
}
