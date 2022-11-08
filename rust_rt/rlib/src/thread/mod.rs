use core::{fmt::Display, num::NonZeroU32, time::Duration};

use crate::arch::START_NEW_THREAD;

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::boxed::Box;

pub fn black_box<T>(dummy: T) -> T {
    unsafe {
        let ret = core::ptr::read_volatile(&dummy);
        core::mem::forget(dummy);
        ret
    }
}

#[cfg(feature = "alloc")]
pub fn start_new_thread<F, T>(f: F) -> Result<JoinHandle, ()>
where
    F: FnOnce() -> T,
    F: Send + 'static,
    T: Send + 'static,
{


    let stack_size = 0x3FF;
    let stack_layout = alloc::alloc::Layout::from_size_align(stack_size, 0x8).unwrap();
    let raw_stack = unsafe{
        let raw_stack = alloc::alloc::alloc(stack_layout);
       raw_stack as *mut core::ffi::c_void
    };
    
    let main = move || {
        let res = f();
        unsafe{
            alloc::alloc::dealloc(raw_stack as *mut u8, stack_layout)
        }
    };
    let main: Box<dyn FnOnce() /* + 'static + Send */> = box main;
    let p = Box::into_raw(box main);

    let p = p as *mut core::ffi::c_void;
    let res = unsafe { create_thread(run_thread, p, raw_stack.sub(stack_size as usize)) };
    if res.is_err() {
        unsafe {
            //drop if thread isnt created
            let _ = Box::from_raw(p);
        }
    }
    return res;

    extern "C" fn run_thread(main: *mut core::ffi::c_void) -> ! {
        unsafe {
            Box::from_raw(main as *mut Box<dyn FnOnce() + Send>)();
        }
        crate::arch::halt();
    }
}

pub unsafe fn create_thread( 
    main: extern "C" fn(*mut core::ffi::c_void) -> !,
    args: *mut core::ffi::c_void,
    stack:  *mut core::ffi::c_void,
) -> Result<JoinHandle, ()> {
    let res = crate::arch::syscall_sss_s::<START_NEW_THREAD>(main as u32, args as u32, stack as u32);
    if let Some(id) = NonZeroU32::new(res) {
        Ok(JoinHandle { id })
    } else {
        Err(())
    }
}

pub struct JoinHandle {
    id: NonZeroU32,
}

impl Display for JoinHandle {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.id)
    }
}

pub fn sleep(dur: Duration) {
    unimplemented!("uh you're not suppost to see this yet :)");
    // let nanos = dur.as_nanos() as u64;
    // unsafe {
    //     use crate::arch::SLEEP_NANOS;
    //     crate::arch::syscall_d_v::<SLEEP_NANOS>(nanos);
    // }
}
