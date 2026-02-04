/*  Notes on Const vs Static
 *      CONST is basically a macro -- at compile time all usages of it are just swapped for it's
 *      definition
 *
 *      STATIC is called static *presumably* because it lives at a fixed location in memory. It's a
 *      real variable, and can be referenced (and, with unsafe code, changed) at runtime
 *      
 *      Now for some reason we need a static, mutable u8 array called bus
 */

// Macro defining SIZE as 1 kb
// Claude disagreed, and said this actually just, like, the number 128 in hex for Calvin reasons.
// meaning that SIZE, as used in the bus, is actually 128 *bytes*.
// This is... probably fine? i'm not going to change it, at any rate

// Come to think of it, I have no idea why this is even named size. the size is gonna change.
pub const SIZE: usize = 0x80;

// Initializes a static, mutable array consisting of "SIZE" u8s (that is, bytes)
// Specifically, initializes them as zeroes.
// I have to asssume we're going to need to change this -- the instructions say to support "any
// size which is a power of 2"
static mut BUS: [u8; SIZE] = [0u8; SIZE];
let bus_length: u32 = 7; // 2^7 == 128


// so i think we need to store the current length of the bus within the first parts of the bus itself
// then we also need a function to double it's size.
// uh. won't resizing an array (which I thought to be fixed-size) cause Big Problems?

// initialize function to do various startup tasks
fn init() {

    /*  So, Calvin is using a "validity bitmask"
     *  if a bit is set to 1, the corresponding byte (which we get by Counting) is in use.
     *
     *  Now, Calvin has been known to be. less then perfectly clear on the distinction between bits
     *  and bytes (e.g. SHA 256 kiiinda implying you needed blocks to be bit-aligned, when in fact
     *  all you would ever need is byte-aligning, because computers don't just have free bits
     *  running around -- at best, they have bytes that arent being fully utilized.
     *
     *  So they SAY they only reserve SIZE >> 3 (16) bits for their validity bitmask, but. wouldn't
     *  that only get them 16 bytes of space covered by the mask? I guess it gets them 16 "chunks"
     *  of SOME size.
     *
     *  128 / 16 == 8, which is what gives us this 1 to 8 overhead they mentioned.
     *
     */


}





// Inputs:       a usize, giving the number of bytes to allocate
// Outputs:      an Option<usize>, giving the "offset" of those bytes from some general reference
//               point -- that is, their location in memory.
// side FX:      iunno!
//
pub fn malloc(num_bytes: usize) -> Option<usize> {

    // and then I presume we're gonna need to do some unsafe things

}
