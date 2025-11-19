#![allow(dead_code)]

use core::ops::Range;

#[inline]
pub fn fill(buf: &mut [u8], byte: u8) {
    buf.fill(byte);
}

#[inline]
pub fn zero(buf: &mut [u8]) {
    fill(buf, 0);
}

#[inline]
pub fn copy_from(dst: &mut [u8], src: &[u8]) {
    assert!(dst.len() >= src.len());
    dst[..src.len()].copy_from_slice(src);
}

#[inline]
pub fn move_overlap(buf: &mut [u8], src: Range<usize>, dst_start: usize) {
    let len = src.end - src.start;
    assert!(src.end <= buf.len());
    assert!(dst_start + len <= buf.len());

    if dst_start > src.start {
        for i in (0..len).rev() {
            buf[dst_start + i] = buf[src.start + i];
        }
    } else {
        for i in 0..len {
            buf[dst_start + i] = buf[src.start + i];
        }
    }
}
