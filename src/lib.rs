#![no_std]
#![feature(allocator_api, alloc_error_handler)]

// Déclaration du module de l'allocateur
pub mod allocator;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
