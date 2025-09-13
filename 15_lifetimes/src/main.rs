// fn main() {
//     let longest_str;
//     println!("Hello, world!");
//     let str1 = String::from("aa");
//     {
//         let str2 = String::from("bbb");
//         longest_str = longest(&str1, &str2);
//     }
//     // when using references instead of Strings, the ownership stays
//     // with str2 itself, hence when longest_str is tried to be printed,
//     // it would be out of scope for str2. Thus creating a dangling reference
//     println!("longest string is: {}", longest_str);
// }

// // Write a function that takes two strings as an input
// // and returns the bigger amongst them

// fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
//     if a.len() > b.len() { a } else { b }
// }

struct User<'a> {
    name: &'a str,
}

fn main() {
    let name =  String::from("charith");
    let user = User {name : &name};

    println!("{}", user.name);
}
