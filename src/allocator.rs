// Auteur : Samuel Ktorza

use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;

/// Taille d'un bloc mémoire
const BLOCK_SIZE: usize = 32;
/// Nombre total de blocs
const NUM_BLOCKS: usize = 128;

/// Espace mémoire statique
static mut HEAP: [u8; BLOCK_SIZE * NUM_BLOCKS] = [0; BLOCK_SIZE * NUM_BLOCKS];

/// Définition d'un bloc libre
struct FreeBlock {
    next: Option<&'static mut FreeBlock>,
}

/// L'allocateur Slab
pub struct SlabAllocator {
    free_list: Option<&'static mut FreeBlock>,
}

impl SlabAllocator {
    /// Création d'un nouvel allocateur
    pub fn new() -> Self {
        SlabAllocator { free_list: None }
    }

    /// Initialisation de la liste des blocs libres
    pub unsafe fn init(&mut self) {
        let mut prev: Option<&'static mut FreeBlock> = None;

        for i in 0..NUM_BLOCKS {
            let block_ptr = HEAP.as_mut_ptr().add(i * BLOCK_SIZE) as *mut FreeBlock;
            let block = &mut *block_ptr;
            block.next = prev;
            prev = Some(block);
        }

        self.free_list = prev;
    }
}
