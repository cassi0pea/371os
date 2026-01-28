
use std::mem::transmute;

fn main() {
    println!("Hello, world!");

    let mut letter: char = 'a';
    let mut numb: u32 = 69;

    println!("Change the interpretation of numeric values with transmute on values:\n");

    println!("Character: {}", letter);
    println!("Number: {}", numb);

    let transmuted_letter: char = unsafe {
        transmute::<u32, char>(numb)
    };
    println!("Number transmuted to Character: {}\n\n", transmuted_letter);

    println!("Change the interpretation of memory with transmute on references:\n");

    letter = 'a';
    numb = 69;

    println!("Character: {}", letter);
    println!("Number: {}", numb);

    let ltr_ptr = &letter;

    // I want to transmute the pointer of the LETTER to be a pointer to a NUMBER
    let transmuted_number: u32 = unsafe {
        //this is creating a new pointer variable by transmuting the letter pointer into a number pointer
        *transmute::<&char, &u32>(ltr_ptr)
    };
    println!("Character transmuted to number by means of pointer chicanery: {}\n\n", transmuted_number);

    println!("and now the grand finale:");
    // 48 65 6C 6C 6F 2C 20 57 6F 72 6C 64 21 (13 characters)
    // no "numeric or string data"
    let hello_world_array: [u32; 13] = [0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x2C, 0x20, 0x57, 0x6F, 0x72, 0x6C, 0x64, 0x21];

    for &value in hello_world_array.iter() {
        unsafe {
            print!("{}", transmute::<u32, char>(value))
        }
    }
    print!("\n");




}


/* pub const unsafe fn transmute<Src, Dst>(src: Src) -> Dst
 *
 * So Transmute turns one 
 *
 */
