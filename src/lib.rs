#![feature(lang_items)]
#![no_std]

#[no_mangle]
pub extern fn rust_main() 
{
  let x = ["Hello", "World", "!"];
  let y = x;
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() -> ! {loop{}}