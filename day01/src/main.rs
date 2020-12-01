use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let sum: i32 = 2020;
    let mut report: HashSet<i32> = HashSet::new();
    if let Ok(file) = File::open("input.dat") {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            report.insert(line.unwrap().parse::<i32>().unwrap());
        }
    }

    // Part 1
    for value in &report {
        let other: i32 = sum - value;
        if report.contains(&other) {
            println!(
                "Part 1 Result: {0} + {1} = {2}, {0} * {1} = {3}",
                value,
                other,
                value + other,
                value * other
            );
            break;
        }
    }

    // Part 2
    'outer: for value in &report {
        let tempsum: i32 = sum - value;
        let mut tempreport: HashSet<i32> = report.clone();
        tempreport.remove(value);
        for tempvalue in &tempreport {
            let tempother: i32 = tempsum - tempvalue;
            if tempreport.contains(&tempother) {
                println!(
                    "Part 2 Result: {0} + {1} + {2} = {3}, {0} * {1} * {2} = {4}",
                    value,
                    tempvalue,
                    tempother,
                    value + tempvalue + tempother,
                    value * tempvalue * tempother
                );
                break 'outer;
            }
        }
    }
}
