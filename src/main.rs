#![no_std]
#![no_main]

mod vga_buffer;

// this function is the entry point, since the linker looks for a function named _start
#[no_mangle] // don't mangle the function name
pub extern "C" fn _start() -> ! {
    println!("Hello World");
    println!("This should be on the second line");

    loop {}
}

// this function is called on panic
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
