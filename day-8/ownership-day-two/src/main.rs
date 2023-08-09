fn main() {
   three() 
}

// fn clone() {
//     let a = String::from("test");
//     let b = a.clone();
// 
//     println!("a: {} \nb: {}", a, b)
// }

// fn give_var_vs_pass_value() {
//     let x: i16 = 100;
// 
//     pass_value(&x);
//     give_var(x);
// }
// 
// fn give_var(variable: i16) {
//     println!("var: {}", variable);
// }
// 
// fn pass_value(variable: &i16) {
//     println!("var: {}", variable);
// }

fn three() {
    let a = String::from("hi");

    let (b, length) = get_length(a);

    println!("b: {}, length: {}", b, length);
}

fn get_length(var: String) -> (String, usize) {
    let length = var.len();
    return (var, length);
}
