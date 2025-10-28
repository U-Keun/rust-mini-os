#![allow(dead_code)]

#[inline]
pub fn fill(buf: &mut [u8], byte: u8) {
    buf.fill(byte);
}

#[inline]
pub fn copy_from(dst: &mut [u8], src: &[u8]) {
    assert!(dst.len() >= src.len());
    dst[..src.len()].copy_from_slice(src);
}

#[inline]
pub fn move_overlap(buf: &mut [u8], src_range: core::ops::Range<usize>, dst_start: usize) {
    let len = src_range.end - src_range.start;
    assert!(src_range.end <= buf.len());
    assert!(dst_start + len <= buf.len());

    if dst_start > src_range.start {
        for i in (0..len).rev() {
            buf[dst_start + i] = buf[src_range.start + i];
        }
    } else {
        for i in 0..len {
            buf[dst_start + i] = buf[src_range.start + i];
        }
    }
}

pub mod cstr {
    use core::ffi::CStr;

    #[inline]
    pub unsafe fn from_ptr<'a>(p: *const u8) -> &'a CStr {
        unsafe { CStr::from_ptr(p.cast()) }
    }
}
