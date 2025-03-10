use std::fs;
use std::str::FromStr;

fn main() {
    let file_name: String = "input.txt".to_string();
    let input_content = fs::read_to_string(file_name).expect("Error opening file");

    let parts: Vec<&str> = input_content.split("\n").collect();

    let mut boxes: Vec<Vec<&str>> = Vec::new();

    let mut total_area: i32 = 0;

    for part in parts {
        boxes.push(part.split("x").collect());
    }
    
    for boxx in boxes {
        let l = i32::from_str(boxx.get(0).expect("Cannot parse first element")).unwrap();
        let w = i32::from_str(boxx.get(1).expect("Cannot parse second element")).unwrap();
        let h = i32::from_str(boxx.get(2).expect("Cannot parse third element")).unwrap();

        let area = 2*l*w + 2*w*h + 2*h*l;

        total_area += area;
        total_area += smallest_side(&l, &w, &h);

        // println!("{:?} x {:?} x {:?}", first, second, third);
    }

    println!("{}", total_area);   
}

fn smallest_side(l: &i32, w: &i32, h: &i32) -> i32 {
    let side1 = l * w;
    let side2 = w * h;
    let side3 = h * l;

    side1.min(side2).min(side3)
}