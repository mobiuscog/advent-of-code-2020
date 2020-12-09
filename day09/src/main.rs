use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::VecDeque;

fn main() {
    let mut input: Vec<i64> = Vec::new();
    if let Ok(file) = File::open("input.dat") {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            input.push(line.unwrap().parse::<i64>().unwrap());
        }
    }

    let invalid_number = part1(&input);
    part2(&input, invalid_number);
}

fn part1(lines: &Vec<i64>) -> i64 {
    let mut buffer:VecDeque<i64> = VecDeque::new();
    let mut index = 0;
    for line in lines {
        let value = line;
        if index >= 25 {
            let mut valid = false;
            for entry in &buffer {
                if buffer.contains(&(value - entry)) {
                    valid = true;
                    break;
                }
            }
            if !valid {
                println!("Invalid number for part1: {}", value);
                return *value;
            }
            buffer.pop_back();
        }
        buffer.push_front(*value);
        index += 1;
    }
    return 0;
}

fn part2(lines: &Vec<i64>, invalid_number: i64) {
    for offset in 0..lines.len() {
        let mut total:i64 = 0;
        let mut start_offset:usize = offset;
        let mut smallest:i64 = 0;
        let mut largest:i64 = 0;
        loop {
            let value = lines[start_offset];
            total += value;
            if (value < smallest) || (smallest == 0) {
                smallest = value;
            }
            if value > largest {
                largest = value;
            }

            if total == invalid_number {
                println!("Sum of smallest and largest for part 2: {}", smallest + largest);
                return;
            }
            if total > invalid_number {
                break;
            }
            start_offset += 1;
        }
    }
}