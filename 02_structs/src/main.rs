// user struct
struct User {
    first_name: String,
    last_name: String,
    email: String,
    age: u32,
}

impl User {
    fn print_user_data(&self) {
        println!("firstname: {}", self.first_name);
        println!("lastname: {}", self.last_name);
        println!("email: {}", self.email);
        println!("age: {}", self.age);
    }
}

// rectangle struct
struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
}

fn main() {
    let user = User {
        first_name: String::from("charith"),
        last_name: String::from("palabindhela"),
        email: String::from("saicharithpalabindhela@gmail.com"),
        age: 22,
    };

    user.print_user_data();

    println!("-------------------------------------------");

    let rect = Rect {
        width: 30,
        height: 50,
    };

    println!("the area of rectangle is: {}", rect.area());
    println!("the perimeter of rectangle is: {}", rect.perimeter());
}

// Structs are more similar to classes in javascript
