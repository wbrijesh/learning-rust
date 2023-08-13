struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u32,
}

struct Color (u8, u8, u8);

#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

fn main() {
    print_fields();

    copy_fields();
    
    colors_struct();

    use_area_fns();

    debug_struct();
}

fn print_fields() {
    let test_user = User {
        active: true, username: String::from("John Doe"),
        email: String::from("john@example.com"),
        sign_in_count: 0,
    };

    println!("username: {}", test_user.username);
    println!("active: {}", test_user.active);
    println!("email: {}", test_user.email);
    println!("sign in count: {}", test_user.sign_in_count);
}

fn copy_fields() {
    let template_user = User {
        active: true,
        username: String::from("John Doe"),
        email: String::from("john@example.com"),
        sign_in_count: 0,
    };

    let test_user = User {
        email: String::from("not-john@example.com"),
        ..template_user
    };

    println!("email: {}", test_user.email);
}

fn colors_struct() {
    let blue = Color(0,0,255);
    let green = Color(0,255,0);
    let red = Color(255, 0, 0);

    println!("blue is rgb({},{},{})", blue.0, blue.1, blue.2);
}

fn use_area_fns() {
    let width: u32 = 130;
    let height: u32 = 420;

    println!("using fn one: {}", area_fn_one(width, height));
    println!("using fn two: {}", area_fn_two((width, height)));
}

fn area_fn_one(h: u32, w: u32) -> u32 {
    return h * w;
}

fn area_fn_two(args: (u32, u32)) -> u32 {
    return args.0 * args.1;
}

fn debug_struct() {
    let test_rectangle = Rectangle {
        height: 150,
        width: 310,
    };

    println!("struct is: {:#?}", test_rectangle);

    dbg!(&test_rectangle);
}
