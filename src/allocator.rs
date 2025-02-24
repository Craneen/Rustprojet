// Slab Allocator en Rust
//
// Cet allocateur implémente une gestion mémoire basée sur des blocs fixes. Il permet
// des opérations d'allocation et de désallocation rapides en utilisant un algorithme
// de premier ajustement (First Fit) pour minimiser la fragmentation mémoire.
//
// L'allocateur est conçu pour fonctionner en mode no_std et offre une robustesse
// face aux erreurs telles que la double désallocation et l'utilisation après libération.


use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;
use core::cell::UnsafeCell;

/// Taille fixe d'un bloc mémoire en octets.
const BLOCK_SIZE: usize = 32;
/// Nombre total de blocs disponibles dans l'allocateur.
pub const NUM_BLOCKS: usize = 128;

/// Zone mémoire avec alignement explicite
#[repr(align(8))]
struct Heap {
    memory: UnsafeCell<[u8; BLOCK_SIZE * NUM_BLOCKS]>,
}


unsafe impl Sync for Heap {}

/// Représente la mémoire brute utilisée par l'allocateur.
///
/// Cette structure utilise un alignement explicite pour garantir que les blocs
/// respectent les contraintes d'alignement nécessaires pour les opérations d'allocation.

static HEAP: Heap = Heap {
    memory: UnsafeCell::new([0; BLOCK_SIZE * NUM_BLOCKS]),
};

/// Représente un bloc libre dans la liste chaînée de l'allocateur.
///
/// Chaque bloc libre pointe vers le suivant dans la chaîne.
struct FreeBlock {
    next: Option<&'static mut FreeBlock>,
}

/// Allocateur Slab utilisant une gestion basée sur des blocs fixes.
///
/// L'allocateur maintient une liste chaînée des blocs libres et un compteur
/// pour suivre le nombre de blocs disponibles. Il est conçu pour être utilisé
/// dans des environnements minimalistes avec une gestion explicite de la mémoire.
pub struct SlabAllocator {
    free_list: UnsafeCell<Option<&'static mut FreeBlock>>,
    free_count: UnsafeCell<usize>, // Compteur des blocs libres
}

unsafe impl Sync for SlabAllocator {}

impl SlabAllocator {
    /// Crée une nouvelle instance de l'allocateur avec une liste de blocs libres non initialisée.
    ///
    /// Cette méthode ne fait que définir la structure ; l'initialisation des blocs doit
    /// être effectuée explicitement avec init.

    pub const fn new() -> Self {
        SlabAllocator {
            free_list: UnsafeCell::new(None),
            free_count: UnsafeCell::new(0),
        }
    }

    /// Initialise l'allocateur en créant une liste chaînée de blocs libres.
    ///
    /// # Safety
    /// - Cette fonction doit être appelée une seule fois avant toute opération d'allocation ou de désallocation.
    /// - Les accès directs à la mémoire brute via UnsafeCell doivent être effectués
    ///   avec précaution pour éviter les comportements indéfinis.

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
    
        
    /// Retourne le nombre de blocs libres actuellement disponibles.
    ///
    /// Ce compteur est mis à jour automatiquement lors des opérations
    /// d'allocation et de désallocation.

    pub fn free_count(&self) -> usize {
        unsafe { *self.free_count.get() }
    }
}

unsafe impl GlobalAlloc for SlabAllocator {
    /// Alloue un bloc mémoire en fonction du layout spécifié.
    ///
    /// # Arguments
    /// - layout : Décrit la taille et l'alignement requis pour l'allocation.
    ///
    /// # Safety
    /// - Le pointeur retourné ne doit pas être utilisé après avoir été libéré.
    /// - Le layout doit respecter les contraintes de taille et d'alignement
    ///   définies par l'allocateur.
    ///
    /// # Retour
    /// - Un pointeur valide vers le bloc alloué, ou un pointeur nul si aucune mémoire n'est disponible.

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
    
    /// Libère un bloc mémoire précédemment alloué et le réinsère dans la liste des blocs libres.
    ///
    /// # Arguments
    /// - ptr : Pointeur vers le bloc à libérer.
    /// - layout : Layout utilisé lors de l'allocation.
    ///
    /// # Safety
    /// - Le pointeur doit avoir été retourné par une allocation réussie.
    /// - Cette fonction panique si une tentative de double désallocation est détectée.

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