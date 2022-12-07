use std::{cmp, fs};

fn main() {
    let file = fs::read_to_string("src/input").expect("Failed to read file");
    part_one(&file);
    part_two(&file);
}

fn part_two(file: &str) {
    // Separate string into starting stacks and instructions sections
    let (starting_stacks, instructions) = file.split_at(file.find("move").unwrap() - 1);

    // Find the number of stacks dynamically
    let starting_stacks_len = starting_stacks.split_once('\n').unwrap().0.len();
    let num_stacks: u32 = ((starting_stacks_len - (starting_stacks_len / 4)) / 3)
        .try_into()
        .unwrap();

    // Remove leading and trailing whitespace and replace all remaining whitespace with .
    // Makes it easier to count
    let starting_stacks_trim = starting_stacks
        .split_once(" 1")
        .unwrap()
        .0
        .replace(' ', ".");

    // Represent stacks as vector of strings
    let mut stacks: Vec<String> = vec!["".to_string(); num_stacks.try_into().unwrap()];

    // Build initial stacks
    for line in starting_stacks_trim.lines() {
        for (i, c) in line.char_indices() {
            // Get items stack
            let item_stack = (i / 4) + 1;
            let stack_str = stacks.get_mut(item_stack - 1).unwrap();
            if c != '.' && c != '[' && c != ']' {
                stack_str.push(c);
            }
        }
    }

    // Reverse strings
    for s in stacks.iter_mut() {
        *s = s.chars().rev().collect::<String>();
    }

    // Split instruction string into words
    for instruction in instructions.trim().lines() {
        // Move amount: commands[1]
        // Move from:   commands[3]
        // Move to:     commands[5]
        let commands: Vec<&str> = instruction.split(' ').collect();
        let amt: usize = commands[1].parse().unwrap();
        let from = commands[3].parse::<usize>().unwrap();
        let to = commands[5].parse::<usize>().unwrap();

        // Find lesser of two indexes to split vector
        // Have to do this because trying to get a mutable slice by index returns the mutable
        // reference to the whole vector for some reason.
        let split_point = cmp::min(from, to);
        let (left, right) = stacks.split_at_mut(split_point);

        // Move crates
        if from > to {
            let stack = right.get_mut((from - to) - 1).unwrap();
            let mut moved: String = "".to_string();
            let mut tmp = "".to_string();
            for _i in 0..amt {
                tmp.push(stack.pop().unwrap());
            }
            moved.push_str(&tmp.chars().rev().collect::<String>());
            left.get_mut(left.len() - 1).unwrap().push_str(&moved);
        } else if from < to {
            let stack = left.get_mut(left.len() - 1).unwrap();
            let mut moved: String = "".to_string();
            let mut tmp = "".to_string();

            for _i in 0..amt {
                tmp.push(stack.pop().unwrap());
            }
            moved.push_str(&tmp.chars().rev().collect::<String>());
            right
                .get_mut((to - from) - 1)
                .unwrap()
                .push_str(&moved);
        }
    }
    let mut top_of_stacks: String = String::new();
    for stack in &stacks {
        top_of_stacks.push(stack.chars().last().unwrap());
    }
    println!("Part Two: {top_of_stacks}");
}

fn part_one(file: &str) {
    // Separate string into starting stacks and instructions sections
    let (starting_stacks, instructions) = file.split_at(file.find("move").unwrap() - 1);

    let starting_stacks_len = starting_stacks.split_once('\n').unwrap().0.len();
    let num_stacks: u32 = ((starting_stacks_len - (starting_stacks_len / 4)) / 3)
        .try_into()
        .unwrap();

    let starting_stacks_trim = starting_stacks
        .split_once(" 1")
        .unwrap()
        .0
        .replace(' ', ".");

    // Represent stacks as vector of strings
    let mut stacks: Vec<String> = vec!["".to_string(); num_stacks.try_into().unwrap()];

    for line in starting_stacks_trim.lines() {
        for (i, c) in line.char_indices() {
            // Get items stack
            let item_stack = (i / 4) + 1;
            let stack_str = stacks.get_mut(item_stack - 1).unwrap();
            if c != '.' && c != '[' && c != ']' {
                stack_str.push(c);
            }
        }
    }

    // Reverse strings
    for s in stacks.iter_mut() {
        *s = s.chars().rev().collect::<String>();
    }

    // Split instruction string into words
    for instruction in instructions.trim().lines() {
        // Move amount: commands[1]
        // Move from:   commands[3]
        // Move to:     commands[5]
        let commands: Vec<&str> = instruction.split(' ').collect();

        // Move crates
        for _i in 0..commands[1].parse().unwrap() {
            let from = stacks
                .get_mut(commands[3].parse::<usize>().unwrap() - 1)
                .unwrap();
            let item = from.pop().unwrap();
            stacks
                .get_mut(commands[5].parse::<usize>().unwrap() - 1)
                .unwrap()
                .push(item);
        }
    }
    let mut top_of_stacks: String = String::new();
    for stack in &stacks {
        top_of_stacks.push(stack.chars().last().unwrap());
    }
    println!("Part One: {top_of_stacks}");
}
