use std::collections::HashSet;
use std::fs;

fn main() {
    let file = fs::read_to_string("src/input").expect("Failed to read file");
    let index = find_distinct(&file, 12);
    println!("Index: {index}");
}

fn find_distinct(file: &str, len: usize) -> usize {
    let mut distinct: String = String::new();
    let mut index: usize = 0;

    for (i, c) in file.trim().chars().enumerate() {
        if distinct.len() == len {
            let mhs: HashSet<char> = HashSet::from_iter(distinct.chars());
            if mhs.len() == len {
                index = i;
                break;
            }
            distinct.remove(0);
        }
        distinct.push(c);
    }

    index
}
