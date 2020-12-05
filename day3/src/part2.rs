use std::fs;

fn get_trees_on_slope(move_right_amount: usize, move_down_amount: usize) -> u32 {
    let file_content = fs::read_to_string("data.txt").unwrap_or_default();
    let character_list: Vec<Vec<char>> = file_content.trim().lines().map(|line| line.chars().collect()).collect();

    let mut trees_encountered = 0;

    let mut x = move_right_amount;
    let mut y = move_down_amount;

    loop {
        let line = character_list.get(y).unwrap();
        if line[x] == '#' {
            trees_encountered += 1;
        }

        x = (x + move_right_amount) % line.len();
        y += move_down_amount;

        if y >= character_list.len() {
            break;
        }
    }

    trees_encountered
    
}

pub fn solution() -> std::io::Result<()> {
    let one_right_one_down = get_trees_on_slope(1, 1);
    let three_right_one_down = get_trees_on_slope(3, 1);
    let five_right_one_down = get_trees_on_slope(5, 1);
    let seven_right_one_down = get_trees_on_slope(7, 1);
    let one_right_two_down = get_trees_on_slope(1, 2);
    
    let result = one_right_one_down * three_right_one_down * five_right_one_down *
                 seven_right_one_down * one_right_two_down;

    println!("part2: {}", result);

    Ok(())
}