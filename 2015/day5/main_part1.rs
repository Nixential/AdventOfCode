use std::fs;

fn has_repeated_letter(s: &str) -> bool {
    let mut chars = s.chars();
    let mut prev = chars.next();

    for current in chars {
        if Some(current) == prev {
            return true;
        }
        prev = Some(current);
    }
    
    false
}

fn has_at_least_three_vowels(s: &str) -> bool {
    let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    let count = s.chars().filter(|c| vowels.contains(c)).count();
    count >= 3
}

fn is_nice(string: &str) -> bool {
    // for char in string.to_string().chars() {
    //     if char == 'a' {
    //         return true;
    //     }
    // }
    // ab, cd, pq, or xy
    if string.contains("ab") || string.contains("cd") || string.contains("pq") || string.contains("xy") {
        return false;
    }
    if !has_at_least_three_vowels(string) {
        return false;
    }
    if !has_repeated_letter(string) {
        return false;
    }
    return true;
}


fn main() {
    let contents = fs::read_to_string("input.txt").expect("Error opening file");
    let mut count_nice: i32 = 0;

    let strings: Vec<&str> = contents.split('\n').collect();
    println!("{:?}", strings);

    for word in strings {
        if is_nice(word) {
            count_nice += 1;
        }
    }

    println!("Count nice = {}", count_nice);
}