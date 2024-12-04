use std::fs;

fn main() {
    let file = fs::read_to_string("src/data.txt").expect("There was an error reading the file\n");
    
    println!("File content: \n{}", file);  

    let mut matrix: Vec<Vec<char>> = Vec::new();

    for line in file.lines() {
        let line_chars: Vec<char> = line.trim().chars().collect();
        matrix.push(line_chars);
    }

    println!("Matrix size: {}", matrix.len());  

    let mut sum = 0;
    let mut sum2 = 0;

    for x in 0..matrix.len() {
        println!("Matrix[{}] length {}", x, matrix[x].len());
        for y in 0..matrix[x].len() {
                if x + 2 < matrix.len() && y + 1 < matrix[x].len() && x >= 1 && y >= 1 {

                    if matrix[x][y] == 'A'{
                    if matrix[x-1][y-1] == 'S' && matrix[x-1][y+1] == 'S' && matrix[x+1][y-1] == 'M' && matrix[x+1][y+1] == 'M' {sum2 +=1;}
                    if matrix[x-1][y-1] == 'M' && matrix[x-1][y+1] == 'M' && matrix[x+1][y-1] == 'S' && matrix[x+1][y+1] == 'S' {sum2 +=1;}
                    if matrix[x-1][y-1] == 'S' && matrix[x+1][y-1] == 'S' && matrix[x-1][y+1] == 'M' && matrix[x+1][y+1] == 'M' {sum2 +=1;}
                    if matrix[x-1][y-1] == 'M' && matrix[x+1][y-1] == 'M' && matrix[x-1][y+1] == 'S' && matrix[x+1][y+1] == 'S' {sum2 +=1;}

                }
            }
        }
    }

    for x in 0..matrix.len() {
        println!("This line has {} length: line {}", x, matrix[x].len());  
        
        for y in 0..matrix[x].len() {
            if matrix[x][y] == 'X' {
                if y + 3 < matrix[x].len()  {
                    if matrix[x][y + 1] == 'M' && matrix[x][y + 2] == 'A' && matrix[x][y + 3] == 'S' {
                        println!("Found horizontal at index {} {}", x, y);
                        sum += 1;
                    }
                }

                if x + 3 < matrix.len() -1 {
                    if matrix[x + 1][y] == 'M' && matrix[x + 2][y] == 'A' && matrix[x + 3][y] == 'S' {
                        println!("Found vertical at index {} {}", x, y);
                        sum += 1;
                    }

                    if y + 3 < matrix[x].len()  {
                        if matrix[x + 1][y + 1] == 'M' && matrix[x + 2][y + 2] == 'A' && matrix[x + 3][y + 3] == 'S' {
                            println!("Found down diagonal at index {} {}", x, y);
                            sum += 1;
                        }
                    }
                }

                if x >= 3 {
                    if matrix[x - 1][y] == 'M' && matrix[x - 2][y] == 'A' && matrix[x - 3][y] == 'S' {
                        println!("Found backwards vertical at index {} {}", x, y);
                        sum += 1;
                    }

                    if y + 3 < matrix[x].len()   {
                        if matrix[x - 1][y + 1] == 'M' && matrix[x - 2][y + 2] == 'A' && matrix[x - 3][y + 3] == 'S' {
                            println!("Found up diagonal at index {} {}", x, y);
                            sum += 1;
                        }
                    }
                }

                if y >= 3 {
                    if matrix[x][y - 1] == 'M' && matrix[x][y - 2] == 'A' && matrix[x][y - 3] == 'S' {
                        println!("Found backwards horizontal at index {} {}", x, y);
                        sum += 1;
                    }

                    if x + 3 < matrix.len() -1 {
                        if matrix[x + 1][y - 1] == 'M' && matrix[x + 2][y - 2] == 'A' && matrix[x + 3][y - 3] == 'S' {
                            println!("Found backwards down diagonal at index {} {}", x, y);
                            sum += 1;
                        }
                    }
                }

                if x >= 3 && y >= 3 {
                    if matrix[x - 1][y - 1] == 'M' && matrix[x - 2][y - 2] == 'A' && matrix[x - 3][y - 3] == 'S' {
                        println!("Found backwards up diagonal at index {} {}", x, y);
                        sum += 1;
                    }
                }
            }
        }
    }

    println!("Total valid XMAS = {}", sum);
    println!("Total valid X-MAS = {}", sum2);
}

