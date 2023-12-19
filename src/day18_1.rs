use anyhow::Result;
use std::io::BufRead;

fn main() -> Result<()> {
    let stdin = std::io::stdin();

    let mut y = 1000000; // really large height
    let mut sum = 1;
    for line in stdin.lock().lines() {
        let line = line?;
        let mut parts = line.split_whitespace();
        let dir = parts.next().unwrap();
        let steps = parts.next().unwrap().parse::<i64>().unwrap();

        match dir {
            "U" => {
                y += steps;
            }
            "D" => {
                y -= steps;
                sum += steps;
            }
            "L" => {
                // Remove area from previous 'right'
                sum -= (y) * (steps);
            }
            "R" => {
                 // assume always right first to get area from height
                sum += (y + 1) * (steps);
            }
            _ => panic!("wrong direction {}", dir),
        }
    }

    println!("Answer: {sum}");
    Ok(())
}
