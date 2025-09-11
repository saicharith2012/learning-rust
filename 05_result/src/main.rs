use std::fs::read_to_string;

fn main() {
    let greeting_file_result = read_to_string("hello.txt"); // path relative to the root dir

    match greeting_file_result {
        Ok(file_content) => {
            println!("File read successfully: {}", file_content)
        }
        Err(err) => {
            println!("Error: {}", err)
        }
    }
}
