use std::collections::HashSet;
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

    let mut groups:Vec<Vec<HashSet<char>>> = Vec::new();
    let mut group:Vec<HashSet<char>> = Vec::new();
    let mut field_count = 0;
    for line in &input {
        if line == "" {
            if field_count > 0 {
                groups.push(group);
                group = Vec::new();
            }
            field_count = 0;
            continue;
        }
        let mut answers:HashSet<char> = HashSet::new();
        for char in line.chars() {
            answers.insert(char);
            field_count += 1;
        }
        group.push(answers);
    }
    if field_count > 0 {
        groups.push(group);
    }

    part1(&groups); // 7128
    part2(&groups);
}

fn part1(groups: &Vec<Vec<HashSet<char>>>) {
    let mut sum:usize = 0;
    for group in groups {
        let mut group_answers:HashSet<char> = HashSet::new();
        for answers in group {
            group_answers.extend(answers);
        }
        sum += group_answers.len();
    }
    println!("Total of all group unique answers for Part 1: {}", sum);
}



fn part2(groups: &Vec<Vec<HashSet<char>>>) {
    let mut sum:usize = 0;
    for group in groups {
        let mut group_answers:HashSet<char> = HashSet::new();
        for answers in group {
            group_answers.extend(answers);
        }
        let mut group_common_answers:HashSet<char> = group_answers.clone();
        for answers in group {
            group_common_answers.retain(|&k| answers.contains(&k));
        }
        sum += group_common_answers.len();
    }
    println!("Total of all group common answers for Part 2: {}", sum);
}
