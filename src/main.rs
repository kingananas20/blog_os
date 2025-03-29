#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use blog_os::{hlt_loop, println, vga_buffer::Color, vga_set_color};
use bootloader::{entry_point, BootInfo};
use x86_64::structures::paging::Page;

entry_point!(kernel_main);

// this function is the entry point, since the linker looks for a function named _start
#[no_mangle] // don't mangle the function name
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use blog_os::memory;
    use x86_64::VirtAddr;
    blog_os::init();

    #[cfg(test)]
    test_main();

    println!("Hello World!");

    println!("It did not crash");
    hlt_loop()
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
