use crate::runtime::halt;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    if let Some(loc) = info.location() {
        let msg = info.message();
        crate::kprintln!("PANIC: {}:{}: {}", loc.file(), loc.line(), msg);
    } else {
        crate::kprintln!("PANIC: <unknown location>");
    }
    halt()
}

#[macro_export]
macro_rules! PANIC {
    ($fmt:literal $(, $arg:expr)*) => {{
        $crate::kprintln!("PANIC: {}:{}: {}",
            core::file!(), core::line!(), core::format_args!($fmt $(, $arg)*));
        $crate::runtime::halt()
    }};
    () => {{
        $crate::kprintln!("PANIC: {}:{}", core::file!(), core::line!());
        $crate::runtime::halt()
    }};
}
