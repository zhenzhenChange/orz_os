///
/// 工具包
///
pub mod io {
    use core::fmt::{self, Write};

    use crate::kits::sbi;

    struct Stdout;

    impl Write for Stdout {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            s.chars().for_each(|c| sbi::console(c as usize));
            Ok(())
        }
    }

    pub fn print(args: fmt::Arguments) {
        Stdout.write_fmt(args).unwrap();
    }
}

pub mod sbi {
    use sbi_rt::{NoReason, Shutdown, SystemFailure};

    pub fn console(c: usize) {
        #[allow(deprecated)]
        sbi_rt::legacy::console_putchar(c);
    }

    pub fn shutdown(failure: bool) -> ! {
        if failure {
            sbi_rt::system_reset(Shutdown, SystemFailure);
        } else {
            sbi_rt::system_reset(Shutdown, NoReason);
        }

        unreachable!()
    }
}

pub mod ffi {
    pub fn clear_bss() {
        extern "C" {
            fn sbss();
            fn ebss();
        }

        (sbss as usize..ebss as usize).for_each(|addr| unsafe {
            (addr as *mut u8).write_volatile(0);
        });
    }
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::kits::io::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::kits::io::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}
