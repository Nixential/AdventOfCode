use std::fs;
use std::collections::HashSet;

fn main() {
    let input_file = "input.txt";
    let contents = fs::read_to_string(input_file).expect("Error reading file");

    let mut santa_x = 0;
    let mut santa_y = 0;
    let mut robo_x = 0;
    let mut robo_y = 0;

    let mut visited_houses: HashSet<(i32, i32)> = HashSet::new();
    visited_houses.insert((0, 0));

    for (i, char) in contents.chars().enumerate() {
        let (x, y) = if i % 2 == 0 {
            (&mut santa_x, &mut santa_y)
        } else {
            (&mut robo_x, &mut robo_y)
        };

        match char {
            '<' => *x -= 1,
            '>' => *x += 1,
            '^' => *y += 1,
            'v' => *y -= 1,
            _ => continue,
        }

        visited_houses.insert((*x, *y));
    }

    println!("Houses visited at least once: {}", visited_houses.len());
}
