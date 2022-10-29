#[panic_handler]
#[no_mangle]
fn panic(info: &core::panic::PanicInfo) -> ! {
    rlib::println!("{}", info);
    rlib::println!("STOPPING");
    rlib::sys::halt();
}
