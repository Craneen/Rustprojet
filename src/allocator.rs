use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;
use core::cell::UnsafeCell;

/// Taille d'un bloc mémoire
const BLOCK_SIZE: usize = 32;
/// Nombre total de blocs
pub const NUM_BLOCKS: usize = 128;

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
///
/// Cet allocateur utilise des blocs de taille fixe.
/// Les fonctions `alloc` et `dealloc` permettent de gérer
/// la mémoire tout en assurant un alignement correct.
pub struct SlabAllocator {
    free_list: UnsafeCell<Option<&'static mut FreeBlock>>,
    free_count: UnsafeCell<usize>, // Compteur des blocs libres
}

unsafe impl Sync for SlabAllocator {}

impl SlabAllocator {
    /// Crée un nouvel allocateur
    pub const fn new() -> Self {
        SlabAllocator {
            free_list: UnsafeCell::new(None),
            free_count: UnsafeCell::new(0),
        }
    }

    /// Initialise l'allocateur en créant une liste chaînée de blocs libres
    ///
    /// # Safety
    /// - Cette fonction doit être appelée une seule fois avant toute utilisation.
    /// - L'accès direct à la mémoire brute via `UnsafeCell` peut provoquer des comportements
    ///   indéfinis si mal utilisé. Il est crucial de s'assurer qu'aucun autre accès simultané
    ///   n'a lieu lors de l'initialisation.
    pub unsafe fn init(&self) {
        let mut prev: Option<&'static mut FreeBlock> = None;

        for i in 0..NUM_BLOCKS {
            let block_ptr = (*HEAP.memory.get()).as_mut_ptr().add(i * BLOCK_SIZE) as *mut FreeBlock;
            let block = &mut *block_ptr;
            block.next = prev;
            prev = Some(block);
        }

        *self.free_list.get() = prev;
        *self.free_count.get() = NUM_BLOCKS;
    }

    /// Retourne le nombre de blocs libres
    pub fn free_count(&self) -> usize {
        unsafe { *self.free_count.get() }
    }
}

unsafe impl GlobalAlloc for SlabAllocator {
    /// Alloue un bloc mémoire en fonction du layout
    ///
    /// # Arguments
    /// - `layout` : Layout spécifiant la taille et l'alignement requis.
    ///
    /// # Safety
    /// - L'utilisateur doit s'assurer que le layout est valide.
    /// - Le pointeur retourné ne doit pas être utilisé après avoir été libéré.
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if layout.size() > BLOCK_SIZE || *self.free_count.get() == 0 {
            return null_mut();
        }

        let free_list = &mut *self.free_list.get();

        if let Some(block) = free_list.take() {
            *free_list = block.next.take();
            *self.free_count.get() -= 1;
            block as *mut FreeBlock as *mut u8
        } else {
            null_mut()
        }
    }

    /// Libère un bloc mémoire en le réinsérant dans la liste chaînée
    ///
    /// # Safety
    /// - Le pointeur `ptr` doit avoir été alloué par cet allocateur.
    /// - Le `layout` passé doit correspondre au layout utilisé pour l'allocation.
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        let block = ptr as *mut FreeBlock;

        let free_list = &mut *self.free_list.get();
        (*block).next = free_list.take();
        *free_list = Some(&mut *block);
        *self.free_count.get() += 1;
    }
    
}