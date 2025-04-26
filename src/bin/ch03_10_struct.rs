fn main() {
    #[derive(Debug)]
    struct User {
        username: String,
        email: String,
        sign_in_count: u32,
        active: bool,
    }

    impl User {
        fn new(username: &str, email: &str) -> User {
            User {
                username: username.to_string(),
                email: email.to_string(),
                sign_in_count: 1,
                active: true,
            }
        }
        fn show(&self) {
            println!("User info: username={}, email={}, sign_in_count={}, active={}",
                self.username, self.email, self.sign_in_count, self.active);
        }
    }

    let mut user1 = User::new("alice", "alice@example.com");
    println!("user1 = {:?}", user1);
    user1.show();

    println!("Username: {}", user1.username);
    println!("Email: {}", user1.email);
    println!("Active: {}", user1.active);
    println!("Sign in count: {}", user1.sign_in_count);
    user1.email = String::from("alice@newmail.com");
    println!("Updated email: {}", user1.email);
    user1.show();

    let user2 = User {
        username: String::from("bob"),
        email: String::from("bob@example.com"),
        ..user1
    };
    println!("user2 = {:?}", user2);
    user2.show();

    // Tuple struct
    #[derive(Debug)]
    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);
    println!("black = {:?}", black);
    // Use fields to avoid dead_code warning
    println!("Color fields: {}, {}, {}", black.0, black.1, black.2);

    // Unit-like struct
    #[derive(Debug)]
    struct Marker;
    let m = Marker;
    println!("marker = {:?}", m);
}
