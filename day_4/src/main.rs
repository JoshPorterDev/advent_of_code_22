use std::fs;

fn main() {
    let file = fs::read_to_string("src/input").expect("Failed to read file");
    let count_any_overlap: u32 = find_any_overlap(&file);
    let count_total_containment: u32 = find_full_containment(&file);
    println!("Full containment count: {count_total_containment}");
    println!("Any overlap count: {count_any_overlap}");
}

fn find_any_overlap(file: &str) -> u32 {
    let lines = file.lines();
    let mut count: u32 = 0;

    for line in lines {
        // Get elves from pairing
        let (first_elf, second_elf) = line.split_once(',').unwrap();
        // Get elf start and end point and save to vector
        let first: Vec<u32> = first_elf
            .split_terminator('-')
            .map(|x| x.parse().unwrap())
            .collect();
        let second: Vec<u32> = second_elf
            .split_terminator('-')
            .map(|x| x.parse().unwrap())
            .collect();

        // Check for overlap
        if first[0] <= second[1] && second[0] <= first[1] {
            count += 1;
        }
    }
    count
}

fn find_full_containment(file: &str) -> u32 {
    let lines = file.lines();
    let mut count: u32 = 0;

    for line in lines {
        // Get elves from pairing
        let (first_elf, second_elf) = line.split_once(',').unwrap();
        // Get elf start and end point and save to vector
        let first: Vec<u32> = first_elf
            .split_terminator('-')
            .map(|x| x.parse().unwrap())
            .collect();
        let second: Vec<u32> = second_elf
            .split_terminator('-')
            .map(|x| x.parse().unwrap())
            .collect();

        // Check if second contains first
        if first[0] >= second[0] && first[1] <= second[1] {
            count += 1;
        }
        // Check if first contains second
        else if second[0] >= first[0] && second[1] <= first[1] {
            count += 1;
        }
    }
    count
}
