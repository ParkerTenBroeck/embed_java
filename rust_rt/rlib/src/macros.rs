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