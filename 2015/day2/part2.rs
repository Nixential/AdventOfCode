use std::fs;
use std::str::FromStr;

fn main() {
    let file_name: String = "input.txt".to_string();
    let input_content = fs::read_to_string(file_name).expect("Error opening file");

    let parts: Vec<&str> = input_content.split("\n").collect();

    let mut boxes: Vec<Vec<&str>> = Vec::new();
    let mut total_ribbon: i32 = 0;

    for part in parts {
        boxes.push(part.split("x").collect());
    }

    for boxx in boxes {
        let l = i32::from_str(boxx.get(0).expect("Cannot parse first element")).unwrap();
        let w = i32::from_str(boxx.get(1).expect("Cannot parse second element")).unwrap();
        let h = i32::from_str(boxx.get(2).expect("Cannot parse third element")).unwrap();

        let ribbon_wrap = smallest_perimeter(&l, &w, &h);
        let ribbon_bow = l * w * h;

        total_ribbon += ribbon_wrap + ribbon_bow;
    }

    println!("{}", total_ribbon);
}

fn smallest_perimeter(l: &i32, w: &i32, h: &i32) -> i32 {
    let mut sides = vec![*l, *w, *h];
    sides.sort();
    2 * (sides[0] + sides[1])
}
