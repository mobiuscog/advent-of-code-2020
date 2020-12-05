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

    let mut highest_id:u16 = 0;
    let mut seat_ids:Vec<u16> = Vec::new();
    for boarding_pass in input {
        let mut start:u8 = 0;
        let mut end:u8 = 127;
        let mut range:u8 = end - start;
        let mut criteria = boarding_pass.chars();
        for char in criteria.take(7) {
            range = range / 2;
            match char {
                'F' => end = start + range,
                'B' => start = end - range,
                _ => {}
            }
        }
        let seat_row = start;
        start = 0;
        end = 7;
        range = end - start;
        criteria = boarding_pass.chars();
        for char in criteria.skip(7) {
            range = range / 2;
            match char {
                'L' => end = start + range,
                'R' => start = end - range,
                _ => {}
            }
        }
        let seat_column = start;
        let seat_id:u16 = (seat_row as u16 * 8_u16) + seat_column as u16;
        seat_ids.push(seat_id);
        if seat_id > highest_id {
            highest_id = seat_id;
        }
    }
    println!("Highest Seat ID for part 1: {}", highest_id);

    seat_ids.sort();
    let mut previous = seat_ids[0] - 1;
    for seat_id in seat_ids {
        if previous != seat_id - 1 {
            println!("Missing seat ID for Part 2 = {}, current = {}, previous = {}", seat_id - 1, seat_id, previous);
        }
        previous = seat_id;
    }
}

