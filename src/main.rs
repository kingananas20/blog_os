#![no_std]
#![no_main]

mod vga_buffer;

static HELLO: &[u8] = b"Hello World";

// this function is the entry point, since the linker looks for a function named _start
#[no_mangle] // don't mangle the function name
pub extern "C" fn _start() -> ! {
    let vga_buffer: *mut u8 = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}

// this function is called on panic
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
