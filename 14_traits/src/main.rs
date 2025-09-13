pub trait Summary {
    fn summarize(&self) -> String; // function signature
}

pub trait Fix {
    fn fix(&self) -> String {
        return String::from("hi there from fix");
    }
}

struct User {
    name: String,
    age: u32,
}

// user struct implementing summary with an implementation of its own.
impl Summary for User {
    fn summarize(&self) -> String {
        return format!("User '{}' is {} years old.", self.name, self.age);
    }
}

impl Fix for User {}

// ---------------------------------------

struct Admin {}

impl Fix for Admin {}

// traits as parameters
fn notify(u: &impl Summary) {
    println!("{}", u.summarize())
}

fn summary_fix_user<T: Summary + Fix>(item: T) {
    println!("{}, {}", item.summarize(), item.fix());
}

fn main() {
    let user = User {
        name: String::from("charith"),
        age: 22,
    };

    let _admin = Admin {};

    println!("{}", user.summarize());
    notify(&user);
    // summary_fix_user(admin); --> throws error since admin doesn't implement Summary trait
    summary_fix_user(user);
}

