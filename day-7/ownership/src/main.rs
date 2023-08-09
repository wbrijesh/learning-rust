fn main() {
    // variable_scopes()
    three()
}

fn variable_scopes() {
    {
        let s: String = "Brijesh".to_string();
        println!("S equals {}", s);
    }

    // this would give error while compiling as there is no variable s in scope
    // println!("S equals {}", s);
}

fn creating_strings() {
    // the compiler infers type string
    let a = "a";

    // we are explicitly stating the type as string, but also need to convert string literal to
    // string
    let b: String = "b".to_string();

    // using factory method to create string and explicitly stating type
    let c: String = String::from("c");

    // using factory method to create string and compiler knows this is a string from the method
    let d = String::from("d");

    // using string literal as variable is immutable anyways
    let e: &str = "e";
}

fn three() {
    let x = String::from("g");
    let y = x;

    // cannot get location of X as its value has been moved to Y, and will throw error value
    // borrowed after moved
    // println!("X is at {:p}", &x);
    
    println!("Y is at {:p}", &y);
}
