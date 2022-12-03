use std::fs;

fn main() {
    let file = fs::read_to_string("src/input").expect("Failed to read file");
    let lines = file.lines();

    // Vector to store calorie counts of each elf
    let mut calorie_counts: Vec<u32> = Vec::new();

    // u32 for each elf's calorie count
    // initialize to zero
    let mut elf_calorie_count: u32 = 0;

    // Loop through each line in file
    for line in lines {
        if line.is_empty() {
            // Add current count to vector and reset the count
            calorie_counts.push(elf_calorie_count);
            elf_calorie_count = 0;
        } else {
            elf_calorie_count += line.parse::<u32>().unwrap();
        }
    }

    // Find the elf carrying the most calories
    calorie_counts.sort();
    println!("Elf with most calories has: {:?}\n", calorie_counts.last().unwrap());

    // Get top 3
    let mut top_three_count: u32 = 0;
    for i in 1..4 {
        let elf_count = calorie_counts.pop().unwrap();
        println!("Elf {} has {} calories", i, elf_count);
        top_three_count += elf_count;
    }
    println!("\nThe top three elves are carrying: {} calories", top_three_count);
}
