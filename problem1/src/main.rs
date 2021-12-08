use std::fs::File;
use std::io;
use std::io::prelude::*;

fn first() {
    let my_file = File::open("data").expect("Couldn't find file");
    let reader = io::BufReader::new(my_file);

    let mut previous_value = -1;
    let mut increments = 0;
    for line in reader.lines() {
        let line = line.expect("Couldn't parse line");
        let line_val = line.parse::<i32>().unwrap();
        if previous_value != -1 {
            if previous_value < line_val {
                increments += 1;
            }
        }
        previous_value = line_val;
    }
    println!("{}", increments);
}

fn second() {
    let my_file = File::open("data").expect("Couldn't find file");
    let reader = io::BufReader::new(my_file);

    let mut previous_value1 = -1;
    let mut previous_value2 = -1;
    let mut previous_value3 = -1;
    let mut increments = 0;
    let mut previous_sum = 0;
    for line in reader.lines() {
        let line = line.expect("Couldn't parse line");
        let line_val = line.parse::<i32>().unwrap();

        if previous_value1 != -1 && previous_value2 != -1 && previous_value3 != -1 {
            let sum = previous_value1 + previous_value2 + previous_value3;
            if sum > previous_sum {
                increments += 1;
            }
            previous_sum = sum;
        }

        previous_value3 = previous_value2;
        previous_value2 = previous_value1;
        previous_value1 = line_val;

    }
    println!("{}", increments);

}

fn main() {
    first();
    second();
}
