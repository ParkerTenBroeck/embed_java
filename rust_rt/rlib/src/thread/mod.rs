use core::{fmt::Display, num::NonZeroU32, time::Duration, cell::UnsafeCell};

use alloc::sync::Arc;

use crate::arch::START_NEW_THREAD;

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::boxed::Box;


type Result<T> = crate::result::Result<T, &'static str>;


#[cfg(feature = "alloc")]
pub fn spawn<F, T>(f: F) -> Result<JoinHandle<T>>
where
    F: FnOnce() -> T,
    F: Send + 'static,
    T: Send + 'static,
{

    let out_packet = Arc::new(Packet{
        result: UnsafeCell::new(None)
    });

    let new_thread_packet = out_packet.clone();
    
    let main = move || {
        let res = f();
        unsafe{
            *new_thread_packet.result.get() = Some(Ok(res))
        }
        drop(new_thread_packet);
    };

    let main: Box<dyn FnOnce() + 'static + Send> = box main;
    let p = Box::into_raw(box main);

    let p = p as *mut core::ffi::c_void;
    let res = unsafe { create_thread(run_thread, p, core::ptr::from_exposed_addr_mut(0x80001000)) };

    if let Err(err) = res {
        unsafe {
            //drop if thread isnt created
            let _ = Box::from_raw(p);
        }
        return Err(err);
    }else{
        return Ok(JoinHandle{
            thread: res.unwrap(),
            packet: out_packet,
        });
    }
    

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
) -> Result<Thread> {
    let res = crate::arch::syscall_sss_s::<START_NEW_THREAD>(main as u32, args as u32, stack as u32);
    if let Some(id) = NonZeroU32::new(res) {
        Ok(Thread { id })
    } else {
        Err("Failed to create thread for... idk some reason".into())
    }
}

struct Packet<T>{
    result: UnsafeCell<Option<Result<T>>>,
}

pub struct JoinHandle<T> {
    thread: Thread,
    packet: Arc<Packet<T>>
}

impl<T> JoinHandle<T>{
    pub fn thread(&self) -> &Thread{
        &self.thread
    }

    pub fn join(mut self) -> Result<T>{
        while !self.is_finished(){
            //TODO ISK ASDLKASLKJ
            crate::arch::sleep_ms(1);
        }
        let res = Arc::get_mut(&mut self.packet).unwrap();
        res.result.get_mut().take().unwrap()
    }

    pub fn is_finished(&self) -> bool{
        // this is like high key smart but I stole it from the rust std :)
        Arc::strong_count(&self.packet) == 1
    }
}

pub struct Thread{

    id: NonZeroU32,
}

impl Display for Thread {
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
