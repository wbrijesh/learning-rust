pub mod utils;

use crate::utils::scope_a::scope_b::same_name;
use crate::utils::scope_a::scope_c::same_name as same_name_two;

fn main() {
    same_name();
    same_name_two();
    println!("Hello, world!");
}
