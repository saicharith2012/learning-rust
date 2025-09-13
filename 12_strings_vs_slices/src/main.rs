// Write a function that takes a string as an input and
// returns the first word from it.

fn main() {
    let name = String::from("hello world");

    // Returning a new string
    let ans = first_word(&name);
    println!("first word: {}", ans);
    
    // Using a string slice
    // let ans2 = &name[0..5]; // borrowing immutably
    
    // // name.clear(); // clear and push create a mutable reference to the string // clearing or mutating the original string is not allowed in between
    // // name.push('t'); // since the immutable reference is being used afterwards
    
    // println!("{}", ans2);
    
    // println!("After clearing: {}", name);
    let word2 = find_first_word(&name);
    println!("first word slice: {}", word2);
}

fn first_word(str: &String) -> String {
    let mut ans = String::from("");
    for i in str.chars() {
        if i == ' ' {
            break;
        }
        ans.push(i);
    }

    return ans;
}

fn find_first_word(word: &String) -> &str {
    let mut index = 0;
    for (_, c) in word.chars().enumerate() {
        if c == ' ' {
            break;
        }
        index = index + 1;
    }

    return &word[0..index];
}
