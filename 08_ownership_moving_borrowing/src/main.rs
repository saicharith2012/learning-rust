// fn main() {
//     // only one owner at a time.
//     let a1 = String::from("charith");
//     // the ownership shifts to a2, hence a1 becomes invalid.
//     {
//         let a2 = a1;

//         println!("number is {}", a2);
//     }
//     println!("number is {}", a1); // throws error
// }

// fn main() {
//     let s1 = String::from("charith");
//     do_something(s1);
//     println!("number is {}", s1);
// }

// fn do_something(s2: String) {
//     println!("{}", s2);
// }

// To solve it return the value and pass it to s1 again (make sure s1 is mutable)
// fn main() {
//     let mut s1 = String::from("charith"); // s1 owns the value
//     s1 = do_something(s2: s1); // s2 owns the value
// returning the value to s1 hence the ownership moves back to s1.
//     println!("number is {}", s1); // s1 owns again
// }

// fn do_something(s2: String) -> String {
//     println!("{}", s2);  // s2 owns the value
//     return s2;
// }

// BORROWING...using references
// fn main() {
//     let s1 = String::from("charith");
//     do_something(&s1); // s2 is borrowing the value owned by s1. Here &s1 is the string slice
//     println!("number is {}", s1);
// }

// fn do_something(s2: &String) {
//     println!("{}", s2); // s1 still owns the value, s2 just borrowed
// }


// MUTATING the borrowed values
fn main() {
    let mut s1 = String::from("sai");
    println!("number is {}", s1);
    do_something(&mut s1); // passing s1 as a mutable reference
    
    println!("number is {}", s1);
}

fn do_something(s2: &mut String) {
    s2.push_str(" charith");
}


// Rules for references
// fn main() {
//     let mut s1 = String::from("charith");
//     let s2 = &mut s1;
//     let s3 = &s1; //  you can have either one mutable reference or any number of immutable references.

//     println!("num is {}, {}, {}", s1, s2, s3)
// }


