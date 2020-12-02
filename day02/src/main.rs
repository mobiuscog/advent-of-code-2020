use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut data: HashSet<String> = HashSet::new();
    if let Ok(file) = File::open("input.dat") {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            data.insert(line.unwrap());
        }
    }
    part1(&data);
    part2(&data);
}

fn part1(data: &HashSet<String>) {
    let mut valid = 0;

    for line in data {
        let splitline:Vec<&str> = line.split(" ").collect();
        let rangesplit:Vec<&str> = splitline[0].split("-").collect();
        let rangemin:i32 = rangesplit[0].parse::<i32>().unwrap();
        let rangemax:i32 = rangesplit[1].parse::<i32>().unwrap();
        let required = &splitline[1][0..1];
        let password = splitline[2];
        let count = password.matches(required).count();
        if rangemax >= count as i32 && count as i32 >= rangemin {
            valid += 1;
        }
    }
    println!("Valid passwords for part 1: {}", valid);
}

fn part2(data: &HashSet<String>) {
    let mut valid = 0;

    for line in data {
        let splitline:Vec<&str> = line.split(" ").collect();
        let rangesplit:Vec<&str> = splitline[0].split("-").collect();
        let position1:i32 = rangesplit[0].parse::<i32>().unwrap();
        let position2:i32 = rangesplit[1].parse::<i32>().unwrap();
        let required = &splitline[1][0..1].chars().nth(0).unwrap();
        let password = splitline[2];
        let position1match = password.chars().nth(position1 as usize - 1).unwrap() == required.clone();
        let position2match = password.chars().nth(position2 as usize - 1).unwrap() == required.clone();
        if position1match != position2match {
            valid += 1;
        }
    }
    println!("Valid passwords for part 2: {}", valid);

}