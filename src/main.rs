use std::fs::File;
use std::collections::HashMap;
use std::io::{self, BufRead};
use std::path::Path;


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}



fn main() {
    let mut sum: u32 = 0;

    let mut dup_number_mapping = HashMap::new();
    dup_number_mapping.insert("oneight".to_string(), "18".to_string());
    dup_number_mapping.insert("twone".to_string(), "21".to_string());
    dup_number_mapping.insert("threeight".to_string(), "38".to_string());
    dup_number_mapping.insert("fiveight".to_string(), "58".to_string());
    dup_number_mapping.insert("sevenine".to_string(), "79".to_string());
    dup_number_mapping.insert("eightwo".to_string(), "82".to_string());
    dup_number_mapping.insert("eighthree".to_string(), "83".to_string());
    dup_number_mapping.insert("nineight".to_string(), "98".to_string());

    let mut number_mapping = HashMap::new();
    number_mapping.insert("one".to_string(), "1".to_string());
    number_mapping.insert("two".to_string(), "2".to_string());
    number_mapping.insert("three".to_string(), "3".to_string());
    number_mapping.insert("four".to_string(), "4".to_string());
    number_mapping.insert("five".to_string(), "5".to_string());
    number_mapping.insert("six".to_string(), "6".to_string());
    number_mapping.insert("seven".to_string(), "7".to_string());
    number_mapping.insert("eight".to_string(), "8".to_string());
    number_mapping.insert("nine".to_string(), "9".to_string());

    if let Ok(lines) = read_lines("./day_1.txt") {
        for line in lines {
            if let Ok(line_result) = line {
                let mut replaced_str = line_result.clone();
                for (eng_num, digit_num) in &dup_number_mapping {
                    replaced_str = replaced_str.replace(eng_num, digit_num)
                }
                for (eng_num, digit_num) in &number_mapping {
                    replaced_str = replaced_str.replace(eng_num, digit_num)
                }
               let char_vec: Vec<u32> = replaced_str.chars()
                    .filter_map(|a| a.to_digit(10))
                    .collect();
                let num: u32 = char_vec.first().copied().unwrap() * 10 + char_vec.last().copied().unwrap();
                sum += num;
            }
        }
    }
    println!("Answer: {}", sum);
}
