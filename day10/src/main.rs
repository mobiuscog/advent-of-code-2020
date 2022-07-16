use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut input: Vec<i64> = Vec::new();
    let mut highest_value:i64 = 0;
    if let Ok(file) = File::open("input.dat") {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let value = line.unwrap().parse::<i64>().unwrap();
            if value > highest_value {
                highest_value = value;
            }
            input.push(value);
        }
    }

    let restricted_list = vec![1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19];//part1(&input, highest_value);
    part2(&restricted_list);
}

fn part1(input: &Vec<i64>, max_jolts: i64) -> Vec<i64>{
    let device_jolts = max_jolts + 3;
    let mut current_jolts:i64 = 0;
    let mut step_1_count = 0;
    let mut step_3_count = 0;
    let mut restricted_list:Vec<i64> = Vec::new();
    loop {
        if current_jolts >= max_jolts {
            break;
        }
        if input.contains(&(current_jolts + 1)) {
            current_jolts += 1;
            step_1_count += 1;

        } else if input.contains(&(current_jolts + 2)) {
            current_jolts += 2;
        } else if input.contains(&(current_jolts + 3)) {
            current_jolts += 3;
            step_3_count += 1;
        }
        restricted_list.push(current_jolts);
    }
    if device_jolts - current_jolts == 1 {
        step_1_count += 1;
    } else if device_jolts - current_jolts == 3 {
        step_3_count += 1;
    }
    println!("Number of adapters:{}, highest adapter jolts:{}", input.len(), max_jolts);
    println!("Device jolts:{}, Current jolts:{}, 1-jolt steps:{}, 3-jolt steps:{}", device_jolts, current_jolts, step_1_count, step_3_count);
    println!("Part 1 answer: {}", step_1_count * step_3_count);
    return restricted_list;
}

fn part2(input: &Vec<i64>) {
    let mut count:u64 = 1;

    let mut reverse:Vec<i64> = input.clone();
    reverse.reverse();

    for value in reverse {
        if value == *(input.get(0).unwrap()) {
            break;
        }
        let mut paths = 0;
        if !input.contains(&(value - 1)) {

            if input.contains(&(value - 3)) {
                paths += 1;
            }

            if input.contains(&(value - 2)) {
                paths += 1;
            }

        }
        print!("{}: {}, ", value, paths);
        count += paths;
    }
    println!("");
    // println!("{:?}", branch_list);
    // branch_list.reverse();
    // for value in branch_list {
    //     count *= value as u64;
    // }
    // let count = check_joltage(&input, 0, target);
    println!("Part 2 answer: {}", count);
}

fn check_joltage(adapters:&Vec<i64>, value: i64, target:i64) -> u32 {

    if value == target {
        return 1;
    }

    let mut count = 0;
    for test_increment in 1..=3 {
        if adapters.contains(&(value + test_increment)) {
            count += check_joltage(adapters, value + test_increment, target);
        }
    }
    return count;
}