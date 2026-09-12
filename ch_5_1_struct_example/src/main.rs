struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u32,
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("user123"),
        email: String::from("user123@email.com"),
        sign_in_count: 0,
    };

    user1.email = String::from("user1234@gmail.com");
    println!(
        "user1: {0}, {1}, {2}, {3}",
        user1.username, user1.email, user1.active, user1.sign_in_count
    );

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another.email123@email.com"),
        sign_in_count: user1.sign_in_count,
    };
    println!(
        "user2: {0}, {1}, {2}, {3}",
        user2.username, user2.email, user2.active, user2.sign_in_count
    );

    let user3 = build_user(user2.username, user2.email);
    println!(
        "user3: {0}, {1}, {2}, {3}",
        user3.username, user3.email, user3.active, user3.sign_in_count
    );
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 0,
    }
}
