use std::{fs::File, io::Read};

#[derive(Debug)]
struct Set {
    red: usize,
    green: usize,
    blue: usize,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Game {
    game_number: usize,
    number_of_sets: usize,
    sets: Vec<Set>,
    red_cubes: usize,
    green_cubes: usize,
    blue_cubes: usize,
}

fn main() -> Result<(), anyhow::Error> {
    // Input data
    let mut input = File::open("puzzle_input.txt")?;
    let mut output = String::new();
    input.read_to_string(&mut output)?;
    // Process the input data
    let games: Vec<Game> = output.lines().filter_map(|line| parse_game(line)).collect();
    let mut count = 0;

    for game in games {
        let mut all_sets_satisfy_conditions = true;

        for set in &game.sets {
            if set.red > 12 || set.green > 13 || set.blue > 14 {
                all_sets_satisfy_conditions = false;
                break; // No need to check other sets in this game if one set doesn't satisfy the conditions
            }
        }

        if all_sets_satisfy_conditions {
            count += game.game_number;
        }
    }

    println!("Total: {}", count);

    Ok(())
}

fn parse_game(line: &str) -> Option<Game> {
    let parts: Vec<&str> = line.split(':').collect();

    if parts.len() != 2 {
        return None;
    }

    let header_parts: Vec<&str> = parts[0].trim().split_whitespace().collect();

    if header_parts.len() != 2 {
        return None;
    }

    let game_number: usize = header_parts[1].parse().ok()?;
    let sets: Vec<&str> = parts[1].split(';').map(|s| s.trim()).collect();

    let mut red_cubes = 0;
    let mut green_cubes = 0;
    let mut blue_cubes = 0;
    let mut set_data = Vec::new();

    for set in &sets {
        let set_parts: Vec<&str> = set.split_whitespace().collect();

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for i in (0..set_parts.len()).step_by(2) {
            if i + 1 < set_parts.len() {
                let color = set_parts[i + 1].trim_end_matches(',');
                let count = set_parts[i].parse().unwrap_or_default();

                match color {
                    "red" => red = count,
                    "green" => green = count,
                    "blue" => blue = count,
                    _ => {}
                }
            }
        }

        red_cubes += red;
        green_cubes += green;
        blue_cubes += blue;

        set_data.push(Set { red, green, blue });
    }

    Some(Game {
        game_number,
        number_of_sets: sets.len(),
        sets: set_data,
        red_cubes,
        green_cubes,
        blue_cubes,
    })
}
