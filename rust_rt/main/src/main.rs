#![no_std]
#![no_main]
#![feature(const_for)]
#![feature(strict_provenance)]
#![feature(default_alloc_error_handler)]

pub mod alloc;
pub mod panic_handler;
use core::{hint::black_box, time::Duration};

use rlib::nji::{
    class::ClassRef,
    object::ObjectArrayRef,
    primitives::{JBooleanRef, JCharRef, JDoubleRef, JIntRef, JLongRef},
};
pub use rlib::*;

#[no_mangle]
pub fn main() {
    let mut vec = alloc::vec::Vec::new();
    vec.push(JIntRef::new(54).to_obj_ref());
    vec.push(JCharRef::new('b').to_obj_ref());
    vec.push(JBooleanRef::new(false).to_obj_ref());
    vec.push(JDoubleRef::new(-54.2478378).to_obj_ref());
    vec.push(JLongRef::new(-54).to_obj_ref());

    let class = ClassRef::for_name("java.lang.Math").unwrap();
    let method = class
        .get_method(
            &"fma".into(),
            ObjectArrayRef::from_vec(alloc::vec![
                JDoubleRef::primitive_class(),
                JDoubleRef::primitive_class(),
                JDoubleRef::primitive_class()
            ]),
        )
        .unwrap();

    let mut args = ObjectArrayRef::from_vec(alloc::vec![
        JDoubleRef::new(2.0).to_obj_ref(),
        JDoubleRef::new(4.0).to_obj_ref(),
        JDoubleRef::new(6.0).to_obj_ref()
    ]);


    let ret = method.invoke_static(&mut args).unwrap();

    let start = rlib::sys::current_time_nanos();

    for _ in 0..50000 {
        let ret = method.invoke_static(&mut args);
        let ret = ret.unwrap().unwrap();
        let ret = unsafe { JDoubleRef::from_obj_ref(ret) };
        let ret = ret.val();
        black_box(ret);
        //println!("{ret}");
    }
    let end = rlib::sys::current_time_nanos();
    let dur = Duration::from_nanos(end - start);
    println!("{:?}", dur);

    // println!("method: {}", method);

    println!("{:?}", JIntRef::primitive_class());
    //println!("{:#?}",JIntRef::new(54).get_class().get_fields());
    //panic!();

    let class = ClassRef::for_name("java.lang.Integer").unwrap(); //turtle.get_class();
    println!("{:#?}", class.get_fields());
    println!("{:#?}", class.get_methods());
    println!("{:#?}", class.get_constructors());
}
