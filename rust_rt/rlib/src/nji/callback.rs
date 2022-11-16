use crate::{arch::{syscall_ss_s, syscall_s_v}};

use super::object::{ObjectRef, Object};


#[repr(C)]
struct FfiTuple<T: core::marker::Tuple>{
    tup: T
}

#[repr(C)]
struct Callback<F, Args>
where
    F: FnOnce<Args> + Send + 'static,
    Args: core::marker::Tuple,
{
    call_fn: fn(Callback<F, Args>, FfiTuple<Args>),
    func: F,
}

impl<F, Args> Callback<F, Args>
where
    F: FnOnce<Args> + Send + 'static,
    Args: core::marker::Tuple,
{
    pub fn new(func: F) -> Self {
        Self {
            func,
            call_fn: Self::call,
        }
    }

    pub fn call(self, args: FfiTuple<Args>) {
        self.func.call_once(args.tup);
    }

    pub extern "C" fn trampoline(s: *mut Self, args: FfiTuple<Args>) -> !{
        unsafe{
            alloc::boxed::Box::from_raw(s).call(args);
            crate::arch::halt()
        }
    }

    pub fn trampoline_ptr(&self) -> extern "C" fn(s: *mut Self, args: FfiTuple<Args>) -> !{
        Self::trampoline
    }

    pub unsafe fn raw_dog_callback(self) {
        let tramp = self.trampoline_ptr();
        let boxed = alloc::boxed::Box::new(self);
        let leaked = alloc::boxed::Box::into_raw(boxed);
        
        let ret = syscall_ss_s::<1020>(tramp as u32, leaked as u32);
        crate::println!("asdasdasd");
        let callback:ObjectRef<Object> = ObjectRef::from_id_bits(ret).expect("Returned Callback object was null");

        crate::println!("{callback}");
        syscall_s_v::<1021>(callback.id_bits());
        crate::println!("{callback}");
        
    }
}


pub fn test() {
    let cb = Callback::new(|v1: u8,v2: u16,v3: u32| {
        crate::println!("v1: {:08X}, v2: {:08X}, v3: {:08X}",v1,v2,v3);
    });
    unsafe{

        cb.raw_dog_callback();
    }
    // (cb.call_fn)(cb, 32);
}

// impl<F> Callback<F, (u32,)>
// where
//     F: FnOnce(u32) + Send + 'static,
// {

//     fn test(&self){

//     }
// }