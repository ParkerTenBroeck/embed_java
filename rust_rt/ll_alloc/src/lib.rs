#![no_std]
#![feature(strict_provenance)]

use core::{alloc::GlobalAlloc, ptr::NonNull};
use rlib::sync::{Mutex, MutexGuard};

// #[global_allocator]
// static ALLOCATOR: emballoc::Allocator<0x50000> = emballoc::Allocator::new();

pub struct Alloc {
    inner: Mutex<AllocInner>,
}

impl Alloc {
    pub fn lock(&self) -> MutexGuard<AllocInner> {
        self.inner.lock()
    }
}

unsafe impl Sync for Alloc {}
unsafe impl Send for Alloc {}

impl Alloc {
    pub const fn new() -> Self {
        Self {
            inner: Mutex::new(AllocInner::new()),
        }
    }
}
pub struct AllocInner {
    /// Number of allocations
    allocations: usize,
    /// Number of bytes allocated to actual program data
    memory_allocated: usize,
    /// The maximum size in bytes the heap can grow to
    heap_true_max: usize,
    first: Option<NonNull<Node>>,
    last: Option<NonNull<Node>>,
}

enum NodeCalc {
    CantFit,
    Tail(NonNull<Node>, *mut u8, Node),
    Insert(NonNull<Node>, *mut u8, Node),
}

unsafe fn calc_next(node: NonNull<Node>, layout: core::alloc::Layout) -> NodeCalc {
    let addr = node.addr().get();
    let addr = addr + core::mem::size_of::<Node>() + node.as_ref().size;
    let addr = (addr + layout.align() - 1) & !(layout.align() - 1);

    let next_node_start = addr - core::mem::size_of::<Node>();
    let next_node_start: *mut Node = core::ptr::from_exposed_addr_mut(next_node_start);
    let mut next_node_start = NonNull::new_unchecked(next_node_start);
    let next_size =
        (layout.size() + core::mem::size_of::<Node>() + layout.align() - 1) & !(layout.align() - 1);
    let addr = core::ptr::from_exposed_addr_mut(addr);

    if let Some(existing_next) = node.as_ref().next {
        // if the end of this node is less than the start of the next node
        // make sure they dont overlap
        if next_node_start.addr().get() + next_size < existing_next.addr().get() {
            NodeCalc::Insert(
                next_node_start,
                addr,
                Node {
                    next: Some(existing_next),
                    last: Some(node),
                    size: next_size,
                },
            )
        } else {
            return NodeCalc::CantFit;
        }
    } else {
        NodeCalc::Tail(
            next_node_start,
            addr,
            Node {
                next: None,
                last: Some(node),
                size: next_size,
            },
        )
    }
}

impl AllocInner {
    pub const fn new() -> Self {
        Self {
            first: None,
            last: None,
            allocations: 0,
            memory_allocated: 0,
            heap_true_max: 0,
        }
    }

    pub fn allocations(&self) -> usize {
        self.allocations
    }

    pub fn program_memory_allocated(&self) -> usize {
        self.memory_allocated
    }

    pub fn heap_true_max(&self) -> usize {
        self.heap_true_max
    }

    pub fn heap_true_size(&self) -> usize {
        if let Some(first) = self.first {
            if let Some(last) = self.last {
                unsafe {
                    return last.as_ref().size + last.addr().get() - first.addr().get();
                }
            } else {
                return 0;
            }
        } else {
            return 0;
        }
    }

    pub fn unused_gap_bytes(&self) -> usize{
        let mut gaps = 0;
        let mut node = self.first;
        while let Some(next) = node {
            unsafe{
                node = next.as_ref().next;
            
                if let Some(next_next) = node{
                    gaps += next_next.addr().get() - (next.addr().get() + next.as_ref().size)
                }
            }
        }
        return gaps;
    }
    
    unsafe fn allocate_infront(
        &mut self,
        mut node: NonNull<Node>,
        layout: core::alloc::Layout,
    ) -> Option<NonNull<u8>> {
        let addr = node.addr().get();
        let addr = addr + core::mem::size_of::<Node>() + node.as_ref().size;
        let addr = (addr + layout.align() - 1) & !(layout.align() - 1);

        let next_node_start = addr - core::mem::size_of::<Node>();
        let next_node_start: *mut Node = core::ptr::from_exposed_addr_mut(next_node_start);
        let mut next_node_ptr = NonNull::new_unchecked(next_node_start);
        let next_size = (layout.size() + core::mem::size_of::<Node>() + layout.align() - 1)
            & !(layout.align() - 1);
        let addr = core::ptr::from_exposed_addr_mut(addr);

        if let Some(existing_next) = node.as_ref().next {
            // if the end of this node is less than the start of the next node
            // make sure they dont overlap
            if next_node_ptr.addr().get() + next_size < existing_next.addr().get() {
                let t = Node {
                    next: Some(existing_next),
                    last: Some(node),
                    size: next_size,
                };
                next_node_ptr.as_ptr().write(t);
                if let Some(mut exist_next) = node.as_mut().next {
                    exist_next.as_mut().last = Some(next_node_ptr);
                }
                node.as_mut().next = Some(next_node_ptr);

                self.allocations += 1;
                self.memory_allocated += layout.size();
                NonNull::new(addr)
            } else {
                return None;
            }
        } else {
            let t = Node {
                next: None,
                last: Some(node),
                size: next_size,
            };
            next_node_ptr.as_ptr().write(t);
            node.as_mut().next = Some(next_node_ptr);
            self.last = Some(next_node_ptr);

            self.allocations += 1;
            self.memory_allocated += layout.size();
            NonNull::new(addr)
        }
    }
}

unsafe impl GlobalAlloc for Alloc {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let lock = &mut *self.inner.lock();

        let layout = core::alloc::Layout::from_size_align_unchecked(
            layout.size(),
            usize::max(layout.align(), core::mem::align_of::<Node>()),
        );

        if lock.first.is_none() {
            let heap_start_align = 0x10000;
            let addr = rlib::rt::heap_address();
            let addr = addr as usize;
            let addr = addr + core::mem::size_of::<Node>();
            let addr = (addr + heap_start_align - 1) & !(heap_start_align - 1);
            let heap_start = addr - core::mem::size_of::<Node>();
            let heap_start: *mut Node = core::ptr::from_exposed_addr_mut(heap_start);
            let mut heap_start = NonNull::new_unchecked(heap_start);
            heap_start.as_mut().next = None;
            heap_start.as_mut().last = None;
            heap_start.as_mut().size = core::mem::size_of::<Node>();
            lock.first = Some(heap_start);
            lock.last = Some(heap_start);
        }

        let mut next = lock.first;
        while let Some(mut node) = next {
            // if let Some(allocation) = lock.allocate_infront(node, layout) {
            //     return allocation.as_ptr();
            // }

            match calc_next(node, layout) {
                NodeCalc::CantFit => {}
                NodeCalc::Tail(next_node_ptr, data_addr, next_node_data) => {
                    next_node_ptr.as_ptr().write(next_node_data);
                    node.as_mut().next = Some(next_node_ptr);
                    lock.last = Some(next_node_ptr);

                    lock.allocations += 1;
                    lock.memory_allocated += layout.size();
                    return data_addr;
                }
                NodeCalc::Insert(next_node_ptr, data_addr, next_node_data) => {
                    next_node_ptr.as_ptr().write(next_node_data);
                    if let Some(mut exist_next) = node.as_mut().next {
                        exist_next.as_mut().last = Some(next_node_ptr);
                    }
                    node.as_mut().next = Some(next_node_ptr);

                    lock.allocations += 1;
                    lock.memory_allocated += layout.size();
                    return data_addr;
                }
            }
            next = node.as_ref().next;
        }
        panic!("Failed to find a spot to fit node??? bruh");
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        let lock = &mut *self.inner.lock();
        let node_start = ptr.sub(core::mem::size_of::<Node>());
        let node_start: *mut Node = core::mem::transmute(node_start);

        let last = &mut node_start.as_mut().unwrap_unchecked().last;
        lock.allocations -= 1;
        lock.memory_allocated -= layout.size();
        if let Some(last) = last {
            last.as_mut().next = node_start.as_mut().unwrap_unchecked().next;
            match last.as_mut().next {
                Some(mut next) => next.as_mut().last = Some(*last),
                None => {
                    lock.last = Some(*last);
                }
            }
        } else {
            panic!();
        }

        // crate::println!("dealloc {:?}, {:?}",node_start, node_start.as_ref().unwrap_unchecked());
    }
}

#[derive(Debug)]
struct Node {
    next: Option<NonNull<Node>>,
    last: Option<NonNull<Node>>,
    size: usize,
}
