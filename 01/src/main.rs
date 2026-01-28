use std::fs;
use std::env;

/* Notes:
 * Other stuff i have to support (by tomorrow!) includes:
 *  - have single character flags be prefixed with "-"
 *  - have wordier flags be prefixed with "--"
 *  - the ability to automatically do a wordcount on standard in if no filename is provided
 *  - every fucking flag under the sun
 *  - jesus christ why did i put this off. Kill me with a gun.
 *
 */

fn main() {
    // collects arguments from the command line and puts them in a vector of strings
    let mut args = env::args();

    // These all default to true because the "flagless" behavior is to print all three
    let mut print_bytes :bool = True;
    let mut print_chars :bool = True;
    let mut print_words :bool = True;
    let mut print_lines :bool = True;


    // Okay. So as far as those flags go.
    // look i bet theres an elegant way to do this with a match case or something but I sorta don't see why I wouldnt just. yknow.
    // compare the entire input WAIT that's why. okay so you differentiate "-" case, "--" case, and "" case, and then match the text from there

    /* For single character flags, the letters I have to look for are "cmlLw"
        Obnoxiously, I can't just look at the first character. Well, I can, but i have to do so on a per-argument basis.
        So: Iterate over the args, look at the first two characters, and set flags from there
     */

    for argument in args {
        // oh for real? this is just gonna work?
        if argument.starts_with("--") {
            // jesus christ it actually is that easy. Pro tip: never ever ever do anything new. there will always just be a method for it.
            print_bytes = False;
            print_chars = False;
            print_words = False;
            print_lines = False;
            // okay so with the double dash you only get one word per go
            match argument.as_str() {
                "bytes" => print_bytes = True,
                "chars" => print_chars = True,
                "lines" => print_lines = True,
                "max-line-length" => 4,
                "words" => 5,
                // MISSING TWO HERE! files from f and total = when or some similar shit
                _ => 0

            }

        }
    }

    // gets the path from the first command line argument
    let path_to_file = args.nth(1).unwrap();

    // reads that entire file into a string
    let file_contents = fs::read_to_string(path_to_file).unwrap();
    // gets the word count
    wc(file_contents);
    return;
}

/*
INPUTS:
    a path to a text file as a string
OUTPUTS:
    A number reporting the number of distinct words (here defined as blocks of letters or symbols separated by spaces) in the file
SIDE EFFECTS:
    None
 */
fn wc(words: String) -> u32 {
    // so i want to go through the string character by character and increment the word counter every time i find a space or a newline
    // UNLESS the last character was also a space or newline
    let mut word_count:u32 = 0;
    let mut newline_count:u32 = 0;
    let mut byte_count:u32 = 0;

    let mut consecutive_empties_flag = 0;
    for character in words.chars() {
        if (character == ' ' || character == '\n') & (consecutive_empties_flag == 0) {
            word_count += 1;
            consecutive_empties_flag = 1;
        } else {
            consecutive_empties_flag = 0;
        }
        if character == '\n' {
            newline_count += 1;
        }
        byte_count += 1;
    }

    println!("word count: {}\n", word_count);
    println!("nwln count: {}\n", newline_count);
    println!("byte count: {}\n", byte_count);

    return word_count;
}









