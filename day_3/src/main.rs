use std::fs;
use regex::Regex;

fn main() {
    println!("Hello, world!");
    
    let file_content = fs::read_to_string("src/data.txt").expect("There was an error reading the file\n");
    let rx = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut sum = 0;
    for matched in rx.captures_iter(&file_content) {
        println!("Captured regex {}", &matched[0]);
        let num_one:i32 = matched[1].parse().unwrap();
        let num_two:i32 = matched[2].parse().unwrap();
        sum += num_one * num_two; 
    }

    println!("Total sum: {}", sum);
}
