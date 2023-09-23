#[derive(Debug)]
struct Rectangle {
    height: usize,
    width: usize
}

impl Rectangle {
    fn area(&self) -> usize {
        return self.width * self.height
    }
}

fn main() {
    let example_rectangle = Rectangle {
        width: 20,
        height: 35,
    };

    println!("Area of {:#?} is {} square pixels.", example_rectangle, example_rectangle.area())
}
