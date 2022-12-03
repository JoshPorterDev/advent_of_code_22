use std::fs;

// Rock     1
// Paper    2
// Scissors 3

// Lose     0
// Draw     3
// Win      6

// Lose     X
// Draw     Y
// Win      Z

fn main() {
    let file = fs::read_to_string("src/input").expect("Failed to read file");
    let lines = file.lines();

    let mut score: u32 = 0;

    for line in lines {
        score += calculate_score(&decide_based_on_desired_outcome(line));
    }

    println!("Final Score: {score}");
}

// Function calculates score for a round of rock paper scissors
// Input: takes string reference in the form of "A B"
// Return: a u32 type holding the score value
fn calculate_score(line: &str) -> u32 {
    let mut score: u32 = 0;

    // Get selections
    let player_selection: char = line.chars().nth(2).unwrap();
    let opponent_selection: char = line.chars().next().unwrap();

    match opponent_selection {
        'A' => println!("Opponent plays: rock"),
        'B' => println!("Opponent plays: paper"),
        'C' => println!("Opponent plays: scissors"),
        _ => (),
    }

    match player_selection {
        // Rock
        'A' => {
            score += 1;
            println!("Player plays: rock");
            match opponent_selection {
                'A' => score += 3,
                'B' => score += 0,
                'C' => score += 6,
                _ => score += 0,
            }
        }
        // Paper
        'B' => {
            score += 2;
            println!("Player plays: paper");
            match opponent_selection {
                'A' => score += 6,
                'B' => score += 3,
                'C' => score += 0,
                _ => score += 0,
            }
        }
        // Scissors
        'C' => {
            score += 3;
            println!("Player plays: scissors");
            match opponent_selection {
                'A' => score += 0,
                'B' => score += 6,
                'C' => score += 3,
                _ => score += 0,
            }
        }
        _ => println!("No move"),
    }

    println!("Score: {score}\n");
    score
}

// Function takes as input the elves strategy guide and returns a string of the decision in the
// proper format
// Input: the elf's strategy guide as a string in the format "A X"
// Returns: a string of the decision in format "A B"
fn decide_based_on_desired_outcome(line: &str) -> String {
    let (first, mut last) = line.split_at(1);

    // Get selections
    let player_selection: char = line.chars().nth(2).unwrap();
    let opponent_selection: char = line.chars().next().unwrap();

    match opponent_selection {
        'A' => {
            match player_selection {
                'X' => last = " C",
                'Y' => last = " A",
                'Z' => last = " B",
                _ => ()
            };
        }
        'B' => {
            match player_selection {
                'X' => last = " A",
                'Y' => last = " B",
                'Z' => last = " C",
                _ => ()
            };
        }
        'C' => {
            match player_selection {
                'X' => last = " B",
                'Y' => last = " C",
                'Z' => last = " A",
                _ => ()
            };
        }
        _ => (),
    }

    let decision: String = first.to_string() + last;
    println!("Decision string: {decision}");
    decision
}
