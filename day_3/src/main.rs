use std::fs;

fn main() {
    let file = fs::read_to_string("src/input").expect("Failed to read file");
    // Two copies of lines variable because I struggled with resolving the moved copy error
    // since lines() calls the into_iter() method which changes ownership
    let lines = file.lines();
    let lines2 = file.lines();

    let mut line_num: u32 = 0;
    let mut group_num: u32 = 0;
    let mut total_priority: u32 = 0;
    let mut group_total_priority: u32 = 0;
    let mut group_rucksacks: String = String::from("");

    // Find the priority for all rucksacks
    for line in lines {
        line_num += 1;
        total_priority += find_total_priority(line, line_num);
    }

    // Find the priority for all elf groups
    for line in lines2 {
        line_num += 1;
        // Every group has 3 elves
        if line_num % 3 == 0 {
            group_num += 1;
            group_rucksacks += &format!(" {line}");
            group_total_priority += find_priority(find_group_badge(&group_rucksacks, group_num));
            // Reset the group rucksack string
            group_rucksacks = "".to_string();
        } else {
            group_rucksacks += &format!(" {line}");
        }
    }

    println!("Total priority: {total_priority}");
    println!("Group total priority: {group_total_priority}");
}

// Function takes a char as input and returns its numeric priority
fn find_priority(item: char) -> u32 {
    // Vector to hold all possible types
    // will use their index to determine their priority
    let types: Vec<char> = Vec::from([
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J',
        'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ]);

    // Get item priority
    let item_priority = types.iter().position(|&i| i == item).unwrap() + 1;

    item_priority.try_into().unwrap()
}

// Function takes line as input and returns the group badge as a char
fn find_group_badge(line: &str, group_num: u32) -> char {
    let mut rucksacks = line.split_whitespace();
    let mut badge: char = '0';

    // Get all three rucksacks
    let first = rucksacks.next().unwrap();
    let second = rucksacks.next().unwrap();
    let third = rucksacks.next().unwrap();

    println!("Group: {group_num}");
    println!("----------------------");
    println!("Rucksack 1: {first}\nRucksack 2: {second}\nRucksack 3: {third}");

    // Find badge
    // Loop through first bag
    for item in first.chars() {
        // Check if item exists in both bags
        if second.contains(item) && third.contains(item) && item != badge {
            println!("Badge: {item}");
            badge = item;
        }
    }

    println!();
    badge
}

// Function takes a line as input and calculates and returns priority as a u32
fn find_total_priority(line: &str, line_num: u32) -> u32 {
    // Divide rucksack into two compartments
    let (first, second) = line.split_at(line.len() / 2);

    let mut total_priority: u32 = 0;

    // Vector to hold record of items in both compartments
    let mut in_both: Vec<char> = Vec::new();

    println!("Rucksack #{line_num}");
    println!("----------------------");
    println!("First compartment : {first}");
    println!("Second compartment: {second}");

    // Loop through first compartment
    for item in first.chars() {
        // Check if item is in second compartment
        // second condition because a compartment can have many items of the same type
        // it only matters if any type is present in both compartments
        if second.contains(item) & !in_both.contains(&item) {
            in_both.push(item);
            // Get item priotity
            total_priority += find_priority(item);
            println!("{item} is in second compartment");
            println!("{item} has priority of {}\n", find_priority(item));
        }
    }

    total_priority
}
