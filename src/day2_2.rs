use anyhow::Result;
use std::io::BufRead;

fn main() -> Result<()> {
    let stdin = std::io::stdin();
    let mut sum = 0;
    for line in stdin.lock().lines() {

        let line = line?;
        let colon_split: Vec<&str> = line.split(":").collect();
        let game_string: Vec<&str> = colon_split[0].split(" ").collect();
        let game_id = game_string[1].parse::<u32>().unwrap();
        let games = colon_split[1].split(";");
        let mut max_red: u32 = 0;
        let mut max_green: u32 = 0;
        let mut max_blue: u32 = 0;

        for game in games {
            let g_str: Vec<&str> = game.split(",").collect();
            for dice_str in g_str {
                let s: Vec<&str> = dice_str.trim().split(" ").collect();
                let dice_count = s[0].trim().parse::<u32>().unwrap();
                let dice_color = s[1].trim();

                if dice_color == "green" && max_green < dice_count {
                    max_green = dice_count;
                } else if dice_color == "red" && max_red < dice_count {
                    max_red = dice_count;
                } else if dice_color == "blue" && max_blue < dice_count {
                    max_blue = dice_count;
                }
            }
        }

        let power = max_green * max_red * max_blue;
        sum += power;
        //println!("game {}", game_id);
        //println!("power {}", power);
    }
    println!("Answer: {}", sum);
    Ok(())
}
