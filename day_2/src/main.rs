use std::fs;

fn main() {
    let file_content = fs::read_to_string("src/data.txt").expect("There was an error reading the file\n");

    let file_lines = file_content.lines();

    let mut safe_count = 0;

    for line in file_lines {

        let mut safe = true;
        let numbers = line.split(" ");

        let mut unsafe_counts = 0;
        let mut last_number: i32 = -69;
        let mut last_difference: i32 = -69;
        for number in numbers {
            let this_number = number.parse().unwrap();
            println!("Reading number {} | Last number was {}", number, last_number);
            if last_number != -69 {

                println!("La diferencia absoluta entre ellos es {}", last_number.abs_diff(this_number));
                println!("La diferencia real entre ellos es {}", last_number - this_number);
                println!("El signo actual {} | El signo anterior {}", (last_number - this_number).signum(), last_difference);

                if last_number.abs_diff(this_number) > 3 || last_number.abs_diff(this_number) <= 0 {
                    safe = false;
                    unsafe_counts += 1;
                } else {
                    let this_difference = last_number - this_number;
                    if last_difference != -69 {
                        if this_difference.signum() != last_difference || this_difference.signum() == 0 {
                            safe = false;
                            unsafe_counts += 1;
                        } else {
                            last_difference = this_difference.signum();
                        }
                    } else {
                        last_difference = this_difference.signum();
                    }
                }
            } 
            last_number = this_number;
            println!("This pair is safe? >> {}", safe);

        }
        
        println!("<<>> Quantitiy of unsafe pairs >> {} ", unsafe_counts);
        println!("This combination is safe? >>>  {}", safe);
        if  safe || unsafe_counts == 1 { 
            safe_count += 1;
        }
        println!();

    }

    println!("Safe counts: {}", safe_count);
}
