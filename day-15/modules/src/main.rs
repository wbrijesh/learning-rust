pub mod vegetables;


use crate::vegetables::asparagus::asparagus_fn;


fn main() {
    println!("{}", asparagus_fn("Hello, world!".to_string()));
}
