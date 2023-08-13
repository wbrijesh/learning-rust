struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u16,
}

fn build_user(username: String, email: String) -> User {
    return User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    };
}

fn main() {
    let user_one = User {
        active: true,
        username: String::from("user_one_username"),
        email: String::from("user_one@mail.ext"),
        sign_in_count: 0,
    };

    let user_two = build_user(username: String::from("user_two_username"), email: String::from("user_two@mail.ext"));

    println!("email of user two is: {}", user_two.email);
}
