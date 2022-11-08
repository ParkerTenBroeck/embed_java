

#[derive(Default)]
pub struct Wrapper {}

impl Wrapper {
    pub fn new() -> Self {
        Self {}
    }
}

impl core::fmt::Write for Wrapper {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        crate::arch::print_str(s);
        Ok(())
    }
}
