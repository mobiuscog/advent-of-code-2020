use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use regex::Regex;

fn main() {
    let mut input: Vec<String> = Vec::new();
    if let Ok(file) = File::open("input.dat") {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            input.push(line.unwrap());
        }
    }

    let mut records:Vec<HashMap<&str, &str>> = Vec::new();
    let mut record:HashMap<&str, &str> = HashMap::new();
    let mut field_count = 0;
    for line in &input {
        if line == "" {
            if field_count > 0 {
                records.push(record);
                record = HashMap::new();
            }
            field_count = 0;
            continue;
        }
        for field in line.split(" ") {
            let mut fields = field.split(":");
            record.insert(fields.next().unwrap(), fields.next().unwrap());
            field_count += 1;
        }
    }
    if field_count > 0 {
        records.push(record);
    }

    part1(&records);
    part2(&records);
}

fn part1(records: &Vec<HashMap<&str, &str>>) {
    let mut valid_count:i32 = 0;
    for record in records {
        if record.len() == 8 || (record.len() == 7 && !record.contains_key("cid")) {
            valid_count += 1;
        }
    }
    println!("Valid record count in Part 1: {}", valid_count);
}



fn part2(records: &Vec<HashMap<&str, &str>>) {
    let mut valid_count:i32 = 0;
    let hcl_regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    let pid_regex = Regex::new(r"^[0-9]{9}$").unwrap();
    'record: for record in records {
        if record.len() == 8 || (record.len() == 7 && !record.contains_key("cid")) {
            for field in record {
                match field {
                    (&"byr", value) => {
                        // byr (Birth Year) - four digits; at least 1920 and at most 2002.
                        match value.parse::<i32>() {
                            Ok(year) => {
                                if year < 1920 || year > 2002 {
                                    continue 'record;
                                }
                            }
                            _ => continue 'record
                        }
                    },
                    (&"iyr", value) => {
                        // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
                        match value.parse::<i32>() {
                            Ok(year) => {
                                if year < 2010 || year > 2020 {
                                    continue 'record;
                                }
                            }
                            _ => continue 'record
                        }
                    },
                    (&"eyr", value) => {
                        // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
                        match value.parse::<i32>() {
                            Ok(year) => {
                                if year < 2020 || year > 2030 {
                                    continue 'record;
                                }
                            }
                            _ => continue 'record
                        }
                    },
                    (&"hgt", value) => {
                        // hgt (Height) - a number followed by either cm or in:
                        if value.ends_with("cm") {
                            // If cm, the number must be at least 150 and at most 193.
                            match value.chars().rev().skip(2).collect::<String>().chars().rev().collect::<String>().parse::<i32>() {
                                Ok(height) => {
                                    if height < 150 || height > 193 {
                                        continue 'record;
                                    }
                                }
                                _ => continue 'record
                            }
                        } else if value.ends_with("in") {
                            // If in, the number must be at least 59 and at most 76.
                            match value.chars().rev().skip(2).collect::<String>().chars().rev().collect::<String>().parse::<i32>() {
                                Ok(height) => {
                                    if height < 59 || height > 76 {
                                        continue 'record;
                                    }
                                }
                                _ => continue 'record
                            }
                        } else {
                            continue 'record;
                        }
                    },
                    (&"hcl", value) => {
                        // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
                        if !hcl_regex.is_match(value) {
                            continue 'record;
                        }
                    },
                    (&"ecl", value) => {
                        // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
                        let valid = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
                        if !valid.contains(value) {
                            continue 'record;
                        }

                    },
                    (&"pid", value) => {
                        // pid (Passport ID) - a nine-digit number, including leading zeroes.
                        if !pid_regex.is_match(value) {
                            continue 'record;
                        }
                    },
                    _ => {
                        // cid (Country ID) - ignored, missing or not.
                    }
                }
            }
            valid_count += 1;
        }
    }
    println!("Valid record count in Part 2: {}", valid_count);

}
