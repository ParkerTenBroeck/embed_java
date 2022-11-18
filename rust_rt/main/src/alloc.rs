extern crate alloc;
use core::{alloc::GlobalAlloc, ptr::NonNull};
use rlib::sync::Mutex;

pub use alloc::*;

// #[global_allocator]
// static ALLOCATOR: emballoc::Allocator<0x50000> = emballoc::Allocator::new();


#[global_allocator]
static ALLOCATOR: Alloc = Alloc::new();

struct Alloc{
    inner: Mutex<AllocInner>
}

unsafe impl Sync for Alloc{}

unsafe impl Send for Alloc{}

impl Alloc{
    pub const fn new() -> Self{
        Self 
        { 
            inner: Mutex::new(AllocInner::new())
        }
    }
}
struct AllocInner{
    allocations: usize,
    memory_allocated: usize,
    first: Option<NonNull<Node>>,
    last: Option<NonNull<Node>>
}

impl AllocInner{
    pub const fn new() -> Self{
        Self{
            first: None,
            last: None,
            allocations: 0,
            memory_allocated: 0,
        }
    }
}


unsafe impl GlobalAlloc for Alloc{
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let lock = &mut *self.inner.lock();


        let mut align = layout.align();
        if align < core::mem::align_of::<Node>(){
            align = core::mem::align_of::<Node>();
        };

        match lock.first{
            Some(mut node) => {
                loop{
                    if let Some(mut next) = node.as_mut().next{

                        let addr = next.addr().get();
                        let addr = addr + core::mem::size_of::<Node>() + next.as_mut().size;
                        let addr = (addr + align - 1) & !(align - 1); 

                        let node_start = addr - core::mem::size_of::<Node>();
                        let size = (layout.size() + core::mem::size_of::<Node>() + align - 1) & !(align - 1);

                        if let Some(mut next_next) = next.as_mut().next{
                            if node_start + size < next_next.addr().get(){

                                let node_start:*mut Node = core::ptr::from_exposed_addr_mut(node_start);
                                let mut node_start = NonNull::new_unchecked(node_start);

                                next_next.as_mut().last = Some(node_start);

                                node_start.as_mut().next = Some(next_next);
                                node_start.as_mut().last = Some(next);
                                node_start.as_mut().size = (layout.size() + core::mem::size_of::<Node>() + align - 1) & !(align - 1); 

                                next.as_mut().next = Some(node_start);


                                let addr = core::ptr::from_exposed_addr_mut(addr);
                                // crate::println!("3: {:?}, {:?}",&node_start, node_start.as_ref());
                                lock.memory_allocated += layout.size();
                                lock.allocations += 1;
                                return addr;
                            } 
                        }
                        


                        node = next;
                    }else{
                        let addr = node.addr().get();
                        let addr = addr + core::mem::size_of::<Node>() + node.as_mut().size;
                        let addr = (addr + align - 1) & !(align - 1); 

                        let node_start = addr - core::mem::size_of::<Node>();
                        let node_start:*mut Node = core::ptr::from_exposed_addr_mut(node_start);
                        let mut node_start = NonNull::new_unchecked(node_start);

                        node_start.as_mut().next = None;
                        node_start.as_mut().last = Some(node);
                        node_start.as_mut().size = (layout.size() + core::mem::size_of::<Node>() + align - 1) & !(align - 1); 

                        node.as_mut().next = Some(node_start);


                        let addr = core::ptr::from_exposed_addr_mut(addr);
                        // crate::println!("2: {:?}, {:?}",&node_start, node_start.as_ref());
                        lock.memory_allocated += layout.size();
                        lock.allocations += 1;
                        return addr;
                    }
                }
            }
            None => {
                let addr = rlib::rt::heap_address();
                let addr = addr as usize;
                let addr = addr + core::mem::size_of::<Node>();
                let addr = (addr + align - 1) & !(align - 1); 
                let heap_start = addr - core::mem::size_of::<Node>();

                let heap_start:*mut Node = core::ptr::from_exposed_addr_mut(heap_start);
                let mut heap_start = NonNull::new_unchecked(heap_start);
                
                heap_start.as_mut().next = None;
                heap_start.as_mut().last = None;
                heap_start.as_mut().size = (layout.size() + core::mem::size_of::<Node>() + align - 1) & !(align - 1); 
                
                lock.first = Some(heap_start);

                let addr = core::ptr::from_exposed_addr_mut(addr);
                // crate::println!("1: {:?}, {:?}",&heap_start, heap_start.as_ref());
                lock.memory_allocated += layout.size();
                lock.allocations += 1;
                return addr;
            }
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: core::alloc::Layout) {
        let lock = &mut *self.inner.lock();
        let node_start = ptr.sub(core::mem::size_of::<Node>());
        let node_start: *mut Node = core::mem::transmute(node_start);

        let last = &mut node_start.as_mut().unwrap_unchecked().last;
        lock.allocations -= 1;
        lock.memory_allocated -= layout.size();
        if let Some(last) = last{
            last.as_mut().next = node_start.as_mut().unwrap_unchecked().next;
        }else{
            lock.first = node_start.as_mut().unwrap_unchecked().next;
        }

        // crate::println!("dealloc {:?}, {:?}",node_start, node_start.as_ref().unwrap_unchecked());
    }
}

#[derive(Debug)]
struct Node{
    next: Option<NonNull<Node>>,
    last: Option<NonNull<Node>>,
    size: usize,
}


// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// use core::{
//     alloc::{GlobalAlloc, Layout},
//     cell::UnsafeCell,
//     fmt::Debug,
//     mem::{align_of, size_of},
//     ptr::{self, NonNull},
// };

// #[repr(C)]
// struct AllocatedMemory {
//     size: usize,
//     align: usize,
//     data: *mut u8,
//     next: Option<NonNull<AllocatedMemory>>,
// }

// impl Debug for AllocatedMemory {
//     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
//         f.debug_struct("AllocatedMemory")
//             .field("_start_", &(self as *const Self))
//             .field("size", &self.size)
//             .field("align", &self.align)
//             .field("data", &self.data)
//             .field("next", &self.next)
//             .field("_end_", unsafe { &self.alligned_end() })
//             .finish()
//     }
// }

// impl AllocatedMemory {
//     unsafe fn alligned_end(&self) -> *mut u8 {
//         self.data.map_addr(|mut end| {
//             end += self.size;
//             end += align_of::<&Self>() - 1;
//             end &= !(align_of::<&Self>() - 1);
//             end
//         })
//     }

//     #[allow(unused)]
//     unsafe fn calc_free_space_ahead(&self) -> usize {
//         if let Option::Some(next) = self.next {
//             next.as_ptr().addr() - self.alligned_end().addr()
//         } else {
//             usize::MAX - (self.data.addr() + self.size)
//         }
//     }
// }
// struct AllocatedMemoryIterator {
//     current: Option<NonNull<AllocatedMemory>>,
// }
// impl Iterator for AllocatedMemoryIterator {
//     type Item = NonNull<AllocatedMemory>;

//     fn next(&mut self) -> Option<Self::Item> {
//         unsafe {
//             let curr = self.current;
//             if let Option::Some(curr) = curr {
//                 self.current = (*curr.as_ptr()).next;
//             }
//             curr
//         }
//     }
// }

// struct SimpleAccocator {
//     head: UnsafeCell<*mut AllocatedMemory>,
// }
// #[global_allocator]
// static ALLOCATOR: SimpleAccocator = SimpleAccocator {
//     head: UnsafeCell::new(0 as *mut AllocatedMemory),
// };
// unsafe impl Send for SimpleAccocator {}
// unsafe impl Sync for SimpleAccocator {}

// unsafe impl GlobalAlloc for SimpleAccocator {
//     unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
//         if (*self.head.get()).is_null() {
//             *self.head.get() = core::mem::transmute(interface::heap_address());
//             let head = *self.head.get();
//             Self::add_new(head, &layout);
//             (*head).data
//         } else {
//             let iter = AllocatedMemoryIterator {
//                 current: Option::Some(NonNull::new_unchecked(*self.head.get())),
//             };
//             for alloc in iter {
//                 let mut start = (*alloc.as_ptr()).alligned_end().addr();
//                 start += size_of::<AllocatedMemory>();
//                 start += layout.align() - 1;
//                 start &= !(layout.align() - 1);
//                 if let Option::Some(next) = (*alloc.as_ptr()).next {
//                     if start < next.addr().into() {
//                         let new = (*alloc.as_ptr()).alligned_end();
//                         let new: *mut AllocatedMemory = core::mem::transmute(new);
//                         Self::add_new(new, &layout);
//                         (*new).next = (*alloc.as_ptr()).next;
//                         (*alloc.as_ptr()).next = Option::Some(NonNull::new_unchecked(new));
//                         return (*new).data;
//                     } else {
//                         continue;
//                     }
//                 } else {
//                     let new = (*alloc.as_ptr()).alligned_end();
//                     let new: *mut AllocatedMemory = core::mem::transmute(new);
//                     Self::add_new(new, &layout);
//                     (*alloc.as_ptr()).next = Option::Some(NonNull::new_unchecked(new));
//                     return (*new).data;
//                 }
//             }
//             ptr::null_mut()
//         }
//     }

//     unsafe fn dealloc(&self, ptr: *mut u8, _layout: core::alloc::Layout) {
//         if (*self.head.get()).is_null() {
//             panic!();
//         } else {
//             let mut iter = AllocatedMemoryIterator {
//                 current: Option::Some(NonNull::new_unchecked(*self.head.get())),
//             };
//             let mut last = iter.next().unwrap();
//             if (*last.as_ptr()).data == ptr {
//                 (*self.head.get()) = match (*last.as_ptr()).next {
//                     Some(some) => some.as_ptr(),
//                     None => ptr::null_mut(),
//                 };
//                 return;
//             }
//             for alloc in iter {
//                 if (*alloc.as_ptr()).data == ptr {
//                     (*last.as_ptr()).next = (*alloc.as_ptr()).next;
//                     return;
//                 } else {
//                     last = alloc;
//                 }
//             }
//         }
//         panic!("to free: {:?}\nhead: {:?}", ptr, (**self.head.get()));
//     }
// }
// impl SimpleAccocator {
//     unsafe fn add_new(alloc: *mut AllocatedMemory, layout: &Layout) {
//         (*alloc).align = layout.align();
//         (*alloc).size = layout.size();
//         (*alloc).next = Option::None;
//         (*alloc).data = core::mem::transmute(alloc.map_addr(|mut add| {
//             let align_mask_to_round_down = !(layout.align() - 1);
//             add += size_of::<AllocatedMemory>();
//             add += layout.align() - 1;
//             add & align_mask_to_round_down
//         }));
//     }
//     #[allow(unused)]
//     unsafe fn count_allocations(&self) -> usize {
//         if self.head.get().is_null() {
//             0
//         } else {
//             let iter = AllocatedMemoryIterator {
//                 current: Option::Some(NonNull::new_unchecked(*self.head.get())),
//             };
//             iter.count()
//         }
//     }
// }
