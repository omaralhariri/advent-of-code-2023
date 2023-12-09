use std::{fs, collections::BTreeMap};

fn main() {
    let file = fs::read_to_string("input.txt")
        .expect("Cannot read file");
    let lines: Vec<&str> = file.lines().collect();
    let mut lines_digits_idx: BTreeMap<usize, Vec<char>>  = BTreeMap::new();

    for (index, line) in lines.iter().enumerate() {
        let chars: Vec<char> = line.chars().filter(|x: &char| {
            return x.is_digit(10);
        }).collect();

        lines_digits_idx.entry(index).or_insert(chars);
    }

    let mut digits: Vec<usize> = vec![];
    
    for map in lines_digits_idx.iter() {
        let chars = map.1;
        let mut two_digits: String = "".to_string();
        if chars.len() == 0 {
            continue;
        } else if chars.len() == 1 {
            let first_digit: String = chars.first().unwrap().to_owned().to_string();
            two_digits.push_str(first_digit.as_str());
            two_digits.push_str(first_digit.as_str());
        } else {
            let first_digit = chars.first().unwrap().to_string();
            let last_digit = chars.last().unwrap().to_string();
            two_digits.push_str(first_digit.as_str());
            two_digits.push_str(last_digit.as_str());
        }
        
        digits.push(two_digits.to_owned().parse::<usize>().unwrap());
    }

    let sum: usize = digits.iter().sum();
    println!("Sum {}",sum); 
}

