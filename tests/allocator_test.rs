#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use core::alloc::{GlobalAlloc, Layout};
use slab_allocator::allocator::SlabAllocator;
use core::panic::PanicInfo;

/// Allocateur global
#[global_allocator]
static ALLOCATOR: SlabAllocator = SlabAllocator::new();

/// Fonction de panic pour les tests
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// Runner pour les tests
fn test_runner(tests: &[&dyn Fn()]) {
    for test in tests {
        test();
    }
}

/// Point d'entrée pour les tests
#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        ALLOCATOR.init();
    }

    test_main(); // Lance les tests

    loop {}
}

/// Test d'allocation simple
#[test_case]
fn test_allocation() {
    unsafe {
        let layout = Layout::from_size_align(16, 8).expect("Layout invalide");
        let ptr1 = ALLOCATOR.alloc(layout);
        assert!(!ptr1.is_null(), "Erreur : allocation de ptr1");

        ALLOCATOR.dealloc(ptr1, layout);

        let ptr2 = ALLOCATOR.alloc(layout);
        assert!(!ptr2.is_null(), "Erreur : réallocation de ptr2");
    }
}
