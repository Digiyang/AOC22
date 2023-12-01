use std::io::Write;
use std::{collections::BTreeMap, fs::File, io::Read};
fn main() -> Result<(), anyhow::Error> {
    // create a hash map where numbers in letters are the keys and the numbers in digits are the values
    let map = BTreeMap::from([
        ("zero", '0'),
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
        ("0", '0'),
        ("1", '1'),
        ("2", '2'),
        ("3", '3'),
        ("4", '4'),
        ("5", '5'),
        ("6", '6'),
        ("7", '7'),
        ("8", '8'),
        ("9", '9'),
    ]);

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

    let mut output_file = File::create("output.txt")?;

    let mut sum = 0;

    for line in output.lines() {
        let mut new_number = String::new();
        // Sort keys based on their starting index in the line
        let mut sorted_keys: Vec<&str> = map.keys().cloned().collect();
        sorted_keys.sort_by_key(|key| line.find(key).unwrap_or_else(|| usize::MAX));

        // Iterate through the sorted keys and collect all occurrences of the corresponding values
        let mut numbers: Vec<char> = sorted_keys
            .iter()
            .flat_map(|&key| {
                line.match_indices(key).map(|(_index, _)| {
                    if key.chars().all(|c| c.is_digit(10)) {
                        key.chars().next().unwrap()
                    } else {
                        map[key].to_string().chars().next().unwrap()
                    }
                })
            })
            .collect();

        if numbers.len() == 1 {
            numbers.push(numbers[0]);
            // concatenate the two numbers
            new_number = numbers.iter().collect();
            sum += new_number.parse::<i32>().unwrap();
        } else {
            // concatenate first and last element of the vector
            new_number = numbers[0].to_string() + &numbers[numbers.len() - 1].to_string();
            sum += new_number.parse::<i32>().unwrap();
        }
        writeln!(output_file, "{:?} | {}", numbers, new_number)?;
    }
    println!("{}", sum);
    Ok(())
}
