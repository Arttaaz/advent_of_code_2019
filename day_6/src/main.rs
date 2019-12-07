
use std::iter::FromIterator;
use std::collections::HashSet;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::Read;
use rayon::prelude::*;

fn main() {
    let f = OpenOptions::new().read(true).open("input2").unwrap();
    let mut buf = BufReader::new(f);
    let mut str = "".to_owned();

    buf.read_to_string(&mut str).unwrap();
    let v : Vec::<(String, String)> = str.split("\n").filter_map(|x| {
        if !x.is_empty() {
            let mut s = x.split(")");
            Some((s.next().unwrap().to_string(), s.next().unwrap().to_string()))
        } else {
            None
        }
    }).collect();

    let mut tree : HashMap<String, HashSet<String>> = HashMap::new();
    for node in v {
        let entry = tree.entry(node.0.clone()).or_insert_with(HashSet::new);
        entry.insert(node.1.clone());
        let entry = tree.entry(node.1).or_insert_with(HashSet::new);
        entry.insert(node.0);
    }
    // let mut nodes : Vec<&String> = tree.get("COM").expect("COM not found").iter().map(|x| x).collect();
    // let mut depth = 1;
    // let mut sum = 0;
    // while !nodes.is_empty() {
    //     sum += nodes.len()*depth;
    //     nodes = nodes.par_iter().filter_map(|x| tree.get(*x)).flatten().collect();
    //     depth += 1;
    // }
    //
    // println!("checksum : {}", sum);

    //part2
    let mut nodes: HashSet<String> = tree.get(&"YOU".to_string()).unwrap().clone();
    let mut new_nodes = Vec::new();
    let mut depth = 1;
    while !nodes.is_empty() && !nodes.contains(&"SAN".to_owned()) {
        new_nodes = nodes.par_iter().map(|x| neighbors(&tree, x)).flatten().collect();
        let mut new_new_nodes = HashSet::new();
        for node in new_nodes {
            if !nodes.contains(&node) {
                new_new_nodes.insert(node);
            }
        }
        nodes = new_new_nodes;
        depth += 1;
        dbg!(&depth);
    }

    println!("distance: {}", depth-2);
}

fn neighbors<'a>(tree : &'a HashMap<String, HashSet<String>>, value: &String) -> HashSet<String> {
    match tree.get(value) {
        Some(s) => s.clone(),
        None => HashSet::new(),
    }
}
