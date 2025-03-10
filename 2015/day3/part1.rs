use std::fs;
use std::collections::HashMap;

fn main() {
    let input_file = "input.txt";

    let contents = fs::read_to_string(input_file).expect("Error reading file");

    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut map: HashMap<(i32, i32), i32> = HashMap::new();

    map.insert((0, 0), 1);

    for char in contents.chars() {
        match char {
            '<' => x -= 1,
            '>' => x += 1,
            '^' => y += 1,
            'v' => y -= 1,
            _ => continue,
        }

        let count = map.entry((x, y)).or_insert(0);
        *count += 1;
    }

    println!("Houses visited at least once: {}", map.len());
}
