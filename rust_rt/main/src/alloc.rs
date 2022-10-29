extern crate alloc;
pub use alloc::*;

extern crate wee_alloc;

// #[global_allocator]
// static ALLOCATOR: emballoc::Allocator<0x80000> = emballoc::Allocator::new();

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

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
