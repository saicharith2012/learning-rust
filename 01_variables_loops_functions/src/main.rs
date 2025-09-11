fn main() {
    println!("Hello, world!");

    println!("{}", is_even(10));
    
    println!("The resulting fibonacci term is {}.", fib(4));

    let name = String::from("charith");
    println!("The input string length is {}.", get_string_length(name))

}

// Write a function is_even that takes a number as an input
// and returns true if it is even.
fn is_even(num: i32) -> bool {
    if num % 2 == 0 {
        return true;
    }

    return false;
}

// Write a function fib that finds the fibbonacci of a number
// it takes as input....0,1,1,2,3
fn fib(num: u32) -> u32 {
    let mut first = 0; // mutable variable can be changed...by default variables can't be changed in rust.
    let mut second = 1;

    if num == 0 {
        return first;
    }

    if num == 1 {
        return second;
    }

    for _ in 0..num-1 {
        let temp = second;
        second = second + first;
        first = temp;
    }

    return second
}

// Write a function get_string_length that takes a string 
// as an input and returns its length.
fn get_string_length(str: String) -> usize {
    str.chars().count()
}

// i32 => signed integer
// u32 => unsigned integer