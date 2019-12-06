
use std::collections::HashSet;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::Read;


fn main() {
    let f = OpenOptions::new().read(true).open("input").unwrap();
    let mut buf = BufReader::new(f);
    let mut str = "".to_owned();

    buf.read_to_string(&mut str).unwrap();
    let v : Vec::<(&str, &str)> = str.split("\n").filter_map(|x| {
        if !x.is_empty() {
            let mut s = x.split(")");
            Some((s.next().unwrap(), s.next().unwrap()))
        } else {
            None
        }
    }).collect();

    let mut tree : HashMap<&str, HashSet<&str>> = HashMap::new();
    for node in v {
        let entry = tree.entry(node.0).or_insert_with(HashSet::new);
        if !entry.contains(node.1) {
            entry.insert(node.1);
        }
    }
    let mut nodes : Vec<&&str> = tree.get("COM").expect("COM not found").iter().map(|x| x).collect();
    let mut depth = 1;
    let mut sum = 0;
    while !nodes.is_empty() {
        sum += nodes.len()*depth;
        nodes = nodes.iter().filter_map(|x| tree.get(*x)).flatten().collect();
        depth += 1;
    }

    println!("checksum : {}", sum);
}
