mod part1;
mod part2;

fn main() -> std::io::Result<()> {
    part1::solution()?;
    part2::solution()?;

    Ok(())
}
