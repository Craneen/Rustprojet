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
    use super::allocator::SlabAllocator;
    use core::alloc::{GlobalAlloc, Layout};

    #[test]
    fn test_allocation() {
        static ALLOCATOR: SlabAllocator = SlabAllocator::new();

        unsafe {
            ALLOCATOR.init();

            let layout = Layout::from_size_align(16, 8).expect("Layout invalide");

            // Test d'allocation
            let ptr1 = ALLOCATOR.alloc(layout);
            assert!(!ptr1.is_null(), "Erreur : allocation échouée");

            // Test de libération
            ALLOCATOR.dealloc(ptr1, layout);

            // Réutilisation du bloc libéré
            let ptr2 = ALLOCATOR.alloc(layout);
            assert!(!ptr2.is_null(), "Erreur : réallocation échouée");

            // Allocation jusqu'à saturation
            for _ in 0..127 {
                let ptr = ALLOCATOR.alloc(layout);
                assert!(!ptr.is_null(), "Erreur : saturation prématurée");
            }

            // Test de saturation
            let overflow_ptr = ALLOCATOR.alloc(layout);
            assert!(overflow_ptr.is_null(), "Erreur : dépassement mémoire accepté");
        }
    }
}
