use std::io;

#[derive(Debug)]
struct User {
    email: String,
    password: String,
}

fn main() {
    let dummy_database = vec![
        User {
            email: String::from("john@example.ext"),
            password: String::from("password"),
        },
        User {
            email: String::from("jane@example.ext"),
            password: String::from("password456"),
        },
    ];

    let email: String = answer("Enter your email:");
    let password: String = answer("Enter your password:");

    let mut user_match: Option<&User> = Default::default();
    for user in &dummy_database {
        if user.email == email && user.password == password {
            user_match = Some(user);
            break;
        }
    }

    if Option::is_some(&user_match) {
        println!("Login successful! You are {:#?}", user_match);
    } else {
        println!("Login failed!");
    }
}

fn answer(prompt: &str) -> String {
    println!("{}", prompt);
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    return line.trim().to_string();
}
