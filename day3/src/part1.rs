use std::fs;

pub fn solution() -> std::io::Result<()> {
    let file_content = fs::read_to_string("data.txt")?;
    let character_list: Vec<Vec<char>> = file_content.trim().lines().map(|line| line.chars().collect()).collect();

    let mut trees_encountered = 0;
    const MOVE_RIGHT: usize = 3;
    const MOVE_DOWN: usize = 1;

    let mut x = MOVE_RIGHT;
    let mut y = MOVE_DOWN;

    loop {
        let line = character_list.get(y).unwrap();
        if line[x] == '#' {
            trees_encountered += 1;
        }
        
        x = (x + MOVE_RIGHT) % line.len();
        y += MOVE_DOWN;

        if y >= character_list.len() {
            break
        }
    }

    println!("Trees encountered: {}", trees_encountered);

    Ok(())
}