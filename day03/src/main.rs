use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut input: Vec<String> = Vec::new();
    if let Ok(file) = File::open("input.dat") {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            input.push(line.unwrap());
        }
    }

    part1(&input);
    part2(&input);
}

fn part1(input: &Vec<String>) {
    let mut treecount:i32 = 0;
    let mut index:usize = 0;
    for line in input {
        let location = line.chars().nth(index).unwrap();
        if location == '#' {
            treecount += 1;
        }
        index += 3;
        index %= line.len();
    }
    println!("Number of Trees in Part 1: {}", treecount);
}

fn part2(input: &Vec<String>) {
    let mut treecount:i32 = count_trees(input, 1, 0);
    treecount *= count_trees(input, 3, 0);
    treecount *= count_trees(input, 5, 0);
    treecount *= count_trees(input, 7, 0);
    treecount *= count_trees(input, 1, 1);
    println!("Tree multiplier for Part 2: {}", treecount);
}

fn count_trees(input: &Vec<String>, right_jump: i8, line_skip: i8) -> i32 {
    let mut treecount:i32 = 0;
    let mut index:usize = 0;
    let mut skip_remaining = 0;
    for line in input {
        if skip_remaining > 0 {
            skip_remaining -= 1;
            continue;
        } else {
            skip_remaining = line_skip;
        }

        let location = line.chars().nth(index).unwrap();
        if location == '#' {
            treecount += 1;
        }
        index += right_jump as usize;
        index %= line.len();
    }
    return treecount;
}