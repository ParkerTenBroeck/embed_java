use core::marker::PhantomData;

use crate::arch::{syscall_ds_v, syscall_s_s, syscall_sss_s};

use super::object::{ObjectRef, ObjectRefTrait};

#[repr(C)]
struct FfiTuple<T: core::marker::Tuple>(T);

//--------------------------------------------------------

extern "C" fn trampoline_exit() -> ! {
    crate::arch::halt()
}

pub trait CallbackTrait<Args>
where
    Args: core::marker::Tuple,
    Self: Sized,
{
    type JniObj;

    fn call(&self, args: Args);
    extern "C" fn trampoline(&'static self, args: Args) {
        self.call(args);
    }
    fn into_jvm_obj(self) -> ObjectRef<Self::JniObj> {
        let tramp = Self::trampoline;
        let tramp_exit = trampoline_exit;

        let boxed = alloc::boxed::Box::new(self);
        let leaked = alloc::boxed::Box::into_raw(boxed);

        unsafe {
            let ret = syscall_sss_s::<1020>(tramp as u32, tramp_exit as u32, leaked as u32);
            ObjectRef::from_id_bits(ret).expect("Returned Callback object was null")
        }
    }
}

pub trait CallbackObjTrait<Args>
where
    Args: core::marker::Tuple,
    Self: ObjectRefTrait,
{
    type RustCallback: CallbackTrait<Args>;
    fn call_rust(&self, args: Args) {
        unsafe {
            let ptr = syscall_s_s::<1021>(self.id_bits());
            let ptr: *const Self::RustCallback = core::ptr::from_exposed_addr(ptr as usize);
            let ptr = ptr.as_ref().unwrap();
            ptr.call(args)
        }
    }
}

// -------------------------------------------------------------------------------------------

pub struct CallbackObj<F, Args>
where
    F: Fn<Args> + Send + 'static,
    Args: core::marker::Tuple,
{
    _f: PhantomData<(F, Args)>,
}

impl<F, Args> CallbackObjTrait<Args> for ObjectRef<CallbackObj<F, Args>>
where
    F: Fn<Args> + Send + 'static,
    Args: core::marker::Tuple,
{
    type RustCallback = Callback<F, Args>;
}

// -------------------------------------

pub struct Callback<F, Args>
where
    F: Fn<Args> + Send + 'static,
    Args: core::marker::Tuple,
{
    func: F,
    _p: PhantomData<Args>,
}

impl<F, Args> CallbackTrait<Args> for Callback<F, Args>
where
    F: Fn<Args> + Send + 'static,
    Args: core::marker::Tuple,
{
    type JniObj = CallbackObj<F, Args>;

    fn call(&self, args: Args) {
        self.func.call(args);
    }
}

impl<F, Args> Callback<F, Args>
where
    F: Fn<Args> + Send + 'static,
    Args: core::marker::Tuple,
{
    pub fn new(func: F) -> Self {
        Self {
            func,
            _p: Default::default(),
        }
    }
}

// -------------------------------------------------------------------------------------------

pub struct CallbackMutObj<F, Args>
where
    F: FnMut<Args> + Send + 'static,
    Args: core::marker::Tuple,
{
    _f: PhantomData<(F, Args)>,
}

impl<F, Args> CallbackObjTrait<Args> for ObjectRef<CallbackMutObj<F, Args>>
where
    F: FnMut<Args> + Send + 'static,
    Args: core::marker::Tuple,
{
    type RustCallback = CallbackMut<F, Args>;
}

// -------------------------------------

pub struct CallbackMut<F, Args>
where
    F: FnMut<Args> + Send + 'static,
    Args: core::marker::Tuple,
{
    func: crate::sync::Mutex<F>,
    _p: PhantomData<Args>,
}

impl<F, Args> CallbackTrait<Args> for CallbackMut<F, Args>
where
    F: FnMut<Args> + Send + 'static,
    Args: core::marker::Tuple,
{
    type JniObj = CallbackMutObj<F, Args>;

    fn call(&self, args: Args) {
        self.func.lock().call_mut(args);
    }
}

impl<F, Args> CallbackMut<F, Args>
where
    F: FnMut<Args> + Send + 'static,
    Args: core::marker::Tuple,
{
    pub fn new(func: F) -> Self {
        Self {
            func: crate::sync::Mutex::new(func),
            _p: Default::default(),
        }
    }
}

// -------------------------------------------------------------------------------------------

pub struct CallbackOnceObj<F, Args>
where
    F: FnOnce<Args> + Send + 'static,
    Args: core::marker::Tuple,
{
    _f: PhantomData<(F, Args)>,
}

impl<F, Args> CallbackObjTrait<Args> for ObjectRef<CallbackOnceObj<F, Args>>
where
    F: FnOnce<Args> + Send + 'static,
    Args: core::marker::Tuple,
{
    type RustCallback = CallbackOnce<F, Args>;
}

// -------------------------------------

pub struct CallbackOnce<F, Args>
where
    F: FnOnce<Args> + Send + 'static,
    Args: core::marker::Tuple,
{
    func: crate::sync::Mutex<Option<F>>,
    _p: PhantomData<Args>,
}

impl<F, Args> CallbackTrait<Args> for CallbackOnce<F, Args>
where
    F: FnOnce<Args> + Send + 'static,
    Args: core::marker::Tuple,
{
    type JniObj = CallbackOnceObj<F, Args>;

    fn call(&self, args: Args) {
        self.func
            .lock()
            .take()
            .expect("Already called Once")
            .call_once(args);
    }
}

impl<F, Args> CallbackOnce<F, Args>
where
    F: FnOnce<Args> + Send + 'static,
    Args: core::marker::Tuple,
{
    pub fn new(func: F) -> Self {
        Self {
            func: crate::sync::Mutex::new(Some(func)),
            _p: Default::default(),
        }
    }
}

// -------------------------------------

// pub fn test() {
//     let mut time = crate::arch::current_time_nanos();
//     let cb = CallbackMut::new(move |ran: u32| {
//         let now = crate::arch::current_time_nanos();
//         let diff = now - time;
//         time = now;
//         crate::println!("nano diff: {diff}, times ran: {ran}");
//     });
//     let mut obj = cb.into_jvm_obj();
//     obj.call_rust((23,));
//     unsafe {
//         syscall_ds_v::<1027>(100, obj.id_bits());
//     }
//     // unsafe {
//     //     cb.raw_dog_callback();
//     // }
// }
