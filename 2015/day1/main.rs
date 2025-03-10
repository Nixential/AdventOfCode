use std::fs;

fn main() {
    let file_name = String::from("input.txt");
    let mut count: i32 = 0;

    let file_contents = fs::read_to_string(file_name).expect("Error reading file");

    for char in file_contents.chars() {
        if char == '(' {
            count += 1;
        }
        else if char == ')' {
            count -= 1;
        }
    }
    println!("{}", count);
}