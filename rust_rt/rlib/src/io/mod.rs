use crate::sync::{Mutex, MutexGuard};

pub mod screen;

struct StdOutRaw;

impl StdOutRaw {
    fn write_str(&mut self, s: &str) {
        crate::arch::print_str(s)
    }

    fn flush(&mut self) {
        crate::arch::flush();
    }
}

pub struct StdOut {
    lock: MutexGuard<'static, StdOutRaw>,
}

impl Drop for StdOut {
    fn drop(&mut self) {
        self.lock.flush()
    }
}

static STDOUT: Mutex<StdOutRaw> = Mutex::new(StdOutRaw);

pub fn stdout() -> StdOut {
    StdOut {
        lock: STDOUT.lock(),
    }
}

impl core::fmt::Write for StdOut {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.lock.write_str(s);
        Ok(())
    }
}
