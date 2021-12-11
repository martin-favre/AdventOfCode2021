use std::fs::File;
use std::io;
use std::io::prelude::*;

fn first() {
    let my_file = File::open("data").unwrap();
    let reader = io::BufReader::new(my_file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let mut n_zeroes: Vec<i32> = Vec::new();

    for line in &lines {
        for i in 0..line.len() {
            if n_zeroes.len() <= i {
                n_zeroes.push(0);
            }
            if line.chars().nth(i).unwrap() == '0' {
                n_zeroes[i] += 1;
            }
        }
    }
    let line_len = lines.len() as i32;
    let mut gamma_string = "".to_owned();
    let mut epsilon_string = "".to_owned();
    for zero_count in n_zeroes.into_iter() {
        if zero_count > line_len / 2 {
            gamma_string.push('0');
            epsilon_string.push('1');
        } else {
            gamma_string.push('1');
            epsilon_string.push('0');
        }
    }
    let gamma_value = isize::from_str_radix(&gamma_string, 2).unwrap();
    let epsilon_value = isize::from_str_radix(&epsilon_string, 2).unwrap();
    println!("{}", gamma_value * epsilon_value);
}

fn get_most_common_value(lines: &Vec<String>, index: usize) -> char {
    let mut n_zeroes = 0;
    let mut n_ones = 0;
    for line in lines {
        if line.chars().nth(index).unwrap() == '0' {
            n_zeroes += 1
        } else {
            n_ones += 1
        }
    }
    return if n_zeroes > n_ones { '0' } else { '1' };
}

fn get_rating(mut lines: Vec<String>, most_common: bool) -> i32 {
    let mut index = 0;
    while lines.len() > 1 {
        let majorital_val = get_most_common_value(&lines, index);
        if most_common {
            lines.retain(|x| x.chars().nth(index).unwrap() == majorital_val);
        } else {
            lines.retain(|x| x.chars().nth(index).unwrap() != majorital_val);
        }
        index += 1;
        assert_ne!(index, lines.len());
    }
    return isize::from_str_radix(&lines[0], 2).unwrap() as i32;
}

fn second() {
    let my_file = File::open("data").unwrap();
    let reader = io::BufReader::new(my_file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let oxygen_rating = get_rating(lines.clone(), true);
    let c02_rating = get_rating(lines.clone(), false);
    println!("{}", oxygen_rating * c02_rating);
}

fn main() {
    first();
    second();
}
