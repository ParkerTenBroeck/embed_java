// Entry point to program
core::arch::global_asm!(
    // we really should zero the bss section but.. well java does that for us
    // so uhhh no :)
    ".section .text.start",
    ".globl _start",
    ".type _start, %function",
    "_start:",
    // stack (owned memory) starts at 0x80000000 and extends an unspecified ammount
    "li $sp, 0x80000000",
    // querry the system to see how long (in bytes) the owned memory of this thread is
    "syscall {get_stack_size}",
    // the start of the stack is the end of our owned memory
    "add $sp, $sp, $2",
    // initialize gp and fp registers
    "la $gp, _gp",
    "move $fp, $sp",
    // stack frame should start with return address 0xFFFFFFFF (I think)
    "la $ra, 0xFFFFFFFF",
    // jump to main (really this should be a j instruction but just incase main returns its jal)
    "jal main",
    // loop and constantly syscall 0 (should probably be a breakpoint)
    "1:",
    "syscall 0",
    "b 1b",
    get_stack_size = const { crate::arch::GET_OWNED_MEMORY_LENGTH }
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
