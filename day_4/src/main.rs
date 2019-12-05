use std::io::Read;
use std::fs::OpenOptions;
use std::io::BufReader;

fn main() {
    let f = OpenOptions::new().read(true).open("input").unwrap();
    let mut buf = BufReader::new(f);
    let mut str = "".to_owned();
    buf.read_to_string(&mut str).unwrap();
    let input : Vec<&str> = str.split("-").collect();
    let start : Vec<u8> = input[0].bytes().collect();
    let start = find_first_number(start);
    let end : u64 = u64::from_str_radix("647015", 10).unwrap();
    dbg!(&start);
    dbg!((start..=end).filter(|x| has_adjacent_digits(x.to_string()) && digits_not_decreasing(x.to_string())).count());
}

fn find_first_number(mut input : Vec<u8>) -> u64 {
    let mut current = input[0];
    let mut modified = false;
    for i in 1..input.len() {
        if modified {
            input[i] = 48;
            continue
        }
        if input[i] < current {
            input[i] += 1;
            modified = true;
        }
        current = input[i];
    }
    std::string::String::from_utf8(input).unwrap().parse().unwrap()
}

fn has_adjacent_digits(input: String) -> bool {
    let mut input = input.chars();
    let mut digit = input.next().unwrap();
    let mut avoid_digit = Vec::new();
    while let Some(next_digit) = input.next() {
        if avoid_digit.contains(&next_digit) {
            continue
        }
        if next_digit == digit {
            let next_next_digit = input.next();
            if next_next_digit.is_none() {
                return true
            } else if next_digit == next_next_digit.unwrap() {
                avoid_digit.push(next_digit);
                digit = next_next_digit.unwrap();
                continue
            }
            return true
        } else {
            digit = next_digit;
        }
    }
    false

}

fn digits_not_decreasing(input: String) -> bool {
    let mut iter = input.chars();
    let mut current = iter.next().unwrap();
    while let Some(next) = iter.next() {
        if next < current {
            return false
        }
        current = next;
    }
    true
}
