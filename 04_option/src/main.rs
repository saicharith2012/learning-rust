// Write a function that returns the index of the first 'a' in a string

fn main() {
    let my_string = String::from("charith");

    match find_first_a(my_string) {
        Some(index) => println!("The letter 'a' is found at the index {}.", index),
        None => println!("The letter 'a' is nout found in the string.")
    }
}

fn find_first_a(s: String) -> Option<u32> {
    for (index, character) in s.chars().enumerate() {
        if character == 'a' {
            return Some(index as u32)
        }
    }

    return None;
}
