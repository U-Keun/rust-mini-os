use crate::sbi::sbi_putchar;

#[inline]
pub fn putchar(c: u8) {
    sbi_putchar(c)
}

pub fn puts(s: &str) {
    for &b in s.as_bytes() {
        putchar(b);
    }
}

#[derive(Copy, Clone)]
pub enum Arg<'a> { S(&'a str), D(i32), X(u32) }

pub fn kprintf(fmt: &str, args: &[Arg<'_>]) {
    let mut it = args.iter().copied();
    let mut bytes = fmt.as_bytes().iter().copied();
    while let Some(ch) = bytes.next() {
        if ch != b'%' {
            putchar(ch);
            continue;
        }
        match bytes.next() {
            Some(b'%') => putchar(b'%'),
            Some(b's') => if let Some(Arg::S(s)) = it.next() { puts(s) },
            Some(b'd') => if let Some(Arg::D(v)) = it.next() { print_dec(v) },
            Some(b'x') => if let Some(Arg::X(v)) = it.next() { print_hex32(v) },
            Some(other) => { putchar(b'%'); putchar(other); }
            None => { putchar(b'%'); break; }
        }
    }
}

fn print_dec(mut v: i32) {
    if v == 0 { putchar(b'0'); return; }
    if v < 0 { putchar(b'-'); v = -v; }
    let mut buf = [0u8; 12];
    let mut i = buf.len();
    let mut n = v as u32;
    while n > 0 {
        i -= 1; buf[i] = b'0' + (n % 10) as u8;
        n /= 10;
    }
    for &b in &buf[i..] { putchar(b); }
}

fn print_hex32(v: u32) {
    for i in (0..8).rev() {
        let nib = (v >> (i * 4)) & 0xF;
        let c = b"0123456789abcdef"[nib as usize];
        putchar(c);
    }
}
