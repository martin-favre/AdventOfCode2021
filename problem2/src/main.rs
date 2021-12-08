use std::fs::File;
use std::io;
use std::io::prelude::*;

fn first() {
    let my_file = File::open("data").expect("Couldn't find file");
    let reader = io::BufReader::new(my_file);
    let mut depth = 0;
    let mut horizontal = 0;
    for line in reader.lines() {
        let line = line.expect("Couldn't parse line");
        let mut line_iter = line.split_whitespace();
        let direction = line_iter.next().unwrap();
        let amount = line_iter.next().unwrap().parse::<i32>().unwrap();
        if direction == "forward" {
            horizontal += amount;
        } else if direction == "up" {
            depth -= amount;
        } else if direction == "down" {
            depth += amount;
        }
    }
    println!("Depth: {}", depth);
    println!("Horizontal: {}", horizontal);
    println!("Result: {}", depth * horizontal);
}

fn second() {
    let my_file = File::open("data").expect("Couldn't find file");
    let reader = io::BufReader::new(my_file);
    let mut aim = 0;
    let mut horizontal = 0;
    let mut depth = 0;
    for line in reader.lines() {
        let line = line.expect("Couldn't parse line");
        let mut line_iter = line.split_whitespace();
        let direction = line_iter.next().unwrap();
        let amount = line_iter.next().unwrap().parse::<i32>().unwrap();
        if direction == "forward" {
            horizontal += amount;
            depth += aim*amount;
        } else if direction == "up" {
            aim -= amount;
        } else if direction == "down" {
            aim += amount;
        }
    }
    println!("Depth: {}", depth);
    println!("Horizontal: {}", horizontal);
    println!("Result: {}", depth * horizontal);
}

fn main() {
    first();
    second();

}
