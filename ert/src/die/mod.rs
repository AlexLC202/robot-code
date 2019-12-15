//! Handle failures we can't or don't want to recover from.
//! Unlike AOS, which handles logging in another process,
//! these will take down the whole system.
//! Thread-level issues should just be panics.
//! A wrapper that logs then panics is planned.

/// Implementation detail.
#[doc(hidden)]
pub fn fatal_dump_filename() -> String {
    format!("/tmp/ert_fatal_error_{}.txt", unsafe { libc::rand() })
}

/// Die and log the error without using logging facilities
/// Takes the whole process with it. Destructors do not run.
#[macro_export]
macro_rules! die {
    ($($arg:tt)*) => {
        let info = concat!("At ", std::file!(), ":", std::line!());
        eprintln!("ERT encountered a fatal error! Info to follow:");
        eprintln!("{}", info);
        eprintln!($($arg)*);
        let name = $crate::die::fatal_dump_filename();
        use std::io::Write;
        let res = std::fs::File::create(&name).and_then(|mut file| {
            writeln!(file, "{}", info)?;
            writeln!(file, $($arg)*)
        });
        if res.is_ok() {
            eprintln!("Info reproduced to {}", &name);
        } else {
            eprintln!("Failed to write error info to {}: {:?}", &name, res);
        }
        std::process::exit(-1);
    };
}

pub trait DieOnResult<T> {
    fn expect_or_die(self, msg: &'static str) -> T;
    fn unwrap_or_die(self) -> T;
}
impl<T, E: std::fmt::Debug> DieOnResult<T> for Result<T, E> {
    fn expect_or_die(self, msg: &'static str) -> T {
        match self {
            Ok(t) => t,
            Err(e) => unwrap_die(msg, &e),
        }
    }
    fn unwrap_or_die(self) -> T {
        match self {
            Ok(t) => t,
            Err(e) => unwrap_die("called `Result::unwrap()` on an `Err` value", &e),
        }
    }
}

// This is a separate function to reduce the code size of the methods
#[inline(never)]
#[cold]
fn unwrap_die(msg: &str, error: &dyn std::fmt::Debug) -> ! {
    die!("{}: {:?}", msg, error);
}

/// Die and log the error without using logging facilities
/// Takes the whole process with it. Destructors do not run.
/// Grabs and logs errno automatically.
#[macro_export]
macro_rules! die_with_errno {
    ($fmt:expr, $($arg:tt)*) => {
        let errno_save = nix::errno::errno();
        $crate::die!(concat!($fmt, " (caused by error code {}, {})"), $($arg)*, errno_save, nix::errno::from_i32(errno_save));
    };
    ($fmt:expr) => {
        let errno_save = nix::errno::errno();
        $crate::die!(concat!($fmt, " (caused by error code {}, {})"), errno_save, nix::errno::from_i32(errno_save));
    };
}

#[macro_export]
macro_rules! panic_with_errno {
    ($fmt:expr, $($arg:tt)*) => {
        panic!(concat!($fmt, " (caused by error code {}, {})"), $($arg)*, nix::errno::errno(), nix::errno::Errno::last());
    };
    ($fmt:expr) => {
        panic!(concat!($fmt, " (caused by error code {}, {})"), nix::errno::errno(), nix::errno::Errno::last());
    };
}
