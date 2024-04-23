/* 
// @@@ PSEUDOCODE SPACE START
Goal: Create a text-based game like ZORG, with a 
limited set of commands and the ability to accept 
arguments.




// PSEUDOCODE SPACE END
*/
// @@@ LIBRARY SPACE START
#[warn(non_snake_case)]
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
    dbg!("Supplied arguments: {}", arguments);
}
// FUNCTION SPACE END
