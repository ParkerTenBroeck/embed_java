#![no_std]
#![no_main]
#![feature(const_for)]
#![feature(strict_provenance)]
#![feature(default_alloc_error_handler)]

pub mod alloc;
use core::time::Duration;
pub mod asteroids;

use rlib::nji::{
    class::ClassRef,
    object::ObjectArrayRef,
    primitives::{JBooleanRef, JCharRef, JDoubleRef, JIntRef, JLongRef, JStringRef},
};
pub use rlib::*;

#[no_mangle]
/// # Safety
/// No :)
pub unsafe extern "C" fn fmod(f1: f64, f2: f64) -> f64 {
    libm::fmod(f1, f2)
}

#[no_mangle]
/// # Safety
/// No :)
pub unsafe extern "C" fn fmodf(f1: f32, f2: f32) -> f32 {
    libm::fmodf(f1, f2)
}

#[no_mangle]
pub fn main() {
    rlib::nji::callback::test();
    if true {
        return;
    }

    let mut threads = vec::Vec::new();
    if false {
        for i in 0..(rlib::thread::available_parallelism() - 1) {
            let t = rlib::thread::spawn(move || {
                fn is_prime(n: u32) -> bool {
                    if n <= 1 {
                        return false;
                    }
                    for a in 2..n {
                        if n % a == 0 {
                            return false; // if it is not the last statement you need to use `return`
                        }
                    }
                    true // last value to return
                }

                for n in 0..200000 {
                    if is_prime(n) {
                        println!("Thread: {i} -> {n} is prime");
                    }
                }
                (i, i + 44)
            });
            threads.push(t.unwrap());
        }
    }

    let mut game = asteroids::Game::new();

    loop {
        game.run_frame();
    }

    for t in threads {
        let res = t.join().unwrap();
        println!("thread returned {:?}", res);
    }

    rlib::arch::halt();
    // rlib::process::exit(0);

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

    let start = rlib::arch::current_time_nanos();

    for _ in 0..50000 {
        let ret = method.invoke_static(&mut args);
        let ret = ret.unwrap().unwrap();
        let ret = unsafe { JDoubleRef::from_obj_ref(ret) };
        let ret = ret.val();
        println!("{ret}");
    }
    let end = rlib::arch::current_time_nanos();
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

    let class = ClassRef::for_name("java.awt.Color").unwrap();
    let constructor = class
        .get_constructor(ObjectArrayRef::from_vec(alloc::vec![
            JIntRef::primitive_class(),
            JIntRef::primitive_class(),
            JIntRef::primitive_class()
        ]))
        .unwrap();

    let c = constructor
        .invoke(&mut ObjectArrayRef::from_vec(alloc::vec![
            JIntRef::new(255).to_obj_ref(),
            JIntRef::new(255).to_obj_ref(),
            JIntRef::new(0).to_obj_ref()
        ]))
        .unwrap()
        .unwrap();
    println!("{c}");

    j_frame_test();
}

///
///
/// import java.awt.FlowLayout;  
/// import javax.swing.JButton;  
/// import javax.swing.JFrame;  
/// import javax.swing.JLabel;  
/// import javax.swing.JPanel;  
/// public class JFrameExample {  
///     public static void main(String s[]) {  
///         JFrame frame = new JFrame("JFrame Example");  
///         JPanel panel = new JPanel();  
///         panel.setLayout(new FlowLayout());  
///         JLabel label = new JLabel("JFrame By Example");  
///         JButton button = new JButton();  
///         button.setText("Button");  
///         panel.add(label);  
///         panel.add(button);  
///         frame.add(panel);  
///         frame.setSize(200, 300);  
///         frame.setLocationRelativeTo(null);  
///         frame.setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);  
///         frame.setVisible(true);  
///     }  
/// }  
///
pub fn j_frame_test() {
    let frame_class = ClassRef::for_name("javax.swing.JFrame").unwrap();
    let panel_class = ClassRef::for_name("javax.swing.JPanel").unwrap();
    let label_class = ClassRef::for_name("javax.swing.JLabel").unwrap();
    let button_class = ClassRef::for_name("javax.swing.JButton").unwrap();
    let flow_class = ClassRef::for_name("java.awt.FlowLayout").unwrap();

    let string_class = ClassRef::for_name("java.lang.String").unwrap();

    let title = JStringRef::from_naitive_str("JFrame (but cursed)");
    let label = JStringRef::from_naitive_str("Rust Java button");
    let button_name = JStringRef::from_naitive_str("BUTTON");

    let frame_constructor = frame_class
        .get_constructor(ObjectArrayRef::from_vec(alloc::vec![string_class]))
        .unwrap();

    let jframe = frame_constructor
        .invoke(&mut ObjectArrayRef::from_vec(alloc::vec![
            title.to_obj_ref()
        ]))
        .unwrap()
        .unwrap();

    //let javax_swing_jframe_set_size__int_int = frame_class.get_method("setSize".into(), parameters);

    let javax_swing_jframe_set_visible__java_lang_string = frame_class
        .get_method(
            &JStringRef::from_naitive_str("setVisible"),
            ObjectArrayRef::from_vec(alloc::vec![JBooleanRef::primitive_class()]),
        )
        .unwrap();
    javax_swing_jframe_set_visible__java_lang_string.invoke(
        jframe,
        &mut ObjectArrayRef::from_vec(alloc::vec![JBooleanRef::new(true).to_obj_ref()]),
    );
}
