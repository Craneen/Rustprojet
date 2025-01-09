use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;
use core::cell::UnsafeCell;

/// Taille d'un bloc mémoire
const BLOCK_SIZE: usize = 32;
/// Nombre total de blocs
const NUM_BLOCKS: usize = 128;

/// Zone mémoire
struct Heap {
    memory: UnsafeCell<[u8; BLOCK_SIZE * NUM_BLOCKS]>,
}

unsafe impl Sync for Heap {}

static HEAP: Heap = Heap {
    memory: UnsafeCell::new([0; BLOCK_SIZE * NUM_BLOCKS]),
};

/// Représentation d'un bloc libre
struct FreeBlock {
    next: Option<&'static mut FreeBlock>,
}

/// Allocateur Slab
pub struct SlabAllocator {
    free_list: UnsafeCell<Option<&'static mut FreeBlock>>,
}

unsafe impl Sync for SlabAllocator {}

impl SlabAllocator {
    pub const fn new() -> Self {
        SlabAllocator {
            free_list: UnsafeCell::new(None),
        }
    }

    pub unsafe fn init(&self) {
        let mut prev: Option<&'static mut FreeBlock> = None;

        for i in 0..NUM_BLOCKS {
            let block_ptr = (*HEAP.memory.get()).as_mut_ptr().add(i * BLOCK_SIZE) as *mut FreeBlock;
            let block = &mut *block_ptr;
            block.next = prev;
            prev = Some(block);
        }

        *self.free_list.get() = prev;
    }
}

unsafe impl GlobalAlloc for SlabAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if layout.size() > BLOCK_SIZE {
            return null_mut();
        }

        let free_list = &mut *self.free_list.get();

        if let Some(block) = free_list.take() {
            *free_list = block.next.take();
            block as *mut FreeBlock as *mut u8
        } else {
            null_mut()
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        let block = ptr as *mut FreeBlock;

        let free_list = &mut *self.free_list.get();
        (*block).next = free_list.take();
        *free_list = Some(&mut *block);
    }
}
