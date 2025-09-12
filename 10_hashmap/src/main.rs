use std::collections::HashMap;

fn main() {
    let mut users = HashMap::new();

    users.insert(String::from("charith"), 22);
    users.insert(String::from("sai"), 17);

    println!("original users data: {:?}", users);
    match users.get("charith") {
        Some(age) => println!("charith's age is: {}", age),
        None => println!("No user named 'charith' found."),
    }
    users.remove("sai");
    println!("user data after removing sai: {:?}", users);
    users.clear();
    println!("user data after clearing everything: {:?}", users);

    println!("------------------------------------------------");

    let pairs = vec![(String::from("tony"), 55), (String::from("steve"), 120)];
    println!("the vector is {:?}", pairs);

    println!("After inserting the tuples from the vector, the resulting hashmap is {:?}", insert_tuples_into_hashmap(&pairs));
}

// Write a function that takes a vector of tuples
// (each tuple containing a key and a value) and returns a Hashmap
// where the keys are the unique keys from the input tuples
// and the values are vectors of all corresponding  values associated with each key

fn insert_tuples_into_hashmap(vec: &Vec<(String, i32)>) -> HashMap<&String, i32> {
    let mut hm = HashMap::new();

    for (key, value) in vec {
        hm.insert(key, *value);
    }

    return hm;
}
