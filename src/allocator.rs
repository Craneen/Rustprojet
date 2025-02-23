use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;
use core::cell::UnsafeCell;

/// Taille d'un bloc mémoire
const BLOCK_SIZE: usize = 32;
/// Nombre total de blocs
pub const NUM_BLOCKS: usize = 128;

/// Zone mémoire avec alignement explicite
#[repr(align(8))]
struct Heap {
    memory: UnsafeCell<[u8; BLOCK_SIZE * NUM_BLOCKS]>,
}



// Implémentation de `Sync` pour `Heap`
unsafe impl Sync for Heap {}

/// Zone mémoire statique sécurisée
static HEAP: Heap = Heap {
    memory: UnsafeCell::new([0; BLOCK_SIZE * NUM_BLOCKS]),
};

/// Représentation d'un bloc libre
struct FreeBlock {
    next: Option<&'static mut FreeBlock>,
}

/// Allocateur Slab sécurisé
///
/// Cet allocateur utilise des blocs de taille fixe.
/// Les fonctions `alloc` et `dealloc` permettent de gérer
/// la mémoire tout en assurant un alignement correct.
pub struct SlabAllocator {
    free_list: UnsafeCell<Option<&'static mut FreeBlock>>,
    free_count: UnsafeCell<usize>, // Compteur des blocs libres
}

// Implémentation de `Sync` pour `SlabAllocator`
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
        let memory_ptr = (*HEAP.memory.get()).as_mut_ptr();
    
        // Pointeur initial pour le précédent bloc
        let mut prev: *mut FreeBlock = core::ptr::null_mut();
    
        for i in 0..NUM_BLOCKS {
            // Calcul d'un pointeur correctement aligné
            let block_ptr = memory_ptr.add(i * BLOCK_SIZE).cast::<FreeBlock>();
    
            // Initialisation du bloc
            let block = block_ptr.as_mut().expect("Pointeur nul détecté lors de l'initialisation");
    
            // Chaînage des blocs libres
            block.next = if prev.is_null() { None } else { Some(&mut *prev) };
    
            // Mettre à jour le pointeur précédent
            prev = block_ptr;
        }
    
        // Mettre à jour la liste chaînée et le compteur
        *self.free_list.get() = if prev.is_null() { None } else { Some(&mut *prev) };
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
    /// - Le pointeur retourné ne doit pas être utilisé après avoir été libéré.
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if layout.size() > BLOCK_SIZE || *self.free_count.get() == 0 {
            return null_mut();
        }
    
        let free_list = &mut *self.free_list.get();
        let mut current = free_list.take();
        let mut previous: Option<*mut FreeBlock> = None;
    
        while let Some(block) = current {
            // Vérifier si ce bloc convient
            if BLOCK_SIZE >= layout.size() {
                // Met à jour le lien dans la liste chaînée
                if let Some(prev) = previous {
                    (*prev).next = block.next.take();
                } else {
                    *free_list = block.next.take();
                }
    
                *self.free_count.get() -= 1;
                return block as *mut FreeBlock as *mut u8;
            }
    
            // Passer au bloc suivant
            previous = Some(block as *mut FreeBlock);
            current = block.next.take();
        }
    
        null_mut() // Aucun bloc trouvé
    }
    

    /// Libère un bloc mémoire en le réinsérant dans la liste chaînée
    ///
    /// # Safety
    /// - Le pointeur `ptr` doit avoir été alloué par cet allocateur.
    /// - Le `layout` passé doit correspondre au layout utilisé pour l'allocation.
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        let block = ptr as *mut FreeBlock;
    
        // Vérifier si le bloc a déjà été désalloué
        if (*block).next.is_some() {
            panic!("Erreur : tentative de double désallocation détectée !");
        }
    
        // Accéder à la liste des blocs libres
        let free_list = &mut *self.free_list.get();
    
        // Ajouter le bloc à la liste des blocs libres
        (*block).next = free_list.take();
        *free_list = Some(&mut *block);
    
        // Mettre à jour le compteur des blocs libres
        *self.free_count.get() += 1;
    }
    
    
        
}