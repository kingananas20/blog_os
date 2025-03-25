#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use blog_os::{println, vga_buffer::Color, vga_set_color};

// this function is the entry point, since the linker looks for a function named _start
#[no_mangle] // don't mangle the function name
pub extern "C" fn _start() -> ! {
    blog_os::init();

    #[cfg(test)]
    test_main();

    println!("Hello World!");

    fn stack_overflow() {
        stack_overflow();
    }

    stack_overflow();

    println!("It did not crash");
    loop {}
}

// this function is called on panic
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    vga_set_color!(Color::Red, Color::Black);
    println!("{}", info);
    loop {}
}

//--------------------------------[testing]--------------------------------//

// panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}
