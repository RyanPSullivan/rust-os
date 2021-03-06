#![feature(lang_items)]
#![no_std]
#![feature(unique)]
#![feature(const_fn)]

mod vga_buffer;

extern crate volatile; 
extern crate rlibc;

#[no_mangle]
pub extern "C"  fn rust_main() 
{
    // ATTENTION: we have a very small stack and no guard page

    let hello = b"Hello World!";
    let color_byte = 0x1f; // white foreground, blue background

    let mut hello_colored = [color_byte; 24];
    for (i, char_byte) in hello.into_iter().enumerate() {
        hello_colored[i*2] = *char_byte;
    }

    // write `Hello World!` to the center of the VGA text buffer
    let buffer_ptr = (0xb8000 + 1988) as *mut _;
    unsafe { *buffer_ptr = hello_colored };
    
    vga_buffer::print_something();

    loop {}
}


#[lang = "eh_personality"] 
extern "C" fn eh_personality() {}

#[lang = "panic_fmt"] 
#[no_mangle]
pub extern "C" fn panic_fmt() -> ! {
    let buffer_ptr = (0xb8000) as *mut _;
    let red = 0x4f;
    unsafe {
        *buffer_ptr = [b'P', red, b'a', red, b'n', red, b'i', red, b'c', red, b'!', red];
    };
    loop { }
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
    loop {}
}
