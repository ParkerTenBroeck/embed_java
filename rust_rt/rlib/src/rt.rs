#[no_mangle]
#[naked]
#[link_section = ".text.start"]
extern "C" fn _start() -> ! {
    unsafe {
        core::arch::asm! {
            ".set noat",
            //".cpload $25",
            "la $gp, _gp_disp",
            "la $sp, _sp ",
            "move $fp, $sp",
            "jal main",
            "1:",
            "syscall 0",
            "b 1b", options(noreturn),
        }
    }
}

extern "C" {
    pub fn main();
}

#[inline(always)]
/// # Safety
/// this is the start of the heap dont touch it if you arent the global allocator ;)
pub unsafe fn heap_address() -> *mut u8 {
    let ret;
    core::arch::asm!(
        ".set noat",
        "la {0}, _heap",
        out(reg) ret
    );
    ret
}

#[panic_handler]
#[cfg(feature = "provide_panic_handler")]
fn panic(info: &core::panic::PanicInfo) -> ! {
    crate::println!("PANIC AT THE DISCO: {:#?}", info);
    loop {
        crate::process::exit(-1);
    }
}