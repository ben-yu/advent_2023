
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
        let hex = parts.next().unwrap();
        let new_steps = i64::from_str_radix(&hex[2..7], 16).unwrap();
        let new_dir = match &hex[7..8] {
            "0" => "R",
            "1" => "D",
            "2" => "L",
            "3" => "U",
            _ => panic!("Invalid direction"),
        };

        match new_dir {
            "U" => {
                y += new_steps;
            }
            "D" => {
                y -= new_steps;
                sum += new_steps;
            }
            "L" => {
                // Remove area from previous 'right'
                sum -= (y) * (new_steps);
            }
            "R" => {
                 // assume always right first to get area from height
                sum += (y + 1) * (new_steps);
            }
            _ => panic!("wrong direction {}", new_dir),
        }
    }

    println!("Answer: {sum}");
    Ok(())
}
