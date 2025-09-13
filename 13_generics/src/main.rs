fn main() {
    let result1 = max_value(1, 2);
    println!("result1: {}", result1);
    let result2 = max_value('a', 'b');
    println!("result1: {}", result2);
}

fn max_value<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}