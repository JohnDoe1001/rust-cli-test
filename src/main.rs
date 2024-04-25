// @@@ LIBRARY SPACE START
use library::lib::utility;
// LIBRARY SPACE END
// @@@ MAIN PROGRAM SPACE START
fn main() {
    utility::argument_processing();
}
// MAIN PROGRAM SPACE END
// @@@ FUNCTION SPACE START

// FUNCTION SPACE END
// @@@ MODULE SPACE START
mod library {
    pub mod lib;
    pub mod binary {
        pub mod logic;
    }
}
// MODULE SPACE END
