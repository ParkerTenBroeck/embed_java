
// Entry point to program
core::arch::global_asm!(
    // we really should zero the bss section but.. well java does that for us
    // so uhhh no :)
    ".section .text.start",
    ".globl _start",
    "_start:",
    "la $gp, _gp",
    "la $sp, _sp ",
    "move $fp, $sp",
    "la $ra, 0xFFFFFFFF",
    "jal main",
    "1:",
    "syscall 0",
    "b 1b",
);


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