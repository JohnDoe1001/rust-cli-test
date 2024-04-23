/* 
// @@@ PSEUDOCODE SPACE START
Goal: Create a text-based game like ZORG, with a 
limited set of commands and the ability to accept 
arguments.




// PSEUDOCODE SPACE END
*/
// @@@ LIBRARY SPACE START
use std::env;

// LIBRARY SPACE END
// @@@ MAIN PROGRAM SPACE START
fn main() {
    argument_processing();
}
// MAIN PROGRAM SPACE END
// @@@ FUNCTION SPACE START
fn argument_processing() {
    let arguments: Vec<String> = env::args().collect();
    let arg1 = &arguments[1];
    let arg2 = &arguments[2];
    println!("Supplied arguments: {} and {}", arg1, arg2);

}
// FUNCTION SPACE END
