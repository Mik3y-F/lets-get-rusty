fn main() {
    println!("Hello, world!");
    let user1: User = build_user(String::from("test@email.com"), String::from("test"));
    let user2: User = User {
        email: String::from("test2@email.com"),
        ..user1
    };

    println!("user2: {:#?}", user2);
}

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

struct _Color(i32, i32, i32);
struct _Point(i32, i32, i32);
