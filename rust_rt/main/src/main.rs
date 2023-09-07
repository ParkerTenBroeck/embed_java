#![no_std]
#![no_main]
#![feature(const_for)]
#![feature(strict_provenance)]

#[allow(unused)]
use core::time::Duration;
pub mod asteroids;

#[allow(unused)]
use rlib::nji::{
    callback::{CallbackOnce, Callback},
    class::ClassRef,
    object::ObjectArrayRef,
    primitives::{JBooleanRef, JCharRef, JDoubleRef, JIntRef, JLongRef, JStringRef},
};
pub use rlib::*;


#[global_allocator]
static ALLOCATOR: ll_alloc::Alloc = ll_alloc::Alloc::new();

#[no_mangle]
pub fn main() {

    // test();
    //let mut bruh:u32 = black_box(0x11223344);
    // let bruh_p = black_box(core::ptr::from_exposed_addr_mut(0xFFFFFFF8));
    // unsafe{core::ptr::write_unaligned(bruh_p, 0x11223344u32)}
    // let bruh_p = black_box(bruh_p);
    // println!("{:x}", unsafe{core::ptr::read_unaligned(bruh_p)});
    // unsafe{core::ptr::write_unaligned(bruh_p, 0x99887766)}
    // println!("{:x}", unsafe{core::ptr::read_unaligned(bruh_p)});
    // unsafe{core::ptr::write(bruh_p, 0x55443322)}
    // println!("{:x}", unsafe{core::ptr::read_unaligned(bruh_p)});
    //panic!();
    // for _ in 0..20 {
    //     let start = rlib::arch::current_time_nanos();
    //     let mut vec = alloc::vec::Vec::new();

    //     for i in 0..6000 {
    //         vec.push(alloc::boxed::Box::new(i));
    //     }

    //     let end = rlib::arch::current_time_nanos();
    //     println!("{:?}", Duration::from_nanos(end - start));

    //     let lock = ALLOCATOR.lock();
    //     let allocations = lock.allocations();
    //     let htm = lock.heap_true_max();
    //     let hts = lock.heap_true_size();
    //     let pga = lock.program_memory_allocated();
    //     let wasted = lock.unused_gap_bytes();
    //     drop(lock);
    //     println!(
    //         "allocations: {}, true max: {}, true size: {}, allocated: {}, wasted: {}",
    //         allocations, htm, hts, pga, wasted
    //     );
    // }

    let mut game = asteroids::Game::new();

    // this just prints heap usage every 10 ms
    // but it can cause some stutering 
    //  use rlib::nji::callback::CallbackTrait;
    use rlib::nji::callback::CallbackTrait;
    let cb = Callback::new(|ran: u32|{
        let lock = ALLOCATOR.lock();
        let allocations = lock.allocations();
        let htm = lock.heap_true_max();
        let hts = lock.heap_true_size();
        let pga = lock.program_memory_allocated();
        let wasted = lock.unused_gap_bytes();
        drop(lock);
        println!(
            "allocations: {}, true max: {}, true size: {}, allocated: {}, wasted: {}",
            allocations, htm, hts, pga, wasted
        );
    });
    rlib::nji::callback::timer::start_peroid(Duration::from_millis(10), cb.into_jvm_obj());

    println!("briuh");
    loop {
        game.run_frame();
    }

    // println!("{:?}", vec);

    // use rlib::nji::callback::CallbackMut;
    // use rlib::nji::callback::CallbackObjTrait;
    // use rlib::nji::callback::CallbackTrait;

    // for _ in 0..500 {
    //     let mut time = crate::arch::current_time_nanos();
    //     let cb = CallbackMut::new(move |ran: u32| {
    //         let now = crate::arch::current_time_nanos();
    //         let diff = now - time;
    //         time = now;
    //         rlib::arch::print_i32(diff as i32);
    //         rlib::arch::print_char('\n');
    //         rlib::arch::print_i32(ran as i32);
    //         rlib::arch::print_char('\n');
    //     });
    //     // cb.call((23,));
    //     let mut obj = cb.into_jvm_obj();
    //     // obj.call_rust((23,));
    //     rlib::nji::callback::timer::start_peroid(Duration::from_millis(100), obj);
    // }

    // test();
}

pub fn test() {
    let mut threads = vec::Vec::new();
    if true {
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
}

//     let mut game = asteroids::Game::new();

//     loop {
//         game.run_frame();
//     }

//     for t in threads {
//         let res = t.join().unwrap();
//         println!("thread returned {:?}", res);
//     }

//     rlib::arch::halt();
//     // rlib::process::exit(0);

//     let mut vec = alloc::vec::Vec::new();
//     vec.push(JIntRef::new(54).to_obj_ref());
//     vec.push(JCharRef::new('b').to_obj_ref());
//     vec.push(JBooleanRef::new(false).to_obj_ref());
//     vec.push(JDoubleRef::new(-54.2478378).to_obj_ref());
//     vec.push(JLongRef::new(-54).to_obj_ref());

//     let class = ClassRef::for_name("java.lang.Math").unwrap();
//     let method = class
//         .get_method(
//             &"fma".into(),
//             ObjectArrayRef::from_vec(alloc::vec![
//                 JDoubleRef::primitive_class(),
//                 JDoubleRef::primitive_class(),
//                 JDoubleRef::primitive_class()
//             ]),
//         )
//         .unwrap();

//     let mut args = ObjectArrayRef::from_vec(alloc::vec![
//         JDoubleRef::new(2.0).to_obj_ref(),
//         JDoubleRef::new(4.0).to_obj_ref(),
//         JDoubleRef::new(6.0).to_obj_ref()
//     ]);

//     let ret = method.invoke_static(&mut args).unwrap();

//     let start = rlib::arch::current_time_nanos();

//     for _ in 0..50000 {
//         let ret = method.invoke_static(&mut args);
//         let ret = ret.unwrap().unwrap();
//         let ret = unsafe { JDoubleRef::from_obj_ref(ret) };
//         let ret = ret.val();
//         println!("{ret}");
//     }
//     let end = rlib::arch::current_time_nanos();
//     let dur = Duration::from_nanos(end - start);
//     println!("{:?}", dur);

//     // println!("method: {}", method);

//     println!("{:?}", JIntRef::primitive_class());
//     //println!("{:#?}",JIntRef::new(54).get_class().get_fields());
//     //panic!();

//     let class = ClassRef::for_name("java.lang.Integer").unwrap(); //turtle.get_class();
//     println!("{:#?}", class.get_fields());
//     println!("{:#?}", class.get_methods());
//     println!("{:#?}", class.get_constructors());

//     let class = ClassRef::for_name("java.awt.Color").unwrap();
//     let constructor = class
//         .get_constructor(ObjectArrayRef::from_vec(alloc::vec![
//             JIntRef::primitive_class(),
//             JIntRef::primitive_class(),
//             JIntRef::primitive_class()
//         ]))
//         .unwrap();

//     let c = constructor
//         .invoke(&mut ObjectArrayRef::from_vec(alloc::vec![
//             JIntRef::new(255).to_obj_ref(),
//             JIntRef::new(255).to_obj_ref(),
//             JIntRef::new(0).to_obj_ref()
//         ]))
//         .unwrap()
//         .unwrap();
//     println!("{c}");

//     j_frame_test();
// }

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
#[allow(unused)]
pub fn j_frame_test() {
//     let frame_class = ClassRef::for_name("javax.swing.JFrame").unwrap();
//     let panel_class = ClassRef::for_name("javax.swing.JPanel").unwrap();
//     let label_class = ClassRef::for_name("javax.swing.JLabel").unwrap();
//     let button_class = ClassRef::for_name("javax.swing.JButton").unwrap();
//     let flow_class = ClassRef::for_name("java.awt.FlowLayout").unwrap();

//     let string_class = ClassRef::for_name("java.lang.String").unwrap();

//     let title = JStringRef::from_naitive_str("JFrame (but cursed)");
//     let label = JStringRef::from_naitive_str("Rust Java button");
//     let button_name = JStringRef::from_naitive_str("BUTTON");

//     let frame_constructor = frame_class
//         .get_constructor(ObjectArrayRef::from_vec(alloc::vec![string_class]))
//         .unwrap();

//     let jframe = frame_constructor
//         .invoke(&mut ObjectArrayRef::from_vec(alloc::vec![
//             title.to_obj_ref()
//         ]))
//         .unwrap()
//         .unwrap();

//     //let javax_swing_jframe_set_size__int_int = frame_class.get_method("setSize".into(), parameters);

//     let javax_swing_jframe_set_visible__java_lang_string = frame_class
//         .get_method(
//             &JStringRef::from_naitive_str("setVisible"),
//             ObjectArrayRef::from_vec(alloc::vec![JBooleanRef::primitive_class()]),
//         )
//         .unwrap();
//     javax_swing_jframe_set_visible__java_lang_string.invoke(
//         jframe,
//         &mut ObjectArrayRef::from_vec(alloc::vec![JBooleanRef::new(true).to_obj_ref()]),
//     );
}
