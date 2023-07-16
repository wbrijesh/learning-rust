use std::env;

fn main() {
    // this program expects a string to be passed as a CLI arguemnt
    
    let args: Vec<String> = env::args().collect();

    let name = &args[1];

    println!("Hi, {}", name);
}
