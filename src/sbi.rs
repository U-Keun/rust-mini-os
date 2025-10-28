#![allow(dead_code)]

#[repr(C)]
pub struct SbiRet {
    pub error: isize,
    pub value: isize,
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
        let _ = sbi_call(0x01, 0, ch as usize, 0, 0, 0, 0, 0);
    }
}
