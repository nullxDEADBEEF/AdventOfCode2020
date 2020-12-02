// read the data file
// split the string to gather password policy
// check if we have the character within the bounds
// if we do, add to a sum
// else continue

use std::fs;

fn main() -> std::io::Result<()> {
    let file = fs::read_to_string("data.txt")?;
    let mut valid_passwords = 0;

    for line in file.lines() {
        let line_data: Vec<_> = line.split(|c| c == '-' || c == ' ').collect();
        let lower_bound: i32 = line_data[0].parse().unwrap();
        let higher_bound: i32 = line_data[1].parse().unwrap();
        let policy_character: char = line_data[2].trim_matches(':').parse().unwrap();
        let password = line_data[3]; 

        let mut character_counter = 0;
        for character in password.chars() {
            if character == policy_character {
                character_counter += 1;
            }
        }
        if character_counter >= lower_bound && character_counter <= higher_bound {
            valid_passwords += 1;
        }

    }

    println!("valid passwords: {}", valid_passwords);

    Ok(())
}
