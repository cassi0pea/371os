use std::fs;
use std::env;


fn main() {
    // collects arguments from the command line and puts them in a vector of strings
    let mut args = env::args();
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
    let mut consecutive_empties_flag = 0;
    for character in words.chars() {
        if (character == ' ' || character == '\n') & (consecutive_empties_flag == 0) {
            word_count += 1;
            consecutive_empties_flag = 1;
        } else {
            consecutive_empties_flag = 0;
        }
    }

    println!("word count: {}\n", word_count);
    return word_count;
}
