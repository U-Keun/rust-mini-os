#![allow(dead_code)]
#![allow(unsafe_op_in_unsafe_fn)]
#![allow(static_mut_refs)]

use core::arch::global_asm;

use crate::csr;
use crate::PANIC;
// use crate::paging::{ self, PTE_R, PTE_W, PTE_X };

// ===== Config & Types =====

pub const PROCS_MAX: usize = 8;
pub const KSTACK_SIZE: usize = 8192; // 8KB

const CALLEE_SAVED_COUNT: usize = 12;
const FRAME_WORDS: usize = CALLEE_SAVED_COUNT + 1;
const WORD_SIZE: usize = core::mem::size_of::<u32>();
const FRAME_BYTES: usize = FRAME_WORDS * WORD_SIZE;

static mut DUMMY_SP: usize = 0;

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum State {
    Unused = 0,
    Runnable = 1,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Process {
    pub pid: i32,
    pub state: State,
    pub sp: usize, // kernel stack pointer
    pub pt_root_phys: usize,
    pub stack: [u8; KSTACK_SIZE],
}

impl Process {
    pub const fn unused() -> Self {
        Self {
            pid: -1,
            state: State::Unused,
            sp: 0,
            pt_root_phys: 0,
            stack: [0; KSTACK_SIZE],
        }
    }
}

// ===== Globals (raw, hidden behind helpers) =====

static mut PROCS: [Process; PROCS_MAX] = [Process::unused(); PROCS_MAX];

pub static mut CURRENT: *mut Process = core::ptr::null_mut();
pub static mut IDLE: *mut Process = core::ptr::null_mut();

#[inline(always)]
unsafe fn procs_base_mut() -> *mut Process {
    core::ptr::addr_of_mut!(PROCS) as *mut Process
}

#[inline(always)]
unsafe fn procs_base() -> *const Process {
    core::ptr::addr_of!(PROCS) as *const Process
}

#[inline(always)]
unsafe fn proc_mut_at(i: usize) -> *mut Process {
    procs_base_mut().add(i)
}

#[inline(always)]
unsafe fn proc_at(i: usize) -> *const Process {
    procs_base().add(i)
}

// ===== Context switch (ASM) =====

unsafe extern "C" {
    fn switch_context(prev_sp: *mut usize, next_sp: *const usize);
}

/*
 * switch_context(prev_sp, next_sp)
 *
 * Save callee-saved on current stack:
 *  ra, s0..s11 (13*4 bytes)
 *
 * *prev_sp = sp
 * sp = *next_sp
 *
 * Restore callee-saved from next stack and return
 * */
global_asm!(r#"
    .section .text
    .global switch_context
    .align 2
switch_context:
    addi   sp, sp, -13*4
    sw     ra,     0*4(sp)
    sw     s0,     1*4(sp)
    sw     s1,     2*4(sp)
    sw     s2,     3*4(sp)
    sw     s3,     4*4(sp)
    sw     s4,     5*4(sp)
    sw     s5,     6*4(sp)
    sw     s6,     7*4(sp)
    sw     s7,     8*4(sp)
    sw     s8,     9*4(sp)
    sw     s9,     10*4(sp)
    sw     s10,     11*4(sp)
    sw     s11,     12*4(sp)

    sw     sp,     0(a0)
    lw     sp,     0(a1)

    lw     ra,     0*4(sp)
    lw     s0,     1*4(sp)
    lw     s1,     2*4(sp)
    lw     s2,     3*4(sp)
    lw     s3,     4*4(sp)
    lw     s4,     5*4(sp)
    lw     s5,     6*4(sp)
    lw     s6,     7*4(sp)
    lw     s7,     8*4(sp)
    lw     s8,     9*4(sp)
    lw     s9,     10*4(sp)
    lw     s10,     11*4(sp)
    lw     s11,     12*4(sp)
    addi   sp, sp, 13*4
    ret
"#);

// ===== PCB allocation & process creation =====

unsafe fn alloc_pcb() -> Option<*mut Process> {
    for i in 0..PROCS_MAX {
        let p = proc_mut_at(i);
        if (*p).state == State::Unused {
            (*p).pid = (i + 1) as i32;
            (*p).state = State::Runnable;
            (*p).sp = 0;
            return Some(p);
        }
    }
    None
}

pub type Entry = extern "C" fn() -> !;

unsafe fn init_kernel_frame_and_sp(proc: *mut Process, entry: Entry) {
    let stack_top = (&(*proc).stack as *const _ as usize) + KSTACK_SIZE;
    let frame_ptr = (stack_top - FRAME_BYTES) as *mut u32;

    for i in 0..CALLEE_SAVED_COUNT {
        core::ptr::write(frame_ptr.add(1 + i), 0);
    }

    core::ptr::write(frame_ptr.add(0), entry as usize as u32);

    (*proc).sp = frame_ptr as usize;
}

pub unsafe fn create_process(entry: Entry) -> *mut Process {
    let p = match alloc_pcb() {
        Some(p) => p,
        None => PANIC!("no free PCB slots"),
    };
    init_kernel_frame_and_sp(p, entry);
    p
}

// ===== Scheduler =====

unsafe fn current_index(current: *const Process) -> Option<usize> {
    if current.is_null() { return None; }
    let base = procs_base() as usize;
    let cur = current as usize;
    let off = cur - base;
    let sz = core::mem::size_of::<Process>();
    Some(off / sz)
}

unsafe fn all_unused() -> bool {
    for i in 0..PROCS_MAX {
        if (*proc_at(i)).state != State::Unused {
            return false;
        }
    }
    true
}

unsafe fn pick_next(current: *mut Process) -> *mut Process {
    if all_unused() { return IDLE; }

    let start = match current_index(current) {
        Some(idx) => (idx + 1) % PROCS_MAX,
        None => 0,
    };

    for step in 0..PROCS_MAX {
        let idx = (start + step) % PROCS_MAX;
        let p = proc_mut_at(idx);
        if (*p).state == State::Runnable && (*p).pid > 0 { return p; }
    }
    IDLE
}

pub unsafe fn yield_now() {
    let cur = CURRENT;
    let next = pick_next(cur);

    if !next.is_null() && next != cur {
        let ksp_top = (&(*next).stack as *const _ as usize) + KSTACK_SIZE;
        csr::write_sscratch(ksp_top as usize);

        let prev_sp_ptr: *mut usize = if cur.is_null() {
            core::ptr::addr_of_mut!(DUMMY_SP)
        } else {
            core::ptr::addr_of_mut!((*cur).sp)
        };

        CURRENT = next;
        switch_context(prev_sp_ptr, core::ptr::addr_of!((*next).sp));
    }
}

// ===== Idle & bootstrap =====

pub extern "C" fn idle_entry() -> ! {
    loop {
        unsafe { core::arch::asm!("wfi", options(nomem, nostack)); }
    }
}

pub unsafe fn init_and_boot(a_entry: Entry, b_entry: Entry) {
    let _kernel_root = crate::mem::paging::init_kernel_paging();

    let idle = match alloc_pcb() {
        Some(p) => p,
        None => PANIC!("no free pcb for idle"),
    };
    (*idle).pid = 0;
    IDLE = idle;
    CURRENT = IDLE;

    let _a = create_process(a_entry);
    let _b = create_process(b_entry);

    yield_now();
}

