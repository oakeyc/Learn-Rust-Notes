struct User {
    name: String,
    age: u32,
    email: String,
}

fn main() {
    let user = User {
        name: "CJ".to_string(),
        age: 100,
        email: String::from("oakeyc@github.com"),
    };
    println!("My name is {} and I'm {} years old", user.name, user.age);
}
