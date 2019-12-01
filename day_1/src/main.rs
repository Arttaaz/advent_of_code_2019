use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    let input = OpenOptions::new().read(true).open("input").unwrap();
    let buf = BufReader::new(input);

    let mut fuel : u64 = buf.lines().fold(0u64, |sum, x| {
        sum + calculate_fuel(x.unwrap().parse().unwrap())
    });
    
    println!("result: {}", fuel);
}


fn calculate_fuel(module: u64) -> u64 {
    let mut fuel = module / 3;
    fuel -= 2;
    let mut fuel2 = fuel;
    while fuel2 > 0 {
        let tmp : i64 = (fuel2 as i64/3) - 2;
        if tmp <= 0 {
            break;
        }
        fuel += tmp.clone() as u64;
        fuel2 = tmp as u64;
    }
    fuel
}
