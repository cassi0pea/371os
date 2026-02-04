

// Prevents cargo from linking the standard library. For character.
#![no_std]

// Generally, before rust runs your program, it does some startup stuff, which relies on C. That C
// code then uses main() as the entry point for your program. Since we're doing a freestanding
// binary, we can't use the C, and thus must define our own entry point
// Thus, no main
#![no_main]

// function to be called on panic, since we cant use the std one
#[panic_handler]
#[allow(unconditional_recursion)]
// Does nothing, literally just recurses forever and should never return anything.
fn panic(info: &core::panic::PanicInfo) -> ! {
    panic(info)
}

// must use no_mangle because the function must be named _start *exactly*
#[unsafe(no_mangle)]

// tells the compiler that this function should use the "C calling convention", which is what the
// hardware is expecting, i think.
pub extern "C" fn _start() -> ! {
    // I would print here, but. yknow. no print

}

