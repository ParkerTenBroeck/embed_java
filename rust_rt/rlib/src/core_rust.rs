#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        let mut wrapper = $crate::core_rust::Wrapper::new();
        core::fmt::Write::write_fmt(&mut wrapper, core::format_args!($($arg)*)).expect("Cant write?");
    };
}
#[macro_export]
#[allow_internal_unstable(format_args_nl)]
macro_rules! println {
    ($($arg:tt)*) => {
        let mut wrapper = $crate::core_rust::Wrapper::new();
        core::fmt::Write::write_fmt(&mut wrapper, core::format_args_nl!($($arg)*)).expect("Cant write?");
    };
}

#[derive(Default)]
pub struct Wrapper {}

impl Wrapper {
    pub fn new() -> Self {
        Self {}
    }
}

impl core::fmt::Write for Wrapper {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        crate::sys::print_str(s);
        Ok(())
    }
}
