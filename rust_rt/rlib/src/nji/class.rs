extern crate alloc;
use alloc::{string::String, vec::Vec};

use crate::arch::*;

use super::{
    object::{ObjectArrayRef, ObjectRef},
    primitives::JStringRef,
    reflection::{ConstructorRef, FieldRef, MethodRef},
};

pub struct Class;
pub type ClassRef = ObjectRef<Class>;

impl ClassRef {
    pub fn for_name(name: &str) -> Option<Self> {
        unsafe {
            Self::from_id_bits(syscall_ss_s::<CLASS_FOR_NAME>(
                name.as_ptr() as u32,
                name.len() as u32,
            ))
        }
    }

    pub fn get_name(&self) -> String {
        unsafe {
            let (str_id, len) = syscall_s_ss::<GET_FULL_CLASS_NAME>(self.id_bits());
            let obj = JStringRef::from_id_bits(str_id).unwrap();
            obj.into_naitive_string_with_capacity(len as usize)
        }
    }

    pub fn get_methods(&self) -> Vec<MethodRef> {
        unsafe {
            let (id, len) = syscall_s_ss::<GET_DECLAIRED_METHODS_OF_JVM_CLASS>(self.id_bits());
            let arr = ObjectArrayRef::from_id_bits(id).unwrap();
            let arr = arr.to_native_vec_with_capacity(len as usize);
            arr
        }
    }

    pub fn get_method(
        &self,
        name: &JStringRef,
        parameters: ObjectArrayRef<Class>,
    ) -> Option<MethodRef> {
        unsafe {
            MethodRef::from_id_bits(syscall_sss_s::<GET_CLASS_METHOD>(
                self.id_bits(),
                name.id_bits(),
                parameters.id_bits(),
            ))
        }
    }

    pub fn get_constructors(&self) -> Vec<ConstructorRef> {
        unsafe {
            let (id, len) = syscall_s_ss::<GET_DECLAIRED_CONSTRUCTORS_OF_JVM_CLASS>(self.id_bits());
            let arr = ObjectArrayRef::from_id_bits(id).unwrap();
            let arr = arr.to_native_vec_with_capacity(len as usize);
            arr
        }
    }

    pub fn get_constructor(&self, parameters: ObjectArrayRef<Class>) -> Option<ConstructorRef> {
        unsafe {
            ConstructorRef::from_id_bits(syscall_ss_s::<GET_CLASS_CONSTRUCTOR>(
                self.id_bits(),
                parameters.id_bits(),
            ))
        }
    }

    pub fn get_fields(&self) -> Vec<FieldRef> {
        unsafe {
            let (id, len) = syscall_s_ss::<GET_DECLAIRED_FIELDS_OF_JVM_CLASS>(self.id_bits());
            let arr = ObjectArrayRef::from_id_bits(id).unwrap();
            let arr = arr.to_native_vec_with_capacity(len as usize);
            arr
        }
    }

    pub fn get_field(&self, name: &JStringRef) -> Option<FieldRef> {
        unsafe {
            FieldRef::from_id_bits(syscall_ss_s::<GET_CLASS_FIELD>(
                self.id_bits(),
                name.id_bits(),
            ))
        }
    }
}
