#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.height * self.width;
    }

    // ignore cases where you can fit at angles other than n * 90 degrees
    fn can_fit(&self, other: &Rectangle) -> bool {
        if self.width > other.width && self.height > other.height {
            return true;
        } else if self.height > other.width && self.width > other.height {
            return true;
        } else {
            return false;
        }
    }
}

impl Rectangle {
    fn square(length: u32) -> Self {
        return Self {
            height: length,
            width: length,
        }
    }
}

fn main() {
    let test_rectangle = Rectangle {
        height: 100,
        width: 250,
    };

    println!("area of test rectangle is {}", test_rectangle.area());

    let r1 = Rectangle {
        height: 300,
        width: 400,
    };

    let r2 = Rectangle {
        height: 200,
        width: 300,
    };

    let r3 = Rectangle {
        height: 350,
        width: 200,
    };

    let r4 = Rectangle {
        height: 350,
        width: 350,
    };

    println!("r1 can fit r2: {}", r1.can_fit(&r2));
    println!("r1 can fit r3: {}", r1.can_fit(&r3));
    println!("r1 can fit r4: {}", r1.can_fit(&r4));

    println!("square of side equal to r1's height: {:#?}", Rectangle::square(r1.height));
}
