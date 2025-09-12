fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(7);

    println!("original array: {:?}", vec); // :? --> debug trait

    println!("evenly filtered array: {:?}", even_filter(&vec));

    even_filter_inplace(&mut vec);
    println!("original array after evenly filtering inplace: {:?}", vec);

    // initialising vectors using rust macros
    let numbers = vec![1, 2, 3];
    for number in numbers {
        print!("{} ", number);
    }
    println!();
}

// Write a function that takes a vector as an input
// and returns a vector with even values
fn even_filter(vec: &Vec<i32>) -> Vec<i32> {
    let mut new_vec = Vec::new();
    for val in vec {
        if val % 2 == 0 {
            new_vec.push(*val);
        }
    }

    return new_vec;
}

// even filter in-place
fn even_filter_inplace(vec: &mut Vec<i32>) {
    let mut i = 0;
    while i < vec.len() {
        if vec[i] % 2 != 0 {
            vec.remove(i);
        } else {
            i += 1;
        }
    }
}
