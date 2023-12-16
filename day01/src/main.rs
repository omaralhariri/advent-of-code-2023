use std::{collections::BTreeMap, fs};

fn main() {
    let numbers: BTreeMap<&str, usize> = BTreeMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let nums_vec = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let file = fs::read_to_string("input.txt").expect("Cannot read file");
    let lines: Vec<&str> = file.lines().collect();
    let mut lines_digits: BTreeMap<usize, BTreeMap<usize, usize>> = BTreeMap::new();

    for (index, line) in lines.iter().enumerate() {
        let mut all: BTreeMap<usize, usize> = BTreeMap::new();
        for word in nums_vec.iter() {
            let one: Vec<(usize, &str)> = line.match_indices(word).collect();
            if one.len() > 0 {
                for w in one.iter() {
                    all.insert(w.0, numbers.get(w.1).unwrap().to_owned());
                }
            }
        }

        if all.len() > 0 {
            lines_digits.insert(index, all);
        }

        for (i, c) in line.chars().into_iter().enumerate() {
            if c.is_digit(10) {
                lines_digits
                    .entry(index)
                    .and_modify(|e| {
                        e.entry(i)
                            .or_insert(c.to_digit(10).unwrap().try_into().unwrap());
                    })
                    .or_insert(BTreeMap::from([(
                        i,
                        c.to_digit(10).unwrap().try_into().unwrap(),
                    )]));
            }
        }
    }

    let mut digits: Vec<usize> = vec![];

    for mut map in lines_digits.values().cloned() {
        let first = map.pop_first().unwrap_or((10, 10)).1;
        let last = map.pop_last().unwrap_or((10, 10)).1;

        let mut two_digits: String = "".to_string();
        if first == 10 && last == 10 {
            continue;
        } else if last == 10 {
            let first_digit: String = first.to_string();
            two_digits.push_str(first_digit.as_str());
            two_digits.push_str(first_digit.as_str());
        } else if first == 10 {
            let last_digit: String = last.to_string();
            two_digits.push_str(last_digit.as_str());
            two_digits.push_str(last_digit.as_str());
        } else {
            let first_digit = first.to_string();
            let last_digit = last.to_string();
            two_digits.push_str(first_digit.as_str());
            two_digits.push_str(last_digit.as_str());
        }

        digits.push(two_digits.to_owned().parse::<usize>().unwrap());
    }

    let sum: usize = digits.iter().sum();
    println!("Sum {}", sum);
}
