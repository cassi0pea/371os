/*  Notes on Const vs Static
 *      CONST is basically a macro -- at compile time all usages of it are just swapped for it's
 *      definition
 *
 *      STATIC is called static *presumably* because it lives at a fixed location in memory. It's a
 *      real variable, and can be referenced (and, with unsafe code, changed) at runtime
 */


pub const SIZE: usize = 0x80; // 128 in hex
// we preset the first two entries to 0xff, i.e. (11111111) to reserve 16 bytes in total on the
// bus for the bitmask itself
static mut DATA: [u8; SIZE] = [0u8; SIZE];


// initialize function. Sets up bitmask, puts the bus length on the bus itself, etc.
fn init() {
    unsafe {
    assert!(SIZE & (SIZE - 1) == 0); // power of two checking
    // so, this mask. each of these is one byte. each BIT within EACH BYTE represents one BYTE in
    // use on the bus.
    
    // Note: this assumes a size of 128! if you want it to be good, fix this later
    //  (you'll need to reserve SIZE >> 3 bits)
    DATA[0] = 0xff;
    DATA[1] = 0xff;   
    }
}



fn bitmask_helper(start: usize, mut n: usize) {
    unsafe {
    // sets the appropriate bits of the bitmask to 1
    
    // tells the position in the bitmask
    let mut bytespot = (start/8);
    // tells the position within a byte in the bitmask
    let mut bitspot = (start%8);
    // While we have not yet set all the bits to 1 that we need to
    while n > 0 {
        for j in bitspot..=n.min(7) {
            DATA[bytespot] |= 1<<j;
            n -= 1;
            if n == 0 { break }
            }
        // if you've done more than one loop you gotta start from 0
        bitspot = 0;
        // go to the next byte
        bytespot += 1;
        }
    }
}


// Inputs:       a usize, giving the number of bytes to allocate
// Outputs:      an Option<usize>, giving the "offset" of those bytes from some general reference
//               point. 
//               In english, an index in BUS (which is literally just an array
// side FX:      iunno!

pub fn malloc(n: usize) -> Option<usize> {
    
    unsafe {
    
    
    // scan for a region of bytes to allocate
    
    let mut bytes_found = 0;
    // the index in DATA of the start of the contiguous region of n free bytes
    let mut start:usize = 0;
    // for i in range(0,16)
    // representing the first 16 bytes of DATA, i.e. the bitmask
    for i in (0..(SIZE >> 3)) {
    
        // representing the 8 data bytes tracked by each byte of the bitmask 
        for j in 0..8 {
            // if the corresponding data byte is in use:
            if ((DATA[i] >> j) & 1 == 1) {
                start = 0;
                bytes_found = 0;
            // if the byte is NOT in use:
            } else {
                // if start is zero, then this is the start of a contiguous "run"
                // Thus, we must start tracking
                if start == 0 {
                    start = (i*8) + j;
                }
                // we found another free byte! huzzah!
                bytes_found += 1;

            }
            // if we have successfully found a contiguous free region:
            if bytes_found == n {
                bitmask_helper(start,n);
                return Some(start);
                }
            }
        }

        return None;
    }

}

// you have some data "val" of type "T" that you are setting at loc
pub fn setter<T>(value:T, location: usize) {
    unsafe {
       // cast the bus to a raw pointer
       // cast the bus to a u8
       // add the value of location
       // cast the bus (which now refers to the spot you want it to) to type T
       // .write(value)
       
        (&raw mut DATA)
            .cast::<u8>()
            .add(location)
            .cast::<T>()
            .write(value);
    }
}



pub fn getter<T>(location: usize) -> T {
    unsafe {
        return (&raw mut DATA)
            .cast::<u8>()
            .add(location)
            .cast::<T>()
            .read();
    }
}


































