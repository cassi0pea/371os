

#![allow(unconditional_recursion)]
#![no_std]
#![no_main]

// function to be called on panic, since we cant use the std one
#[panic_handler]
// Does nothing, literally just recurses forever and should never return anything.
fn panic(info: &core::panic::PanicInfo) -> ! {
    loop {}
}

// must use no_mangle because the function must be named _start *exactly*
#[unsafe(no_mangle)]

// tells the compiler that this function should use the "C calling convention", which is what the
// hardware is expecting, i think.
pub extern "C" fn _start() -> ! {
    loop {}
}

