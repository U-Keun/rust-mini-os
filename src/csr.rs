#![allow(dead_code)]

macro_rules! def_read_csr {
    ($fn: ident, $csr: ident) => {
        #[inline]
        pub fn $fn() -> usize {
            let x: usize;
            unsafe { 
                core::arch::asm!(
                    concat!("csrr {0}, ", stringify!($csr)),
                    lateout(reg) x, 
                    options(nomem, nostack, preserves_flags)
                )
            }
            x
        }
    };
}

macro_rules! def_write_csr {
    ($fn: ident, $csr: ident) => {
        #[inline]
        pub fn $fn(val: usize) {
            unsafe {
                core::arch::asm!(
                    concat!("csrw ", stringify!($csr), ", {0}"),
                    in(reg) val,
                    options(nomem, preserves_flags)
                )
            }
        }
    };
}

def_read_csr!(read_scause, scause);
def_read_csr!(read_stval, stval);
def_read_csr!(read_sepc, sepc);
def_read_csr!(read_sstatus, sstatus);

#[inline]
pub fn write_stvec_direct(handler_addr: usize) {
    debug_assert_eq!(handler_addr & 0b11, 0);
    write_stvec(handler_addr);
}

def_write_csr!(write_stvec, stvec);
def_write_csr!(write_sscratch, sscratch);
