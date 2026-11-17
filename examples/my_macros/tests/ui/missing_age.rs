struct User {
    name: String,
    age: u32,
}

fn make() -> User {
    User { name: String::new() }
}
