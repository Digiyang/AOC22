use std::{fs::File, io::Read};

fn main() -> Result<(), anyhow::Error> {
    // open file
    let mut input_file = match File::open("puzzle-input.txt") {
        Ok(file) => file,
        Err(error) => {
            println!("Error opening file: {}", error);
            return Err(error.into());
        }
    };
    // read file contents into a string
    let mut output = String::new();
    input_file.read_to_string(&mut output)?;

    let mut sum = 0;

    // iterate through each line of the file then iterate through each character of the line
    for line in output.lines() {
        let mut numbers: Vec<char> = Vec::new();
        // iterate through each character of the line, if the character is a number save it to the vector
        for c in line.chars() {
            if c.is_digit(10) {
                numbers.push(c);
            }
        }
        // if the vector has only one number, duplicate it, then concatenate the two numbers
        if numbers.len() == 1 {
            numbers.push(numbers[0]);
            // concatenate the two numbers
            let new_number: String = numbers.iter().collect();
            sum += new_number.parse::<i32>().unwrap();
        } else {
            // concatenate first and last element of the vector
            let new_number = numbers[0].to_string() + &numbers[numbers.len() - 1].to_string();
            sum += new_number.parse::<i32>().unwrap();
        }
    }
    println!("{}", sum);
    Ok(())
}
