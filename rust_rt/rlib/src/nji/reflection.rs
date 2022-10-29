use crate::sys::{syscall_sss_ss, INVOKE_METHOD};

use super::object::{Object, ObjectArrayRef, ObjectRef};

pub struct Method;
pub type MethodRef = ObjectRef<Method>;

impl MethodRef {
    pub fn invoke(
        &self,
        object: ObjectRef<Object>,
        arguments: &mut ObjectArrayRef<Object>,
    ) -> Result<Option<ObjectRef<Object>>, ObjectRef<Object>> {
        self.invoke_(object.id_bits(), arguments.id_bits())
    }

    pub fn invoke_static(
        &self,
        arguments: &mut ObjectArrayRef<Object>,
    ) -> Result<Option<ObjectRef<Object>>, ObjectRef<Object>> {
        self.invoke_(0, arguments.id_bits())
    }

    fn invoke_(
        &self,
        object_id: u32,
        arguments_id: u32,
    ) -> Result<Option<ObjectRef<Object>>, ObjectRef<Object>> {
        unsafe {
            let (ret, err) =
                syscall_sss_ss::<INVOKE_METHOD>(self.id_bits(), object_id, arguments_id);
            if ret == 0 {
                if err == 0 {
                    Ok(None)
                } else {
                    Err(ObjectRef::from_id_bits_unchecked(err))
                }
            } else {
                Ok(Some(ObjectRef::from_id_bits_unchecked(ret)))
            }
        }
    }
}

pub struct Field;
pub type FieldRef = ObjectRef<Field>;

pub struct Constructor;
pub type ConstructorRef = ObjectRef<Constructor>;

impl ConstructorRef {}
