#![allow(dead_code)]
use core::fmt::{ self, Write };
use core::sync::atomic::{ AtomicBool, Ordering };
use crate::sbi::sbi_putchar;

struct Spin(AtomicBool);
impl Spin {
    const fn new() -> Self { Self(AtomicBool::new(false)) }
    fn lock(&self) {
        while self.0.swap(true, Ordering::Acquire) {}
    }
    fn unlock(&self) { self.0.store(false, Ordering::Release); }
}

static LOCK: Spin = Spin::new();

pub struct Console;

impl Write for Console {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        LOCK.lock();
        for &b in s.as_bytes() {
            sbi_putchar(b);
        }
        LOCK.unlock();
        Ok(())
    }
}

#[inline]
pub fn putchar(b: u8) {
    sbi_putchar(b);
}

#[inline]
pub fn puts(s: &str) {
    for &ch in s.as_bytes() {
        putchar(ch);
    }
}

#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        let _ = write!($crate::console::Console, $($arg)*);
    }};
}

#[macro_export]
macro_rules! kprintln {
    () => { $crate::kprint!("\n") };
    ($($arg:tt)*) => {{
        $crate::kprint!("{}\n", format_args!($($arg)*));
    }};
}
