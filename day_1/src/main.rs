use std::fs;
use std::collections::HashMap;

fn main() {
    let file_content = fs::read_to_string("src/data.txt").expect("There was an error reading the file\n");

    let mut left_column: Vec<u32> = vec![];
    let mut right_column: Vec<u32> = vec![];

    let file_rows = file_content.lines();
    for row in file_rows {

        let numbers = row.split("   ");   
        let mut index = 0;
        for number in numbers {
            if(index == 0) {
                left_column.push(number.parse().unwrap());
                index += 1;
            } else {
                right_column.push(number.parse().unwrap());

            }
        }
    }
    
    left_column.sort();
    right_column.sort();

    println!("{:?}", left_column);
    println!("{:?}", right_column);

    let mut sum: u32 = 0;

    let frequencies = right_column.iter().copied().fold(HashMap::new(), |mut map, val| {
        map.entry(val).and_modify(|frq|*frq+=1).or_insert(1);
        map
    } );

    for i in 0..left_column.len() {
        // Part 1
        //sum += left_column[i].abs_diff(right_column[i]);    

        // Part 2
        let frequency = frequencies.get(&left_column[i]);
        match frequency {
            Some(freq) => sum += left_column[i] * freq,
            None => {} 
    }}

    println!("Result: {}", sum); 
}
