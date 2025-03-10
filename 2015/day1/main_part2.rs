use std::fs;

fn main() {
    let file_name = String::from("input.txt");
    let mut count: i32 = 0;
    let mut curr_index: usize = 0;

    let file_contents = fs::read_to_string(file_name).expect("Error reading file");

    for (index, char) in file_contents.char_indices() {
        if char == '(' {
            count += 1;
        }
        else if char == ')' {
            count -= 1;
        }
        if count == -1
        {
            curr_index = index + 1;
            break;
        }
    }
    println!("{}", curr_index);
}