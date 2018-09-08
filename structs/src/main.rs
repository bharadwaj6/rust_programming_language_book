struct User {
    username: String,
    email: String,
    sign_in_count: i32,
    active: bool
}

fn main() {
    let user = User {
        username: String::from("Someone"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
        active: true
    };
    println!("username is {}", user.username);
}
