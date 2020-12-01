// Find the two entries that sum to 2020; what do you get if you multiply them together?

use std::fs;

// read in the file
// loop through each value
// check if two entries give 2020
// if they do multiply them
// else continue

fn main() -> std::io::Result<()> {
    let file = fs::read_to_string("data.txt")?;
    let mut data : Vec<i32> = Vec::new();

    for line in file.lines() {
        let parse_value: i32 = line.parse().unwrap();
        data.push(parse_value);
    }

    for i in data.iter() {
        for j in data.iter() {
            if j + i == 2020 {
                println!("{}", j * i);
                return Ok(());
            }
        }

    }

    Ok(())
}
