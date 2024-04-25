pub mod utility {
    use std::env;

    pub fn argument_processing() {
        let arguments: Vec<String> = env::args().collect();
        let n = 0;
        
        let arg: &String = &arguments[n];
        println!("Supplied arguments: {}", arg);
    }
}
