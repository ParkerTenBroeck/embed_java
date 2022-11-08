pub fn exit(code: i32) -> ! {
    loop {
        unsafe {
            use crate::arch::PROGRAM_EXIT;
            crate::arch::syscall_s_v::<PROGRAM_EXIT>(code as u32);
        }
    }
}
