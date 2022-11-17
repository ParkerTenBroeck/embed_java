use core::marker::PhantomData;

use crate::arch::{syscall_s_v, syscall_sss_s, syscall_ds_v};

use super::object::ObjectRef;

#[repr(C)]
struct FfiTuple<T: core::marker::Tuple>(T);

//--------------------------------------------------------

impl<F, Args> ObjectRef<CallbackObj<F, Args>>
where
    F: Fn<Args> + Send + Sync + 'static,
    Args: core::marker::Tuple,
{
    pub fn call_rust(&self, args: Args) {
        unimplemented!()
    }
}

struct CallbackObj<F, Args>
where
    F: Fn<Args> + Send + Sync + 'static,
    Args: core::marker::Tuple,
{
    _f: PhantomData<(F, Args)>,
}

struct Callback<F, Args>
where
    F: Fn<Args> + Send + 'static,
    Args: core::marker::Tuple,
{
    func: F,
    _p: PhantomData<Args>,
}

impl<F, Args> Callback<F, Args>
where
    F: Fn<Args> + Send + Sync + 'static,
    Args: core::marker::Tuple,
{
    pub fn new(func: F) -> Self {
        Self {
            func,
            _p: Default::default(),
        }
    }

    pub fn call(&self, args: Args) {
        self.func.call(args);
    }

    extern "C" fn trampoline(s: *const Self, args: Args) {
        unsafe {
            s.as_ref().unwrap().call(args);
        }
    }

    extern "C" fn trampoline_exit() -> ! {
        crate::arch::halt()
    }

    pub fn into_jvm_obj(self) -> ObjectRef<CallbackObj<F, Args>> {
        let tramp = Self::trampoline;
        let tramp_exit = Self::trampoline_exit;

        let boxed = alloc::boxed::Box::new(self);
        let leaked = alloc::boxed::Box::into_raw(boxed);

        unsafe {
            let ret = syscall_sss_s::<1020>(tramp as u32, tramp_exit as u32, leaked as u32);
            ObjectRef::from_id_bits(ret).expect("Returned Callback object was null")
        }
    }

    pub unsafe fn raw_dog_callback(self) {
        let callback = self.into_jvm_obj();

        crate::println!("{callback}");
        syscall_s_v::<1021>(callback.id_bits());
        drop(callback)
    }
}

// -------------------------------------------------------------------------------------------

impl<F, Args> ObjectRef<CallbackMutObj<F, Args>>
where
    F: FnMut<Args> + Send + Sync + 'static,
    Args: core::marker::Tuple,
{
    pub fn call_rust(&mut self, args: Args) {
        unimplemented!()
    }
}

struct CallbackMutObj<F, Args>
where
    F: FnMut<Args> + Send + Sync + 'static,
    Args: core::marker::Tuple,
{
    _f: PhantomData<(F, Args)>,
}

#[repr(C)]
struct CallbackMut<F, Args>
where
    F: FnMut<Args> + Send + 'static,
    Args: core::marker::Tuple,
{
    func: crate::sync::Mutex<F>,
    _p: PhantomData<Args>,
}

impl<F, Args> CallbackMut<F, Args>
where
    F: FnMut<Args> + Send + Sync + 'static,
    Args: core::marker::Tuple,
{
    pub fn new(func: F) -> Self {
        Self {
            func: crate::sync::Mutex::new(func),
            _p: Default::default(),
        }
    }

    pub fn call(&self, args: Args) {
        self.func.lock().call_mut(args);
    }

    extern "C" fn trampoline(s: *const Self, args: Args) {
        unsafe {
            s.as_ref().unwrap().call(args);
        }
    }

    extern "C" fn trampoline_exit() -> ! {
        crate::arch::halt()
    }

    pub fn into_jvm_obj(self) -> ObjectRef<CallbackMutObj<F, Args>> {
        let tramp = Self::trampoline;
        let tramp_exit = Self::trampoline_exit;

        let boxed = alloc::boxed::Box::new(self);
        let leaked = alloc::boxed::Box::into_raw(boxed);

        unsafe {
            let ret = syscall_sss_s::<1020>(tramp as u32, tramp_exit as u32, leaked as u32);
            ObjectRef::from_id_bits(ret).expect("Returned Callback object was null")
        }
    }

    pub unsafe fn raw_dog_callback(self) {
        let callback = self.into_jvm_obj();

        crate::println!("{callback}");
        syscall_ds_v::<1021>(100, callback.id_bits());
        drop(callback)
    }
}

pub fn test() {
    let mut time = crate::arch::current_time_nanos();
    let cb = CallbackMut::new(move |ran: u32| {
        let now = crate::arch::current_time_nanos();
        let diff = now - time;
        time = now;

        crate::println!("nano diff: {diff}, times ran: {ran}");
    });

    unsafe {
        cb.raw_dog_callback();
    }
}
