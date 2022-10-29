use core::{
    any::type_name,
    fmt::{Debug, Display},
    marker::PhantomData,
    num::NonZeroU32,
};

extern crate alloc;
use alloc::{string::String, vec::Vec};

use crate::sys::*;

use super::{class::ClassRef, primitives::JStringRef};

pub struct Object(pub(super) NonZeroU32);

impl Object {
    /// # Safety
    ///
    /// Must be a valid obj_id
    pub unsafe fn new(obj_id: NonZeroU32) -> Self {
        Self(obj_id)
    }

    /// # Safety
    ///
    /// Must be a valid obj_id
    pub unsafe fn new_p(obj_id: u32) -> Self {
        Self(obj_id.try_into().expect("Invalid NULL id given"))
    }

    pub fn id_bits(&self) -> u32 {
        self.0.into()
    }
}

pub struct ObjectRef<T>(Object, PhantomData<T>);

impl<T> ObjectRef<T> {
    /// # Safety
    ///
    /// Must be a valid obj_id
    pub unsafe fn from_obj(obj_id: Object) -> Self {
        Self(obj_id, PhantomData)
    }

    /// # Safety
    ///
    /// Must be a valid obj_id
    pub unsafe fn from_obj_ref(obj_id: ObjectRef<Object>) -> Self {
        Self(obj_id.0, PhantomData)
    }

    /// # Safety
    ///
    /// Must be a valid obj_id
    pub unsafe fn from_id_bits(obj_id: u32) -> Option<Self> {
        if let Some(obj_id) = NonZeroU32::new(obj_id) {
            Some(Self(Object::new(obj_id), PhantomData))
        } else {
            None
        }
    }

    /// # Safety
    ///
    /// Must be a valid obj_id
    pub unsafe fn from_id_bits_unchecked(obj_id: u32) -> Self {
        Self(Object::new(NonZeroU32::new_unchecked(obj_id)), PhantomData)
    }

    pub fn to_obj_ref(self) -> ObjectRef<Object> {
        ObjectRef(self.0, PhantomData)
    }

    pub fn id_bits(&self) -> u32 {
        self.0 .0.into()
    }

    pub fn get_class(&self) -> ClassRef {
        unsafe {
            let obj = Object::new_p(syscall_s_s::<GET_OBJECT_CLASS>(self.id_bits()));
            ClassRef::from_obj(obj)
        }
    }

    pub fn to_naitive_string(&self) -> String {
        unsafe {
            let (str_id, len) = syscall_s_ss::<JVM_OBJECT_TO_STRING>(self.id_bits());
            let str = JStringRef::from_id_bits(str_id).unwrap();
            str.into_naitive_string_with_capacity(len as usize)
        }
    }
}

impl<T> Display for ObjectRef<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.to_naitive_string())
    }
}

impl<T> Debug for ObjectRef<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct(type_name::<T>())
            .field("ref_id", &self.0.id_bits())
            .field("native_repr", &self.to_naitive_string())
            .finish()
    }
}

impl Drop for Object {
    fn drop(&mut self) {
        unsafe {
            syscall_s_v::<FREE_JVM_OBJECT>(self.0.try_into().unwrap());
        }
    }
}

pub struct ObjectArray<T>(PhantomData<T>);

pub type ObjectArrayRef<T> = ObjectRef<ObjectArray<T>>;

impl<T> ObjectArrayRef<T> {
    pub fn new(length: usize) -> Self {
        unsafe {
            Self::from_id_bits_unchecked(syscall_s_s::<CREATE_NEW_OBJECT_ARRAY>(length as u32))
        }
    }

    pub fn from_vec(vec: Vec<ObjectRef<T>>) -> Self {
        let mut new = Self::new(vec.len());
        for (index, item) in vec.into_iter().enumerate() {
            new.put_item(index, item)
        }
        new
    }

    /// Items will not be shifted but simply replaced with a null
    pub fn remove_item(&mut self, index: usize) -> Option<ObjectRef<T>> {
        unsafe {
            ObjectRef::from_id_bits(syscall_ss_s::<TAKE_OBJECT_AT_INDEX>(
                self.id_bits(),
                index as u32,
            ))
        }
    }

    pub fn put_item(&mut self, index: usize, item: ObjectRef<T>) {
        unsafe {
            syscall_sss_v::<PUT_OBJECT_AT_INDEX>(self.id_bits(), index as u32, item.id_bits())
        }
    }

    pub fn len(&self) -> usize {
        unsafe { syscall_s_s::<JVM_ARRAY_LENGTH>(self.0.id_bits()) as usize }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn to_native_vec(self) -> Vec<ObjectRef<T>> {
        let len = self.len();
        let mut vec = Vec::with_capacity(len);
        let ptr: *mut ObjectRef<T> = vec.as_mut_ptr();
        let ptr = ptr.addr();
        unsafe {
            let len =
                syscall_sss_s::<MOVE_INTO_NAITIVE_ARRAY>(self.id_bits(), ptr as u32, len as u32);
            vec.set_len(len as usize);
            vec
        }
    }

    pub fn to_native_vec_with_capacity(self, capacity: usize) -> Vec<ObjectRef<T>> {
        let mut vec = Vec::with_capacity(capacity);
        let ptr: *mut ObjectRef<T> = vec.as_mut_ptr();
        let ptr = ptr.addr();
        unsafe {
            let len = syscall_sss_s::<MOVE_INTO_NAITIVE_ARRAY>(
                self.id_bits(),
                ptr as u32,
                capacity as u32,
            );
            vec.set_len(len as usize);
            vec
        }
    }
}
