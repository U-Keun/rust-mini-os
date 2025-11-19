#![allow(dead_code)]

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct SbiRet {
    pub error: isize,
    pub value: isize,
}

mod eid {
    pub const LEGACY_CONSOLE_PUTCHAR: usize = 0x01;
    pub const SRST: usize = 0x5352_5354;
}

mod fid {
    pub const SYSTEM_RESET: usize = 0;
}

mod srst {
    pub const RESET_TYPE_SHUTDOWN: usize = 0x0000_0000;
    pub const RESET_REASON_NO_REASON: usize = 0x0000_0000;
}

#[inline]
pub unsafe fn sbi_call(eid: usize, fid: usize,
                    arg0: usize, arg1: usize, arg2: usize,
                    arg3: usize, arg4: usize, arg5: usize) -> SbiRet {
    let (mut a0, mut a1) = (arg0, arg1);
    unsafe {
        core::arch::asm!(
            "ecall",
            inout("a0") a0,
            inout("a1") a1,
            in("a2") arg2, in("a3") arg3, in("a4") arg4, in("a5") arg5,
            in("a6") fid, in("a7") eid,
            options(nostack, preserves_flags)
        );
    }
    
    SbiRet { error: a0 as isize, value: a1 as isize }
}

#[inline]
pub fn sbi_putchar(ch: u8) {
    unsafe {
        let _ = sbi_call(eid::LEGACY_CONSOLE_PUTCHAR, 
            0, ch as usize, 0, 0, 0, 0, 0);
    }
}

pub fn shutdown() -> ! {
    unsafe {
        let _ = sbi_call(
            eid::SRST, 
            fid::SYSTEM_RESET, 
            srst::RESET_TYPE_SHUTDOWN,
            srst::RESET_REASON_NO_REASON, 
            0, 0, 0, 0);
        loop { core::arch::asm!("wfi", options(nomem, nostack)); }
    }
}
