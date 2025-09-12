fn main() {
    let mut v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got {}", val);
    }

    println!("original array: {:?}", v1);

    // mutable references
    let v1_mut_iter = v1.iter_mut();

    for val in v1_mut_iter {
        *val += 1;
    }

    println!("array after mutable iteration: {:?}", v1);

    // iterating using .next()
    let mut v2 = vec!['a', 'b', 'c'];

    let mut v2_iter = v2.iter_mut();

    while let Some(val) = v2_iter.next() {
        println!("Got {}", val);
    }

    // iterating using into_iter()
    // ownership is transferred to the iterator itself.
    // Hence the original collection identifier is no longer valid.
    let v2_into_iter = v2.into_iter();

    for val in v2_into_iter {
        println!("Owned {}", val);
    }

    // println!("{:?}", v2); // throws error saying ownership moved.

    // using for loop directly on collection is same as using into_iter

    let v3 = vec![1.0, 2.0, 3.0];

    for val in v3 {
        print!("{}", val);
    }

    println!();

    // println!("{:?}", v3); throws error saying ownership moved.

    println!("----------------------------------------");
    // Types of Adapters
    // Consuming Adapters
    let v4 = vec![1, 2, 3];

    let v4_iter = v4.iter();

    let sum: i32 = v4_iter.sum();

    println!("Sum is {}", sum);

    // for i in v1_iter {

    // } // iterator doesn't work anymore since sum took the ownership of iter()

    // Iterator Adapters
    let v5 = vec![4, 5, 6];
    println!("original array: {:?}", v5);

    // MAP

    let v5_iter_map = v5.iter().map(|x| x + 1);

    print!("mapped array -- adding 1 to each element: ");
    for val in v5_iter_map {
        print!("{} ", val);
    }
    println!();

    // FILTER

    let v5_iter_filter = v5.iter().filter(|x| *x % 2 == 0);
    print!("even filter: ");
    for val in v5_iter_filter {
        print!("{} ", val);
    }
    println!();



    // Write the logic to first filter all odd values
    // then double each value and create a new vector.
    let mut new_vec: Vec<i32> = Vec::new();
    
    let v6 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    
    let v6_iter_filter_map = v6.iter().filter(|x| *x % 2 != 0).map(|x| x*2);
    
    for val in v6_iter_filter_map {
        new_vec.push(val);
    }

    // collect() method of iterators can also be used to load the iterator values into a new vector instead of the above FOR loop.

    println!("original vector: {:?}", v6);
    println!("new vector: {:?}", new_vec);
}
