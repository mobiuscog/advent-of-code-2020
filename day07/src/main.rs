use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut bags:HashMap<String, Bag> = HashMap::new();
    if let Ok(file) = File::open("input.dat") {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let rule = line.unwrap();
            let mut rule_split = rule.split(" bags contain ");
            let bag_name = String::from(rule_split.next().unwrap());
            let content_string =  rule_split.next().unwrap();
            let mut bag:Bag = Bag { allowed_child_bags: HashMap::new()};
            if !content_string.ends_with("no other bags.") {
                let contents = content_string.split_terminator('.').next().unwrap().split(", ");
                for child in contents {
                    let mut child_split = child.split(" ").collect::<Vec<&str>>();
                    child_split.remove(child_split.len() - 1);
                    let child_count = child_split.remove(0).parse::<i32>().unwrap();
                    let child_name = child_split.join(" ");
                    bag.allowed_child_bags.insert(String::from(&child_name), child_count);
                }
            }
            bags.insert(bag_name, bag);
        }
    }

    part1(&bags);
    part2(&bags);
}

struct Bag {
    allowed_child_bags: HashMap<String, i32>
}

fn part1(bags: &HashMap<String, Bag>) {
    let mut count = 0;
    let bag_list = bags.keys();
    for bag_name in bag_list {
        if check_bag_hold_shiny_gold(&bag_name, bags) {
            count += 1;
        }
    }
    println!("Bags that can carry 'shiny gold': {}", count);
}

fn check_bag_hold_shiny_gold(bag_name: &String, bags: &HashMap<String, Bag>) -> bool {
    if bags.get(bag_name).unwrap().allowed_child_bags.contains_key("shiny gold") {
        return true;
    }
    for bag in bags.get(bag_name).unwrap().allowed_child_bags.keys() {
        if check_bag_hold_shiny_gold(bag, bags) {
            return true;
        }
    }
    return false;
}

fn part2(bags: &HashMap<String, Bag>) {
    let count = count_contained_bags(&String::from("shiny gold"), bags);
    println!("Bags contained by 'shiny gold': {}", count - 1);
}

fn count_contained_bags(bag_name: &String, bags: &HashMap<String, Bag>) -> i32 {
    let bag = bags.get(bag_name).unwrap();
    let mut bag_capacity:i32 = 0;
    for (name, count) in &bag.allowed_child_bags {
        bag_capacity += count_contained_bags(name, bags) * count;
    }
    return bag_capacity + 1;
}