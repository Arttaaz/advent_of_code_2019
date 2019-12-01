use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    let input = OpenOptions::new().read(true).open("input").unwrap();
    let buf = BufReader::new(input);

    let result = buf.lines().fold(0, |sum, x| {
        sum + calculate_fuel(x.unwrap().parse().unwrap())
    });
    println!("result: {}", result);
}


fn calculate_fuel(module: u64) -> u64 {
    let fuel = module / 3;
    fuel - 2
}
