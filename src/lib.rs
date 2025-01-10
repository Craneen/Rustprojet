#![cfg_attr(not(test), no_std)]
#![feature(allocator_api, alloc_error_handler)]

pub mod allocator;

#[cfg(not(test))]
use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[cfg(test)]
mod tests {
    use super::allocator::{SlabAllocator, NUM_BLOCKS};
    use core::alloc::{GlobalAlloc, Layout};

    // Test d'une allocation simple
    #[test]
    fn test_allocation_simple() {
        static ALLOCATOR: SlabAllocator = SlabAllocator::new();

        unsafe {
            ALLOCATOR.init();

            let layout = Layout::from_size_align(16, 8).expect("Layout invalide");

            // Allocation simple
            let ptr = ALLOCATOR.alloc(layout);
            assert!(!ptr.is_null(), "Erreur : allocation échouée");
        }
    }

    // Test de désallocation d'un bloc
    #[test]
    fn test_deallocation() {
        static ALLOCATOR: SlabAllocator = SlabAllocator::new();

        unsafe {
            ALLOCATOR.init();

            let layout = Layout::from_size_align(16, 8).expect("Layout invalide");

            // Allocation et désallocation
            let ptr = ALLOCATOR.alloc(layout);
            assert!(!ptr.is_null(), "Erreur : allocation échouée");

            ALLOCATOR.dealloc(ptr, layout);
        }
    }

    // Test de réutilisation d'un bloc libéré
    #[test]
    fn test_reuse_freed_block() {
        static ALLOCATOR: SlabAllocator = SlabAllocator::new();

        unsafe {
            ALLOCATOR.init();

            let layout = Layout::from_size_align(16, 8).expect("Layout invalide");

            let ptr1 = ALLOCATOR.alloc(layout);
            assert!(!ptr1.is_null(), "Erreur : allocation échouée");

            // Libérer le premier bloc
            ALLOCATOR.dealloc(ptr1, layout);

            // Réallouer
            let ptr2 = ALLOCATOR.alloc(layout);
            assert!(!ptr2.is_null(), "Erreur : réutilisation échouée");
        }
    }

    // Test d'allocation jusqu'à saturation
    #[test]
    fn test_allocation_to_saturation() {
        static ALLOCATOR: SlabAllocator = SlabAllocator::new();

        unsafe {
            ALLOCATOR.init();

            let layout = Layout::from_size_align(16, 8).expect("Layout invalide");

            // Allocation jusqu’à la saturation
            for _ in 0..NUM_BLOCKS {
                let ptr = ALLOCATOR.alloc(layout);
                assert!(!ptr.is_null(), "Erreur : saturation prématurée");
            }
        }
    }

    // Test du refus d'allocation en cas de saturation
    #[test]
    fn test_allocation_refusal_on_saturation() {
        static ALLOCATOR: SlabAllocator = SlabAllocator::new();

        unsafe {
            ALLOCATOR.init();

            let layout = Layout::from_size_align(16, 8).expect("Layout invalide");

            // Saturer l'allocateur
            for _ in 0..NUM_BLOCKS {
                let ptr = ALLOCATOR.alloc(layout);
                assert!(!ptr.is_null(), "Erreur : saturation prématurée");
            }

            // La prochaine allocation doit échouer
            let ptr = ALLOCATOR.alloc(layout);
            assert!(ptr.is_null(), "Erreur : dépassement mémoire accepté");
        }
    }
}
