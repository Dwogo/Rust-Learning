pub struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("Userboi"),
        active: true,
        sign_in_count: 1,
    };

    println!("User1 username: {}", user1.username);
    println!("User's email: {}", user1.email);
    println!("Is User1 active: {}", user1.active);
    println!(
        "How many times has User1 signed in: {}",
        user1.sign_in_count
    );
}
